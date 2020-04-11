use crate::keyboard::{Shortcut, ShortcutListener};
use evdev::Device;
use main_error::MainError;

use dbus::blocking::LocalConnection;
use dbus::channel::Sender;

use dbus::tree::{DataType, Factory, MethodErr, MethodType, Signal, Tree};
use dbus::Message;
use std::collections::HashMap;

use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod keyboard;

const INTERFACE: &'static str = "nl.icewind.shortcutd";

struct MutateTree<M: MethodType<D>, D: DataType> {
    factory: Factory<M, D>,
    tree: Arc<Mutex<Option<Tree<M, D>>>>,
}

impl<M: MethodType<D> + 'static, D: DataType + 'static> MutateTree<M, D> {
    fn factory(&self) -> &Factory<M, D> {
        &self.factory
    }

    fn mutate_tree(&self, f: impl FnOnce(&Factory<M, D>, Tree<M, D>) -> Tree<M, D>) {
        let tree = self.tree.lock().unwrap().take().unwrap();
        self.tree.lock().unwrap().replace(f(&self.factory, tree));
    }

    pub fn start_receive<C>(&self, connection: &C)
    where
        C: dbus::channel::MatchingReceiver<F = Box<dyn FnMut(Message, &C) -> bool>>
            + dbus::channel::Sender,
    {
        let tree = self.tree.clone();
        connection.start_receive(
            dbus::message::MatchRule::new_method_call(),
            Box::new(move |msg, c| {
                let tree_opt = tree.lock().unwrap();
                let tree = tree_opt.as_ref().unwrap();
                if let Some(replies) = tree.handle(&msg) {
                    for r in replies {
                        let _ = c.send(r);
                    }
                }
                true
            }),
        );
    }

    pub fn new(factory: Factory<M, D>, data: D::Tree) -> Self {
        let tree = factory.tree(data);
        MutateTree {
            factory,
            tree: Arc::new(Mutex::new(Some(tree))),
        }
    }
}

fn main() -> Result<(), MainError> {
    let devices = glob::glob("/dev/input/by-id/*-kbd")?
        .map(|path| Ok(Device::open(&path.unwrap())?))
        .collect::<Result<Vec<Device>, MainError>>()?;

    let listener = Arc::new(ShortcutListener::new());

    let mut signals: HashMap<String, Arc<Signal<()>>> = HashMap::default();

    let rx = listener.listen(devices);

    let mut connection = LocalConnection::new_system()?;
    connection.request_name(INTERFACE, false, true, false)?;

    let mtree = Arc::new(MutateTree::new(Factory::new_fn::<()>(), ()));

    let (new_shortcut_tx, new_shortcut_rx) = channel();

    mtree.mutate_tree(move |factory, tree| {
        let new_shortcut_tx = new_shortcut_tx.clone();
        tree.add(
            factory.object_path("/register", ()).introspectable().add(
                factory.interface(INTERFACE, ()).add_m(
                    factory
                        .method("Register", (), move |m| {
                            let shortcut_str: &str = m.msg.read1()?;

                            match shortcut_str.parse::<Shortcut>() {
                                Ok(shortcut) => {
                                    listener.add(shortcut.clone());
                                    let path = shortcut.identifier();
                                    new_shortcut_tx.send(shortcut).unwrap();

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
        .add(factory.object_path("/", ()).introspectable())
    });

    mtree.start_receive(&connection);

    // Serve clients forever.
    loop {
        connection.process(Duration::from_millis(50))?;

        while let Ok(shortcut) = new_shortcut_rx.try_recv() {
            let identifier = format!("/{}", shortcut.identifier());
            let signal = Arc::new(mtree.factory().signal("Triggered", ()));
            signals.insert(identifier.clone(), signal.clone());

            mtree.mutate_tree(move |factory, tree| {
                tree.add(
                    factory
                        .object_path(identifier, ())
                        .introspectable()
                        .add(factory.interface(INTERFACE, ()).add_s(signal)),
                )
            });
        }

        while let Ok(shortcut) = rx.try_recv() {
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
    }
}
