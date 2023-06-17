# shortcutd

Global shortcuts using evdev

## What

shortcutd is a daemon and client library to allow listening for global shortcuts for systems that don't support it
otherwise (such as wayland).

The shortcutd daemon hooks into the evdev system and exposes a dbus interface for clients to hook into to.
By separating out the code that hooks into evdev (which needs to be done as root) into a separate daemon
it allows non-privileged users to hook into global shortcuts.

Protection against clients using the shortcutd daemon for a keylogger is done by only allowing 3 shortcuts without modifiers to be registered at the same time.

## Starting the daemon

- Copy the dbus configuration `nl.icewind.shortcutd.conf` into `/etc/dbus-1/system.d/`
- Start the daemon as root (or use the provided systemd service).

## Rust api

```rust
use futures::{pin_mut, StreamExt};
use shortcutd::{Shortcut, ShortcutClient};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ShortcutClient::new().await?;
  
    let shortcut: Shortcut = "<Ctrl><Alt>-KeyO".parse()?;
  
    let stream = client.listen(shortcut).await?;
  
    pin_mut!(stream);
  
    while let Some(event) = stream.next().await {
      println!("{} {}", event.shortcut, event.state.as_str());
    }
    Ok(())
}

```

## D-Bus api

- register a new shortcut using the `Register` method at `nl.icewind.shortcutd`/`register`
- listen to the signal at the path returned from the `Register` method to get notified when the shortcut is triggered.
  
  A boolean parameter is provided with the signal to distinguish shortcut presses from releases.

