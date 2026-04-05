use godot::builtin::GString;
use godot::classes::{ILabel, Label};
use godot::obj::{Base, WithBaseField, WithUserSignals};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=Label, tool)]
pub struct ChoiceLabel {
    base: Base<Label>
}

#[godot_api]
impl ILabel for ChoiceLabel {
    fn init(base: Base<Self::Base>) -> Self {
        ChoiceLabel{
            base
        }
    }
}

#[godot_api]
impl ChoiceLabel {
    #[func]
    pub fn on_choice(&mut self, choice: String){
        self.base_mut().set_text(&choice);
    }
}