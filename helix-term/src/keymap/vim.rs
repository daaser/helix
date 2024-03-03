use std::collections::HashMap;

use super::macros::keymap;
use super::{KeyTrie, Mode};
use helix_core::hashmap;

// this function is currently unused but would serve as a basis for optionally
// configuring vim keybindings in the future
pub fn default() -> HashMap<Mode, KeyTrie> {
    let normal = keymap!({ "Normal mode"
        "c" => { "Change"
            "t" => change_till_char,
        },
        "d" => { "Delete"
            "t" => delete_till_char,
        },
    });
    hashmap!(
        Mode::Normal => normal,
    )
}
