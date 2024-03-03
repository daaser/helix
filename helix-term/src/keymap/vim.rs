use std::collections::HashMap;

use super::macros::keymap;
use super::{KeyTrie, Mode};
use helix_core::hashmap;

pub fn default() -> HashMap<Mode, KeyTrie> {
    let normal = keymap!({ "Normal mode"
        "c" => { "Change"
            "t" => change_till_char,
        },
    });
    hashmap!(
        Mode::Normal => normal,
    )
}
