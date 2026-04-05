use crate::choice::ChoiceLabel;
use godot::builtin::{Array, Callable, Color, GString, StringName, Vector2};
use godot::classes::control::LayoutPreset;
use godot::classes::{Control, IControl, ILabel, Label, Node2D, ThemeDb};
use godot::global::{godot_print, HorizontalAlignment};
use godot::obj::{Base, Gd, NewAlloc, OnEditor, Singleton, WithBaseField, WithUserSignals};
use godot::prelude::{godot_api, GodotClass, Node, OnReady, Variant};
use rand::{random, RngExt};
use std::any::{type_name, Any};
use std::f32::consts::TAU;

const SPRITE_SIZE: Vector2 = Vector2{x: 32.0, y: 32.0};

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

    #[export]
    options: Array<GString>,

    chosen_item: OnReady<Gd<ChoiceLabel>>,
    items: Vec<Item>
}

struct Item {
    name: String,
    from: f32,
    to: f32,
}


#[godot_api]
impl Wheel {

    #[signal]
    fn wheel_end_spin(choice: String);

    #[func]
    fn on_btn_spin_wheel(&mut self){
        if !self.is_spin {
            self.is_spin = true;
            let mut tween = self.base_mut().get_tree().create_tween().set_parallel_ex().parallel(true).done();
            let mut front_node = self.to_gd();

            tween.connect(
                "finished",	// boilerplate
                &Callable::from_fn(
                    "finished",	// boilerplate
                    move |_| {
                        let old_rotation = front_node.get_rotation_degrees();

                        if old_rotation > 360.0{
                            let deg = old_rotation % 360.0;
                            front_node.set_rotation_degrees(deg);
                        }
                    },
                ));

            let mut rng = rand::rng();
            let reward_pos = rng.random_range(0..360);
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
                let reward = self.items.get(chosen_item as usize).unwrap().name.clone();
                self.signals().wheel_end_spin().emit(reward);
            }

            // 360 *  speed * power
            tween.tween_property(
                &self.to_gd(),
                "rotation_degrees" ,
                &Variant::from(reward_pos + 360 * 10 * 2),
                3.0
            );
        }
    }

    fn setup_labels(&mut self) {
        self.items.clear();

        let outer_radius = self.outer_radius as f32;
        let bg_color = self.bg_color;
        let inner_radius = self.inner_radius as f32;
        let default_font = ThemeDb::singleton().get_fallback_font();
        let label = Label::new_alloc();

        let names = self.options.clone();

        for child in self.base_mut().get_children().iter_shared() {

            if (child.get_name() != StringName::from("Button"))
            {
                Gd::free(child);
            }
        }

        for (i, name) in names.iter_shared().enumerate() {

            let mut copy_label  = Label::new_alloc();
            copy_label.set_text(&name);

            let start_rads = i as f32 / self.options.len() as f32 * TAU;
            let end_rads = (i + 1) as f32 / self.options.len() as f32 * TAU;

            self.items.push(Item{
                name: name.to_string(),
                from: start_rads.to_degrees(),
                to: end_rads.to_degrees(),
            });

            let mid_rads = (start_rads + end_rads) / 2.0 * -1.0;
            let radius_mid = (inner_radius + outer_radius) / 2.0;

            let draw_pos = radius_mid * Vector2::from_angle(mid_rads);// * offset;

            copy_label.set_position(draw_pos);
            copy_label.set_rotation(mid_rads);
            copy_label.set_horizontal_alignment(HorizontalAlignment::RIGHT);
            let mut node = copy_label.upcast::<Node>();

            self.base_mut().add_child(&node);
            node.set_owner(&self.base().clone().upcast::<Node>());

            //godot_print!("showing name: {}", name);
        }
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
            options: Array::new(),
            items: Vec::new(),
            chosen_item: OnReady::from_node("%Choice")
        }
    }

    fn draw(&mut self) {
        let outer_radius = self.outer_radius as f32;
        let bg_color = self.bg_color;
        let inner_radius = self.inner_radius as f32;
        let line_color = self.line_color;
        let line_width = self.line_width as f32;
        let default_font = ThemeDb::singleton().get_fallback_font();

        self.base_mut().set_anchors_preset(LayoutPreset::FULL_RECT);

        self.base_mut()
            .draw_circle(Vector2::ZERO, outer_radius, bg_color);
        self.base_mut()
            .draw_arc_ex(Vector2::ZERO, inner_radius, 0.0, TAU, 256, line_color)
            .width(line_width)
            .antialiased(true)
            .done();

        let names = self.options.clone();

        if self.options.len() >= 2 {
            for (i, _name) in names.iter_shared().enumerate() {
                let rads = i as f32 / self.options.len() as f32 * TAU;
                let point = Vector2::from_angle(rads);
                self.base_mut()
                    .draw_line_ex(point * inner_radius, point * outer_radius, line_color)
                    .width(line_width)
                    .antialiased(true)
                    .done();
            }
        }
    }

    fn process(&mut self, _delta: f64) {
        if self.options.len() != self.items.len() {
            self.setup_labels();
            self.base_mut().queue_redraw();
        }
    }

    fn ready(&mut self) {
        let wheel = self.to_gd();

        wheel.signals()
            .wheel_end_spin()
            .connect_other(&*self.chosen_item, ChoiceLabel::on_choice);
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}