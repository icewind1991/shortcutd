use shortcutd::{Shortcut, ShortcutClient};
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = ShortcutClient::new()?;

    let shortcut: Shortcut = "<Ctrl><Alt>-KeyP".parse()?;

    client.register(shortcut, |s| {
        eprintln!("shortcut1 {}", s);
    })?;

    let shortcut: Shortcut = "<Ctrl><Alt>-KeyO".parse()?;

    client.register(shortcut, |s| {
        eprintln!("shortcut2 {}", s);
    })?;

    loop {
        client.process(Duration::from_millis(1000))?;
    }
}
