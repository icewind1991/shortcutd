use zbus::dbus_proxy;
use zbus::fdo;

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
