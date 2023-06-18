//! shortcutd is a daemon and client library to allow listening for global shortcuts for systems that don't support it
//! otherwise (such as wayland).
//!
//! The shortcutd daemon hooks into the evdev system and exposes a dbus interface for clients to hook into to.
//! By separating out the code that hooks into evdev (which needs to be done as root) into a separate daemon
//! it allows non-privileged users to hook into global shortcuts.
//!
//! Protection against clients using the shortcutd daemon for a keylogger is done by only allowing 3 shortcuts without modifiers to be registered at the same time.
//!
//! See the [README](https://github.com/icewind1991/shortcutd) for instruction to running the shortcut daemon.
//!
//! Example:
//!
//! ```rust,no_run
//! # use futures::{pin_mut, StreamExt};
//! # use shortcutd::{Shortcut, ShortcutClient};
//! # use std::error::Error;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn Error>> {
//! let client = ShortcutClient::new().await?;
//! let shortcut: Shortcut = "<Ctrl><Alt>-KeyO".parse()?;
//!
//! let stream = client.listen(shortcut).await?;
//! pin_mut!(stream);
//!
//! while let Some(event) = stream.next().await {
//!     println!("{} {}", event.shortcut, event.state);
//! }
//! # Ok(())
//! # }
//! ```
//!

mod dbus;

use dbus::{RegisterProxy, ShortcutSignalProxy};
pub use evdev_shortcut::{Key, Modifier, ModifierList, Shortcut, ShortcutEvent, ShortcutState};
use futures::Stream;
use futures::StreamExt;
use zbus::Connection;

pub struct ShortcutClient {
    connection: Connection,
}

impl ShortcutClient {
    /// Create a new client by connecting to the system dbus
    pub async fn new() -> Result<Self, zbus::Error> {
        Ok(ShortcutClient {
            connection: Connection::system().await?,
        })
    }

    /// Create a client from an existing dbus connection
    pub fn from_zbus(connection: Connection) -> Self {
        ShortcutClient { connection }
    }

    /// Listen to a shortcut
    ///
    /// Returns a stream of of shortcut events
    pub async fn listen(
        &self,
        shortcut: Shortcut,
    ) -> Result<impl Stream<Item = ShortcutEvent> + '_, zbus::Error> {
        let register = RegisterProxy::new(&self.connection).await?;
        let path = register.register(&format!("{}", shortcut)).await?;

        let p = ShortcutSignalProxy::builder(&self.connection)
            .path(path.as_str())?
            .build()
            .await?;
        let signals = p.receive_triggered().await?;

        Ok(signals.filter_map(move |signal| {
            let shortcut = shortcut.clone();
            async move {
                let pressed = signal.args().ok()?.pressed;
                Some(ShortcutEvent {
                    shortcut,
                    state: if pressed {
                        ShortcutState::Pressed
                    } else {
                        ShortcutState::Released
                    },
                })
            }
        }))
    }
}
