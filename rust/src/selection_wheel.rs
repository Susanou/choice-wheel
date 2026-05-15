use crate::choice::ChoiceLabel;
use godot::builtin::{Color, StringName, Vector2};
use godot::classes::control::LayoutPreset;
use godot::classes::file_access::ModeFlags;
use godot::classes::{Control, IControl, Label};
use godot::global::{godot_print, HorizontalAlignment};
use godot::obj::{Base, Gd, NewAlloc, WithBaseField, WithUserSignals};
use godot::prelude::{godot_api, GFile, GodotClass, Node, OnReady, Variant};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;
use std::fs;

#[derive(GodotClass)]
#[class(base=Control, tool)]
pub struct Wheel {
    base: Base<Control>,
    #[export]
    is_spin: bool,

    #[export]
    bg_color: Color,
    #[export]
    line_color: Color,

    #[export]
    line_width: i32,

    #[export]
    outer_radius: i64,
    #[export]
    inner_radius: i64,

    pub next_wheel_id: i32,
    pub possible_wheels: Option<Root>,
    pub chosen_item: OnReady<Gd<ChoiceLabel>>,
    items: Vec<WheelItem>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub wheels: Vec<WheelChoice>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WheelChoice {
    pub name: String,
    pub id: i32,
    pub results: Vec<WheelItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WheelItem {
    pub id: i64,
    pub name: String,
    pub probability: f64,
    #[serde(rename = "next_wheel_id")]
    pub next_wheel_id: i32,
    #[serde(default)]
    from: f32,
    #[serde(default)]
    to: f32,
}


#[godot_api]
impl Wheel {

    #[signal]
    fn wheel_end_spin(choice: String);

    #[func]
    fn on_btn_load_file(&mut self){
        self.chosen_item.call("on_clear_choices", &[]);
        godot_print!("{:?}", self.items.len());
        // Open file in read mode
        let my_file = GFile::open("res://roulette.json", ModeFlags::READ);

        let file = fs::File::open(my_file.unwrap().path_absolute().to_string()).expect("file should open read only");
        let json: Root = serde_json::from_reader(file)
            .expect("file should be proper JSON");

        self.possible_wheels = Option::from(json);
        godot_print!("{:?}", self.possible_wheels.as_ref().unwrap().wheels.get(0).unwrap());

        let first_wheel: WheelChoice = self.possible_wheels.as_ref().unwrap().wheels.get(0).unwrap().clone();
        self.items = first_wheel.results;
        godot_print!("{:?}", self.items);
        godot_print!("{:?}", self.items.len());

        self.setup_labels();
    }

    #[func]
    fn on_btn_spin_wheel(&mut self){
        if !self.is_spin {
            self.is_spin = true;

            let mut tween = self.base_mut().get_tree().create_tween().set_parallel_ex().parallel(false).done();

            let mut rng = rand::rng();
            let reward_pos = rng.random_range(0..360);


            tween.connect(
                "finished",	// boilerplate
                &self.to_gd().callable("end_spin").bind(&[Variant::from(reward_pos)]));


            // 360 *  speed * power
            tween.tween_property(
                &self.to_gd(),
                "rotation_degrees" ,
                &Variant::from(reward_pos + 360 * 10 * 2),
                3.0
            );
        }
    }

    #[func]
    fn on_load_next_wheel(&mut self){
        if self.next_wheel_id != -1 {
            let next_wheel_pos = self.possible_wheels.as_ref().unwrap().wheels.iter().position(|x| x.id == self.next_wheel_id).unwrap();
            self.items = self.possible_wheels.as_ref().unwrap().wheels[next_wheel_pos].results.clone();
            self.setup_labels();
        } else {
            self.signals().wheel_end_spin().emit("END".to_string());
        }
    }

    #[func]
    fn end_spin(&mut self, reward_pos: i32) {
        godot_print!("end_spin");
        let mut front_node = self.to_gd();

        let old_rotation = front_node.get_rotation_degrees();

        if old_rotation > 360.0{
            let deg = old_rotation % 360.0;
            front_node.set_rotation_degrees(deg);
        }

        self.is_spin = false;

        let mut chosen_item: i32 = -1;
        self.items.iter().enumerate().for_each(|(i, item)| {
            if reward_pos as f32 >= item.from && reward_pos as f32 <= item.to {
                godot_print!("{} ", item.name);
                chosen_item = i as i32;
            }
        });

        if chosen_item < 0 {
            panic!();
        } else {
            let item = self.items.get(chosen_item as usize).unwrap().clone();
            let reward = item.name.clone();
            self.signals().wheel_end_spin().emit(reward);

            self.next_wheel_id = item.next_wheel_id;
        }
    }

    fn setup_labels(&mut self) {
        let outer_radius = self.outer_radius as f32;
        let inner_radius = self.inner_radius as f32;

        let mut options = self.items.clone();

        for child in self.base_mut().get_children().iter_shared() {

            if child.get_name() != StringName::from("Button")
            {
                Gd::free(child);
            }
        }

        let mut updated_items : Vec<WheelItem> = vec!();

        for (i, name) in options.iter_mut().enumerate() {

            let mut copy_label  = Label::new_alloc();
            copy_label.set_text(&name.name);

            let start_rads = i as f32 / self.items.len() as f32 * TAU;
            let end_rads = (i + 1) as f32 / self.items.len() as f32 * TAU;

            name.from = start_rads.to_degrees();
            name.to = end_rads.to_degrees();

            let mid_rads = (start_rads + end_rads) / 2.0 * -1.0;
            let radius_mid = (inner_radius + outer_radius) / 2.0;

            let draw_pos = radius_mid * Vector2::from_angle(mid_rads);// * offset;

            copy_label.set_position(draw_pos);
            copy_label.set_rotation(mid_rads);
            copy_label.set_horizontal_alignment(HorizontalAlignment::RIGHT);
            let mut node = copy_label.upcast::<Node>();

            self.base_mut().add_child(&node);
            node.set_owner(&self.base().clone().upcast::<Node>());

            updated_items.push(name.clone());

            //godot_print!("showing name: {}", name);
        }

        self.items = updated_items;
        self.base_mut().queue_redraw();
    }
}

#[godot_api]
impl IControl for Wheel {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            is_spin: false,
            bg_color: Color::BLACK,
            line_color: Color::WHITE,
            line_width: 4,
            outer_radius: 256,
            inner_radius: 64,
            items: Vec::new(),
            next_wheel_id: -1,
            possible_wheels: None,
            chosen_item: OnReady::from_node("%Choice")
        }
    }

    fn draw(&mut self) {
        let outer_radius = self.outer_radius as f32;
        let bg_color = self.bg_color;
        let inner_radius = self.inner_radius as f32;
        let line_color = self.line_color;
        let line_width = self.line_width as f32;

        self.base_mut().set_anchors_preset(LayoutPreset::FULL_RECT);

        self.base_mut()
            .draw_circle(Vector2::ZERO, outer_radius, bg_color);
        self.base_mut()
            .draw_arc_ex(Vector2::ZERO, inner_radius, 0.0, TAU, 256, line_color)
            .width(line_width)
            .antialiased(true)
            .done();

        let mut items = self.items.clone();

        if self.items.len() >= 2 {
            for (i, _name) in items.iter_mut().enumerate() {
                let rads = i as f32 / self.items.len() as f32 * TAU;
                let point = Vector2::from_angle(rads);
                self.base_mut()
                    .draw_line_ex(point * inner_radius, point * outer_radius, line_color)
                    .width(line_width)
                    .antialiased(true)
                    .done();
            }
        }

        godot_print!("Draw was called");
    }

    fn process(&mut self, _delta: f64) {
    }

    fn ready(&mut self) {
        let choice_label = (*self.chosen_item).clone();

        self.signals()
            .wheel_end_spin()
            .connect_other(&choice_label, ChoiceLabel::on_choice);
    }
}