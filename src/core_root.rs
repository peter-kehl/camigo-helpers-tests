//! Testing `no_std` & no-alloc-compatible code of [camigo_helpers].
#![no_std]

extern crate alloc as rust_alloc;

#[path = "core/core_mod.rs"]
mod core;