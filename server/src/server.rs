use evdev_shortcut::{Shortcut, ShortcutEvent, ShortcutListener, ShortcutState};
use futures::stream::StreamExt;
use glob::GlobError;
use main_error::MainError;
use std::path::PathBuf;
use tracing::{error, info};
use zbus::export::futures_util::pin_mut;
use zbus::{dbus_interface, fdo, ConnectionBuilder, ObjectServer, SignalContext};

struct Register {
    listener: ShortcutListener,
    bare_count: usize,
}

const MAX_BARE: usize = 3;

#[dbus_interface(name = "nl.icewind.shortcutd")]
impl Register {
    async fn register(
        &mut self,
        shortcut: &str,
        #[zbus(object_server)] server: &ObjectServer,
    ) -> Result<String, fdo::Error> {
        match shortcut.parse::<Shortcut>() {
            Ok(shortcut) => {
                if shortcut.modifiers.is_empty() && !self.listener.has(&shortcut) {
                    if self.bare_count >= MAX_BARE {
                        return Err(fdo::Error::InvalidArgs(format!(
                            "Only {} shortcuts without modifiers are allowed",
                            MAX_BARE
                        )));
                    }
                    self.bare_count += 1;
                }
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
        bare_count: 0,
    };
    let conn = ConnectionBuilder::system()
        .map_err(|e| {
            error!(error = ?e, "error while connecting to dbus system socket");
            e
        })?
        .name("nl.icewind.shortcutd")?
        .serve_at("/register", bus)?
        .build()
        .await
        .map_err(|e| {
            error!(error = ?e, "error while binding dbus service");
            e
        })?;

    let server = conn.object_server();

    pin_mut!(shortcut_events);
    while let Some(event) = shortcut_events.next().await {
        let event: ShortcutEvent = event;
        let identifier = format!("/{}", event.shortcut.identifier());
        if let Ok(signal_interface) = server.interface::<_, ShortcutSignal>(identifier).await {
            if let Err(e) = ShortcutSignal::triggered(
                signal_interface.signal_context(),
                event.state == ShortcutState::Pressed,
            )
            .await
            {
                eprintln!("{e:#}");
            }
        }
    }

    Ok(())
}
