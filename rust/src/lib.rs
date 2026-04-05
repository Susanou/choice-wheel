#![feature(extend_one)]

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

pub mod selection_wheel;
pub mod ui;
pub mod choice;