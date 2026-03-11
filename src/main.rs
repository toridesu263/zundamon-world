use std::fs;

fn main() {
    let entries = fs::read_dir("tts_queue_Rust");
    match entries {
        Ok(paths) => {
            let mut files: Vec<_> = paths
            .filter_map(|e| {
                let path = e.unwrap().path();
                if path.extension().and_then(|e| e.to_str()) == Some("wav") {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

            files.sort();

            for f in &files {
                println!("{:?}", f);
            }
        }
        Err(e) => println!("エラー: {}", e),
    }
}