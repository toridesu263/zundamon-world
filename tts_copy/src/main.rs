use arboard::Clipboard;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::window::{Window, WindowId};

const SPEAKER_ZUNDAMON: u32 = 3;
const SPEAKER_MEIMEIHIMARI: u32 = 14;

#[derive(Deserialize)]
struct Config {
    queue_dir: String,
}

fn load_config() -> Config {
    let text = fs::read_to_string("config.toml")
        .expect("config.tomlが見つかりません");
    toml::from_str(&text).expect("config.tomlの形式が正しくありません")
}

fn unix_time_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn log(msg: &str) {
    println!("[{}] {}", unix_time_ms(), msg);
}

fn save_to_queue(speaker_id: u32, queue_dir: &str) {
    let mut clipboard = match Clipboard::new() {
        Ok(c) => c,
        Err(e) => {
            log(&format!("クリップボード取得失敗: {:?}", e));
            return;
        }
    };
    let text = match clipboard.get_text() {
        Ok(t) => t,
        Err(e) => {
            log(&format!("クリップボード読み取り失敗: {:?}", e));
            return;
        }
    };
    if text.trim().is_empty() {
        log("クリップボードが空です");
        return;
    }

    /* 
    //  変更前
    let filename = format!("claude_{}_{}.text", unix_time_ms(), speaker_id);
    let filepath = Path::new(QUEUE_DIR).join(&filename);
    match fs::write(&filepath, &text) {
        Ok(_) => log(&format!("保存成功: {}", filename)),
        Err(e) => log(&format!("ファイル書き込み失敗: {:?}", e)),
    }*/
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let filename = format!("claude_{}_{}_{}.txt", unix_time_ms(), i, speaker_id);
        let filepath = Path::new(queue_dir).join(&filename);
        match fs::write(&filepath, line) {
            Ok(_) => log(&format!("保存成功: {}", filename)),
            Err(e) => log(&format!("ファイル書き込み失敗: {:?}", e)),
        }
    }
}

struct App {
    window: Option<Window>,
    modifiers: ModifiersState,
    last_event: String,
    queue_dir: String,
}

impl App {
    fn new(queue_dir: String) -> Self {
        App {
            window: None,
            modifiers: ModifiersState::empty(),
            last_event: "待機中".to_string(),
            queue_dir,
        }
    }

    fn update_title(&self) {
        if let Some(window) = &self.window {
            let msg = self.last_event.clone();
            window.set_title(&format!("ずんだもんわーるど - {}", msg));
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("ずんだもんわーるど - 待機中")
                    .with_inner_size(winit::dpi::LogicalSize::new(400.0, 100.0))
                    .with_resizable(false),
            )
            .unwrap();
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::ModifiersChanged(mods) => {
                self.modifiers = mods.state();
            }
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    let ctrl = self.modifiers.control_key();
                    let shift = self.modifiers.shift_key();

                    match key_event.physical_key {
                        PhysicalKey::Code(KeyCode::F9) if ctrl && shift => {
                            log("Ctrl+Shift+F9 検知 -> ずんだもん");
                            self.last_event = "Ctrl+Shift+F9 -> ずんだもん".to_string();
                            self.update_title();
                            save_to_queue(SPEAKER_ZUNDAMON, &self.queue_dir);
                        }
                        PhysicalKey::Code(KeyCode::F10) if ctrl && shift => {
                            log("Ctrl+Shift+F10 検知 -> めいめいひまり");
                            self.last_event = "Ctrl+Shift+F10 -> めいめいひまり".to_string();
                            self.update_title();
                            save_to_queue(SPEAKER_MEIMEIHIMARI, &self.queue_dir);
                        }
                        _ => {}
                    }                    
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let config = load_config();
    let queue_dir = config.queue_dir;
    if let Err(e) = fs::create_dir_all(&queue_dir) {
        eprintln!("キューフォルダ作成失敗: {:?}", e);
        return;
    }

    log("起動しました");
    log("ウィンドウにフォーカスを当ててからショートカットを教えてください");
    log("Ctrl+Shift+F9 -> ずんだもん");
    log("Ctrl+Shift+F10 -> めいめいひまり");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut App::new(queue_dir)).unwrap();
}