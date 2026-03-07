use godot::classes::{CanvasLayer, Control, ICanvasLayer, Input, InputMap};
use godot::obj::Singleton;
use godot::obj::{Base, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=CanvasLayer, tool)]
struct Ui {
    base: Base<CanvasLayer>
}

#[godot_api]

impl ICanvasLayer for Ui {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }

    fn process(&mut self, __delta: f64) {
        if InputMap::has_action(InputMap::singleton().upcast_mut(), "rotate_wheel") && Input::is_action_just_pressed(Input::singleton().upcast_mut(), "rotate_wheel") {
            self.base().get_node_as::<Control>("ChoiceWheel").show();
        } else if InputMap::has_action(InputMap::singleton().upcast_mut(), "rotate_wheel") && Input::is_action_just_released(Input::singleton().upcast_mut(), "rotate_wheel") {
            self.base().get_node_as::<Control>("ChoiceWheel").hide();
        }
    }
}

