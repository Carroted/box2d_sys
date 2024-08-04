#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)] // just for now since we get literally over 300 warnings about improper c type with u128 rn

use std::hash::{Hash, Hasher};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Hash for b2BodyId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index1.hash(state);
        self.world0.hash(state);
        self.revision.hash(state);
    }
}

impl PartialEq for b2BodyId {
    fn eq(&self, other: &Self) -> bool {
        self.index1 == other.index1
            && self.world0 == other.world0
            && self.revision == other.revision
    }
}

impl Eq for b2BodyId {}
