#![feature(extend_one)]

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

mod player;
mod drawing_test;
pub mod selection_wheel;
pub mod ui;