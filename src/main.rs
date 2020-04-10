use crate::keyboard::{Key, Modifier, Shortcut, ShortcutListener};
use evdev::Device;
use main_error::MainError;
use std::collections::HashSet;
use std::convert::TryFrom;

use dbus::blocking::LocalConnection;
use dbus::channel::Sender;
use dbus::tree::{Factory, MethodErr};
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

mod keyboard;

fn main() -> Result<(), MainError> {
    let args: Vec<String> = std::env::args().collect();
    let device = if args.len() > 1 {
        Device::open(&args[1])?
    } else {
        eprintln!("Usage {} </dev/input/...>", args[0]);
        return Ok(());
    };

    // let shortcut = Shortcut::new(vec![Modifier::Ctrl, Modifier::Ctrl], Key::KeyP);

    let listener = ShortcutListener::new();
    // listener.add(shortcut);

    let rx = listener.listen(device);

    // Let's start by starting up a connection to the session bus and request a name.
    let mut connection: LocalConnection = LocalConnection::new_session()?;
    connection.request_name("nl.icewind.shortcutd", false, true, false)?;

    // The choice of factory tells us what type of tree we want,
    // and if we want any extra data inside. We pick the simplest variant.
    let f = Factory::new_fn::<()>();

    // We create the signal first, since we'll need it in both inside the method callback
    // and when creating the tree.
    let signal = Arc::new(f.signal("ShortcutTriggered", ()).sarg::<&str, _>("sender"));
    let signal2 = signal.clone();
    let signal3 = signal.clone();

    // We create a tree with one object path inside and make that path introspectable.
    let tree = f
        .tree(())
        .add(
            f.object_path("/shortcut", ()).introspectable().add(
                // We add an interface to the object path...
                f.interface("nl.icewind.shortcutd", ())
                    .add_m(
                        // ...and a method inside the interface.
                        f.method("Register", (), move |m| {
                            // This is the callback that will be called when another peer on the bus calls our method.
                            // the callback receives "MethodInfo" struct and can return either an error, or a list of
                            // messages to send back.

                            let name: &str = m.msg.read1()?;
                            match Shortcut::from_str(name) {
                                Ok(shortcut) => {
                                    let s = format!("Ok: {}", shortcut);
                                    listener.add(shortcut);
                                    let mret = m.msg.method_return().append1(s);
                                    Ok(vec![mret])
                                }
                                Err(_) => Err(MethodErr::invalid_arg("Failed to parse shortcut")),
                            }
                            // let s = format!("Hello {}!", name);
                            // let mret = m.msg.method_return().append1(s);
                            //
                            // // let sig = signal
                            // //     .msg(m.path.get_name(), m.iface.get_name())
                            // //     .append1(&*name);
                            //
                            // // Two messages will be returned - one is the method return (and should always be there),
                            // // and in our case we also have a signal we want to send at the same time.
                            // Ok(vec![mret, sig])

                            // Our method has one output argument and one input argument.
                        })
                        .outarg::<&str, _>("reply")
                        .inarg::<&str, _>("name"), // We also add the signal to the interface. This is mainly for introspection.
                    )
                    .add_s(signal2), // Also add the root path, to help introspection from debugging tools.
            ),
        )
        .add(f.object_path("/", ()).introspectable());

    // We add the tree to the connection so that incoming method calls will be handled.
    tree.start_receive(&connection);

    // Serve clients forever.
    loop {
        connection.process(Duration::from_millis(50))?;

        while let Ok(shortcut) = rx.try_recv() {
            connection
                .send(
                    signal3
                        .msg(&"/shortcut".into(), &"nl.icewind.shortcutd".into())
                        .append1(&format!("{}", shortcut)),
                )
                .unwrap();
        }
    }

    Ok(())
}
