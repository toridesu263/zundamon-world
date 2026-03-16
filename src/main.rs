use std::fs;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use cpal::traits::{DeviceTrait, HostTrait};

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

fn main() {
    let host = cpal::default_host();
    let device = host
        .output_devices()
        .unwrap()
        .find(|d| {
            d.name()
                .map(|n| n.contains("CABLE Input"))
                .unwrap_or(false)
        })
        .expect("VB-Cableが見つかりません");

    let (_stream, stream_handle) = OutputStream::try_from_device(&device).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    loop {
        let mut files: Vec<_> = fs::read_dir(r"C:\Users\とりです\Desktop\aboutStreamerbot\tts_queue_player\tts_queue_Rust")
        .unwrap()
        .filter_map(|e| {
            let path = e.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) == Some("wav") && is_file_ready(&path) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    files.sort();

    for f in &files {
        let file = File::open(f).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        sink.append(source);
        sink.sleep_until_end();
        fs::remove_file(f).unwrap();
        println!("再生完了: {:?}", f);
    }

    thread::sleep(Duration::from_millis(500));
    }
}
