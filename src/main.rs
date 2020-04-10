use evdev::Device;
use main_error::MainError;
use std::collections::HashSet;
use crate::keyboard::{Key, Shortcut, Modifier};
use std::convert::TryFrom;

mod keyboard;

fn main() -> Result<(), MainError> {
    let args: Vec<String> = std::env::args().collect();
    let mut device = if args.len() > 1 {
        Device::open(&args[1])?
    } else {
        eprintln!("Usage {} </dev/input/...>", args[0]);
        return Ok(());
    };

    let mut keys = HashSet::new();
    let shortcut = Shortcut {
        modifiers: vec![Modifier::AnyCtrl, Modifier::AnyCtrl],
        key: Key::KeyP
    };

    loop {
        let mut got_event = false;

        for ev in device.events()? {
            got_event = true;

            if let Ok(key) = Key::try_from(ev.code) {
                match ev.value {
                    1 => keys.insert(key),
                    0 => keys.remove(&key),
                    _ => false,
                };
            }
        }

        if got_event {
            if shortcut.is_triggered(&keys) {
                println!("triggered");
            }
        }
    }

    Ok(())
}
