use std::{process::exit, thread::sleep, time::Duration};

use wooting_rgb::{Key, RgbKeyboard};

fn main() {
    println!("Waiting until keyboard is connected...");
    loop {
        // Only continue if keyboard is connected.
        if wooting_rgb::is_wooting_keyboard_connected() {
            break;
        }
    }
    println!("Connected...");

    println!("Setting callback...");
    wooting_rgb::set_disconnected_callback(|| {
        println!("Callback triggered...");
        println!("Finished!");
        exit(0);
    });

    println!("Waiting until disconnect...");
    let mut keyboard = RgbKeyboard;
    let mut set = true;
    loop {
        // Trigger a read so that a disconnect will be noticed. This is a limitation of that API.
        if set {
            keyboard.direct_reset_key(Key::Escape);
        } else {
            keyboard.direct_set_key(Key::Escape, 255, 255, 255);
        }
        set = !set;
        sleep(Duration::from_millis(1000));
    }
}
