use main_error::MainError;

use dbus::blocking::LocalConnection;
use dbus::channel::Sender;

use dbus::tree::{Factory, MethodErr, Signal};
use std::collections::HashMap;

use evdev_shortcut::{Shortcut, ShortcutListener};
use glob::GlobError;
use std::path::PathBuf;
use std::sync::mpsc::{channel, TryRecvError};
use std::sync::Arc;
use std::time::Duration;

const INTERFACE: &'static str = "nl.icewind.shortcutd";

fn main() {
    loop {
        let _ = std::panic::catch_unwind(|| {
            let _ = run();
        });

        eprintln!("crashed, restarting");

        std::thread::sleep(Duration::from_secs(15));
    }
}

fn run() -> Result<(), MainError> {
    let devices =
        glob::glob("/dev/input/by-id/*-kbd")?.collect::<Result<Vec<PathBuf>, GlobError>>()?;

    let listener = Arc::new(ShortcutListener::new());

    let mut signals: HashMap<String, Arc<Signal<()>>> = HashMap::default();

    let shortcut_trigger_rx = listener.listen(&devices)?;

    let mut connection = LocalConnection::new_system()?;
    connection.request_name(INTERFACE, false, true, false)?;

    let (shortcut_register_tx, shortcut_register_rx) = channel();

    let factory = Factory::new_fn::<()>();
    let mut tree = factory
        .tree(())
        .add(
            factory.object_path("/register", ()).introspectable().add(
                factory.interface(INTERFACE, ()).add_m(
                    factory
                        .method("Register", (), move |m| {
                            let shortcut_str: &str = m.msg.read1()?;

                            match shortcut_str.parse::<Shortcut>() {
                                Ok(shortcut) => {
                                    let path = shortcut.identifier();
                                    shortcut_register_tx.send(shortcut).unwrap();

                                    Ok(vec![m.msg.method_return().append1(format!("/{}", path))])
                                }
                                Err(_) => Err(MethodErr::invalid_arg("Malformed shortcut")),
                            }
                        })
                        .outarg::<&str, _>("path")
                        .inarg::<&str, _>("shortcut"),
                ),
            ),
        )
        .add(factory.object_path("/", ()).introspectable());

    // Serve clients forever.
    loop {
        connection.process_with_tree(&tree, Duration::from_millis(50))?;

        while let Ok(shortcut) = shortcut_register_rx.try_recv() {
            let identifier = format!("/{}", shortcut.identifier());

            listener.add(shortcut);

            let signal = Arc::new(factory.signal("Triggered", ()));
            signals.insert(identifier.clone(), signal.clone());

            tree = tree.add(
                factory
                    .object_path(identifier, ())
                    .introspectable()
                    .add(factory.interface(INTERFACE, ()).add_s(signal)),
            );
        }

        match shortcut_trigger_rx.try_recv() {
            Ok(shortcut) => {
                let identifier = format!("/{}", shortcut.identifier());
                if let Some(signal) = signals.get(&identifier) {
                    connection
                        .send(
                            signal
                                .clone()
                                .msg(&identifier.into(), &INTERFACE.into())
                                .append1(&format!("{}", shortcut)),
                        )
                        .unwrap();
                }
            }
            Err(TryRecvError::Disconnected) => panic!("keyboard listener crashed"),
            _ => {}
        }
    }
}
