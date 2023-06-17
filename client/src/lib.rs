pub use evdev_shortcut::{Key, Modifier, ModifierList, Shortcut, ShortcutEvent, ShortcutState};
use futures::Stream;
use futures::StreamExt;
use zbus::dbus_proxy;
use zbus::{fdo, Connection};

#[dbus_proxy(
    interface = "nl.icewind.shortcutd",
    default_service = "nl.icewind.shortcutd",
    default_path = "/register"
)]
trait Register {
    async fn register(&self, shortcut: &str) -> fdo::Result<String>;
}

#[dbus_proxy(
    interface = "nl.icewind.shortcutd",
    default_service = "nl.icewind.shortcutd"
)]
trait ShortcutSignal {
    #[dbus_proxy(signal)]
    async fn triggered(&self, pressed: bool) -> fdo::Result<()>;
}

pub struct ShortcutClient {
    connection: Connection,
}

impl ShortcutClient {
    pub async fn new() -> Result<Self, zbus::Error> {
        Ok(ShortcutClient {
            connection: Connection::system().await?,
        })
    }

    pub fn from_zbus(connection: Connection) -> Self {
        ShortcutClient { connection }
    }

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
