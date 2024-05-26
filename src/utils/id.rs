use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

pub type Id = i64;

pub struct Gen {
    id_map: Mutex<HashMap<&'static str, Id>>
}

lazy_static! {
    static ref GEN_SINGLETON: Gen = Gen {
        id_map: Mutex::new(HashMap::new())
    };
}

impl Gen {
    /// Returns an identifier unique among all [`Gen::next`] calls with the same
    /// argument `name`.
    pub fn next(name: &'static str) -> Id {
        let mut id_map = GEN_SINGLETON.id_map.lock().unwrap();
        if let Some(id) = id_map.get_mut(&name) {
            let result = *id;
            *id += 1;
            result
        } else {
            id_map.insert(name, 1);
            0
        }
    }
}
