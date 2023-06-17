use main_error::MainError;
use evdev_shortcut::{Shortcut, ShortcutEvent, ShortcutListener, ShortcutState};
use glob::GlobError;
use std::path::PathBuf;
use futures::stream::StreamExt;
use tracing::info;
use zbus::{ConnectionBuilder, dbus_interface, fdo, SignalContext, ObjectServer};
use zbus::export::futures_util::pin_mut;

struct Register {
    listener: ShortcutListener,
}

#[dbus_interface(name = "nl.icewind.shortcutd")]
impl Register {
    async fn register(&mut self, shortcut: &str, #[zbus(object_server)] server: &ObjectServer) -> Result<String, fdo::Error> {
        match shortcut.parse::<Shortcut>() {
            Ok(shortcut) => {
                info!(%shortcut, "registering shortcut");
                self.listener.add(shortcut.clone());
                let path = format!("/{}", shortcut.identifier());
                if let Err(e) = server.at(path.as_str(), ShortcutSignal).await {
                    eprintln!("{e:#}");
                }

                Ok(path)
            }
            Err(_) => Err(fdo::Error::InvalidArgs("Malformed shortcut".into())),
        }
    }
}

struct ShortcutSignal;

#[dbus_interface(name = "nl.icewind.shortcutd")]
impl ShortcutSignal {
    #[dbus_interface(signal)]
    async fn triggered(signal_ctxt: &SignalContext<'_>, pressed: bool) -> zbus::Result<()>;
}

#[tokio::main]
async fn main() -> Result<(), MainError> {
    tracing_subscriber::fmt::init();
    let devices =
        glob::glob("/dev/input/by-id/*-kbd")?.collect::<Result<Vec<PathBuf>, GlobError>>()?;

    let listener = ShortcutListener::new();
    let shortcut_events = listener.listen(&devices)?;

    let bus = Register {
        listener,
    };
    let conn = ConnectionBuilder::system()?
        .name("nl.icewind.shortcutd")?
        .serve_at("/register", bus)?
        .build()
        .await?;

    let server = conn.object_server();

    pin_mut!(shortcut_events);
    while let Some(event) = shortcut_events.next().await {
        let event: ShortcutEvent = event;
        let identifier = format!("/{}", event.shortcut.identifier());
        if let Ok(signal_interface) = server.interface::<_, ShortcutSignal>(identifier).await {
            if let Err(e) = ShortcutSignal::triggered(signal_interface.signal_context(), event.state == ShortcutState::Pressed).await {
                eprintln!("{e:#}");
            }
        }
    }

    Ok(())
}
