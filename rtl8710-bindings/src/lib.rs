#![feature(alloc)]
#![feature(global_allocator)]
#![feature(lang_items)]
#![no_std]
#![allow(warnings)]

extern crate cty;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
