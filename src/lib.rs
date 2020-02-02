//! A Rust library for Floaout.
//!
//! Floaout is the forefront audio format that enables immersive sound which takes advantage of both channel-based and object-based system.

// Enable to use seek_relative method.
#![feature(bufreader_seek_relative)]

pub mod format;
pub mod io;