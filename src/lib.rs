#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)] // just for now since we get literally over 300 warnings about improper c type with u128 rn

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
