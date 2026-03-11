use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = File::open("tts_queue_Rust/test.wav").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    sink.sleep_until_end();

    println!("再生完了！");
}

/*
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = File::open("tts_queue_Rust/こんにちは.wav").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    sink.sleep_until_end();

    println!("再生完了");
}
*/