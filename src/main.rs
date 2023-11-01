extern crate rand;

use std::error::Error;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity::Activity};
use discord_rich_presence::activity::Assets;
use rand::Rng;
use winapi::um::winuser;

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = DiscordIpcClient::new("1157397554670096385")?;
    client.connect()?;

    let (tx, rx): (mpsc::Sender<bool>, mpsc::Receiver<bool>) = mpsc::channel();

    let initial_activity = Activity::new()
        .state("None")
        .details("No active window");
    client.set_activity(initial_activity)?;

    thread::spawn(move || {
        let (mut old_window_title, mut old_window_class): (String, String) = get_active_window();
        loop {
            let (current_window_title, current_window_class): (String, String) = get_active_window();

            if old_window_title == current_window_title && old_window_class == current_window_class { continue; }

            old_window_title = current_window_title.clone();
            old_window_class = current_window_class.clone();

            let large_image_id = rand::thread_rng().gen_range(1..=21).to_string();

            // DiscordのRich Presence情報を設定
            let activity = Activity::new()
                .state(&current_window_title)
                .details(&current_window_class)
                .assets(Assets::new().large_image(large_image_id.as_str()).large_text("Astolfo!"));
            client.set_activity(activity).unwrap(); // エラーハンドリングが必要

            thread::sleep(Duration::from_secs(1));
        }
    });

    ctrlc::set_handler(move || {
        tx.send(true).unwrap();
    }).expect("Error setting Ctrl-C handler");

    rx.recv().unwrap();

    Ok(())
}

fn get_active_window() -> (String, String) {
    unsafe {
        let mut window_title: Vec<u16> = vec![0; 512];
        let mut window_class: Vec<u16> = vec![0; 512];
        let active_window = winuser::GetForegroundWindow();

        let title_length = winuser::GetWindowTextW(active_window, window_title.as_mut_ptr(), 512);
        let class_length = winuser::RealGetWindowClassW(active_window, window_class.as_mut_ptr(), 512);

        if title_length > 0 {
            (String::from_utf16_lossy(&window_title[..title_length as usize]), String::from_utf16_lossy(&window_class[..class_length as usize]))

        } else {
            ("None".to_string(), "None".to_string())
        }
    }
}
