mod keycodes;

pub use keycodes::Key;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
pub enum Modifier {
    AnyAlt,
    LeftAlt,
    RightAlt,
    AnyCtrl,
    LeftCtrl,
    RightCtrl,
    AnyShift,
    LeftShift,
    RightShift,
    AnyMeta,
    LeftMeta,
    RightMeta,
}

impl Modifier {
    pub fn is_modifier(&self, key: Key) -> bool {
        match self {
            Modifier::AnyAlt => key == Key::KeyLeftAlt || key == Key::KeyRightAlt,
            Modifier::LeftAlt => key == Key::KeyLeftAlt,
            Modifier::RightAlt => key == Key::KeyRightAlt,
            Modifier::AnyCtrl => key == Key::KeyLeftCtrl || key == Key::KeyRightCtrl,
            Modifier::LeftCtrl => key == Key::KeyLeftCtrl,
            Modifier::RightCtrl => key == Key::KeyRightCtrl,
            Modifier::AnyMeta => key == Key::KeyLeftMeta || key == Key::KeyRightMeta,
            Modifier::LeftMeta => key == Key::KeyLeftMeta,
            Modifier::RightMeta => key == Key::KeyRightMeta,
            Modifier::AnyShift => key == Key::KeyLeftshift || key == Key::KeyRightshift,
            Modifier::LeftShift => key == Key::KeyLeftshift,
            Modifier::RightShift => key == Key::KeyRightshift,
        }
    }
}


pub struct Shortcut {
    pub modifiers: Vec<Modifier>,
    pub key: Key,
}

impl Shortcut {
    pub fn is_triggered(&self, active_keys: &HashSet<Key>) -> bool {
        for modifier in &self.modifiers {
            if active_keys.iter().any(|key| modifier.is_modifier(*key)) {
                break;
            }

            return false;
        }

        active_keys.contains(&self.key)
    }
}