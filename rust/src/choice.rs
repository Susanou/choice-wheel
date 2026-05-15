use godot::classes::{ILabel, Label};
use godot::obj::{Base, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=Label, tool)]
pub struct ChoiceLabel {
    pub selected: Vec<String>,
    base: Base<Label>
}

#[godot_api]
impl ILabel for ChoiceLabel {
    fn init(base: Base<Self::Base>) -> Self {
        ChoiceLabel{
            selected: Vec::new(),
            base
        }
    }
}

#[godot_api]
impl ChoiceLabel {
    #[func]
    pub fn on_clear_choices(&mut self){
        self.selected.clear();
    }

    #[func]
    pub fn on_choice(&mut self, choice: String){
        self.selected.push(choice);
        let selected_choices = self.selected.join(", ");
        self.base_mut().set_text(&selected_choices);
    }
}