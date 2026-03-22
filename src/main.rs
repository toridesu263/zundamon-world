use std::fs;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use cpal::traits::{DeviceTrait, HostTrait};

const QUEUE_DIR: &str = r"C:\Users\とりです\Desktop\aboutStreamerbot\tts_queue_player\tts_queue_Rust";
const VOICEVOX_URL: &str = "http://localhost:50021";

fn is_file_ready(path: &std::path::Path) -> bool {
    let mut prev_size: u64 = 0;
    for _ in 0..5 {
        match std::fs::metadata(path) {
            Ok(meta) => {
                let size = meta.len();
                if size == prev_size && size > 0 {
                    return true;
                }
                prev_size = size;
            }
            Err(_) => return false,
        }
        thread::sleep(Duration::from_millis(100));
    }
    false
}

//ファイル名からspeaker_idを取り出す
// 例: claude_1706000001234_3.txt → 3
fn parse_speaker_id(path: &std::path::Path) -> Option<u32> {
    let stem = path.file_stem()?.to_str()?; // 拡張子なしのファイル名
    let parts: Vec<&str> = stem.split('_').collect();
    parts.last()?.parse().ok()
}

fn synthesize(text: &str, speaker_id: u32) -> Option<Vec<u8>> {
    let client = reqwest::blocking::Client::new();

    // audio_queryでクエリ作成
    let query_url = format!("{}/audio_query?text={}&speaker={}",
        VOICEVOX_URL,
        urlencoding::encode(text),
        speaker_id
    );
    let query = match client.post(&query_url).send() {
        Ok(r) => match r.text() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("audio_queryレスポンス読み取り失敗: {:?}", e);
                return None;
            }
        },
        Err(e) => {
            eprintln!("audio_query失敗: {:?}", e);
            return None;
        }
    };
    
    // synthesisでwav合成
    let synthesis_url = format!("{}/synthesis?speaker={}", VOICEVOX_URL, speaker_id);
    let wav = match client
        .post(&synthesis_url)
        .header("Content-Type", "application/json")
        .body(query)
        .send()
    {
        Ok(r) => match r.bytes() {
            Ok(b) => b.to_vec(),
            Err(e) => {
                eprintln!("synthesisレスポンス読み取り失敗: {:?}", e);
                return None;
            }
        },
        Err(e) => {
            eprintln!("synthesis失敗: {:?}", e);
            return None;
        }
    };

    Some(wav)
}

fn main() {
    let host = cpal::default_host();
    let device = host
        .output_devices()
        .unwrap()
        .find(|d| {
            d.name()
                .map(|n| n.containes("CABLE Input"))
                .unwrap_or(false)
        })
        .expect("VB-Cableが見つかりません");

    let (_stream, stream_handle) = OutputStream::try_from_device(&device).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    println!("監視開始: {}", QUEUE_DIR);

    loop {
        let mut files: Vec<_> = fs::read_dir(QUEUE_DIR)
            .unwrap()
            .filter_map(|e| {
                let path = e.unwrap().path();
                if path.extension().and_then(|e| e.to_str()) == Some("txt") && is_file_ready(&path) {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();
        
        files.sort();

        for f in &files {
            //speaker_id取得
            let speaker_id = match parse_speaker_id(f) {
                Some(id) => id,
                None => {
                    eprintln!("speaker_id取得失敗: {:?}", f);
                    fs::remove_file(f).unwrap();
                    continue;
                }
            };

            // テキスト読み込み
            let text = match fs::read_to_string(f) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("テキスト読み込み失敗: {:?}", e);
                    continue;
                }
            };

            println!("合成中: {:?} (speaker_id: {})", f, speaker_id);

            // VOICEVOX合成
            let wav = match synthesize(&text, speaker_id) {
                Some(w) => w,
                None => {
                    eprintln!("合成失敗、スキップ: {:?}", f);
                    fs::remove_file(f).unwrap();
                    continue;
                }
            };

            // 一時wavファイルに書き出して再生
            let wav_path = f.with_extension("wav");
            fs::write(&wav_path, &wav).unwrap();

            let file = File::open(&wav_path).unwrap();
            let source = Decoder::new(BufReader::new(file)).unwrap();
            sink.append(source);
            sink.sleep_until_end();

            //後片付け
            fs::remove_file(&wav_path).unwrap();
            fs::remove_file(f).unwrap();
            println!("再生完了: {:?}", f);
        }

        thread::sleep(Duration::from_millis(500));
    }
}