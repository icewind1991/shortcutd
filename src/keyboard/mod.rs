use evdev::Device;
pub use keycodes::Key;
use parse_display::{Display, FromStr, ParseError};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt::{self, Display};
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};

mod keycodes;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Display, FromStr)]
pub enum Modifier {
    Alt,
    LeftAlt,
    RightAlt,
    Ctrl,
    LeftCtrl,
    RightCtrl,
    Shift,
    LeftShift,
    RightShift,
    Meta,
    LeftMeta,
    RightMeta,
}

impl Modifier {
    pub fn is_modifier(&self, key: Key) -> bool {
        match self {
            Modifier::Alt => key == Key::KeyLeftAlt || key == Key::KeyRightAlt,
            Modifier::LeftAlt => key == Key::KeyLeftAlt,
            Modifier::RightAlt => key == Key::KeyRightAlt,
            Modifier::Ctrl => key == Key::KeyLeftCtrl || key == Key::KeyRightCtrl,
            Modifier::LeftCtrl => key == Key::KeyLeftCtrl,
            Modifier::RightCtrl => key == Key::KeyRightCtrl,
            Modifier::Meta => key == Key::KeyLeftMeta || key == Key::KeyRightMeta,
            Modifier::LeftMeta => key == Key::KeyLeftMeta,
            Modifier::RightMeta => key == Key::KeyRightMeta,
            Modifier::Shift => key == Key::KeyLeftshift || key == Key::KeyRightshift,
            Modifier::LeftShift => key == Key::KeyLeftshift,
            Modifier::RightShift => key == Key::KeyRightshift,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ModifierList(Vec<Modifier>);

impl Display for ModifierList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for modifier in self.0.iter() {
            write!(f, "<{}>", modifier)?;
        }
        Ok(())
    }
}

impl FromStr for ModifierList {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ModifierList(
            s.split('>')
                .filter(|part| !part.is_empty())
                .map(|part| {
                    if !part.starts_with('<') {
                        Err(ParseError::with_message("Invalid modifier"))
                    } else {
                        Ok(part[1..].parse::<Modifier>()?)
                    }
                })
                .collect::<Result<Vec<Modifier>, ParseError>>()?,
        ))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Display, FromStr)]
#[display("{modifiers}-{key}")]
pub struct Shortcut {
    modifiers: ModifierList,
    key: Key,
}

#[test]
fn test_shortcut_format() {
    assert_eq!(
        "<Ctrl>-KeyP",
        format!("{}", Shortcut::new(vec![Modifier::Ctrl], Key::KeyP))
    );

    assert_eq!(
        "<LeftCtrl><LeftAlt>-KeyLeft",
        format!(
            "{}",
            Shortcut::new(vec![Modifier::LeftCtrl, Modifier::LeftAlt], Key::KeyLeft)
        )
    );

    assert_eq!(
        Shortcut::from_str("<LeftCtrl><LeftAlt>-KeyLeft").unwrap(),
        Shortcut::new(vec![Modifier::LeftCtrl, Modifier::LeftAlt], Key::KeyLeft)
    );
}

impl Shortcut {
    pub fn new(modifiers: Vec<Modifier>, key: Key) -> Self {
        Shortcut {
            modifiers: ModifierList(modifiers),
            key,
        }
    }
}

impl Shortcut {
    pub fn is_triggered(&self, active_keys: &HashSet<Key>) -> bool {
        for modifier in &self.modifiers.0 {
            if active_keys.iter().any(|key| modifier.is_modifier(*key)) {
                break;
            }

            return false;
        }

        active_keys.contains(&self.key)
    }
}

pub struct ShortcutListener {
    shortcuts: Arc<Mutex<HashSet<Shortcut>>>,
}

impl ShortcutListener {
    pub fn new() -> Self {
        ShortcutListener {
            shortcuts: Arc::default(),
        }
    }

    pub fn listen(&self, mut device: Device) -> Receiver<Shortcut> {
        let (tx, rx) = channel();

        let shortcuts = self.shortcuts.clone();

        std::thread::spawn(move || {
            let mut active_keys = HashSet::new();

            loop {
                let mut got_event = false;

                for ev in device.events().unwrap() {
                    got_event = true;

                    if let Ok(key) = Key::try_from(ev.code) {
                        match ev.value {
                            1 => active_keys.insert(key),
                            0 => active_keys.remove(&key),
                            _ => false,
                        };
                    }
                }

                if got_event {
                    for shortcut in shortcuts.lock().unwrap().iter() {
                        if shortcut.is_triggered(&active_keys) {
                            tx.send(shortcut.clone()).unwrap()
                        }
                    }
                }
            }
        });

        rx
    }

    pub fn add(&self, shortcut: Shortcut) {
        self.shortcuts.lock().unwrap().insert(shortcut);
    }

    pub fn remove(&self, shortcut: Shortcut) {
        self.shortcuts.lock().unwrap().remove(&shortcut);
    }
}
