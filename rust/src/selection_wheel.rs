use godot::builtin::{Array, Color, GString, StringName, Vector2};
use godot::classes::control::LayoutPreset;
use godot::classes::{Control, IControl, Label, Node2D, ThemeDb};
use godot::global::HorizontalAlignment;
use godot::meta::ref_to_arg;
use godot::obj::{Base, Gd, Singleton, WithBaseField};
use godot::prelude::{godot_api, GodotClass, Node};
use std::f32::consts::TAU;

const SPRITE_SIZE: Vector2 = Vector2{x: 32.0, y: 32.0};

#[derive(GodotClass)]
#[class(base=Control, tool)]
pub struct Wheel {
    base: Base<Control>,

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
    options: Array<GString>
}

#[godot_api]
impl IControl for Wheel {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            bg_color: Color::BLACK,
            line_color: Color::WHITE,
            line_width: 4,
            outer_radius: 256,
            inner_radius: 64,
            options: Array::new(),
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
        self.setup_labels();
        self.base_mut().queue_redraw();
    }
}

impl Wheel {
    fn setup_labels(&mut self) {
        let outer_radius = self.outer_radius as f32;
        let bg_color = self.bg_color;
        let inner_radius = self.inner_radius as f32;
        let default_font = ThemeDb::singleton().get_fallback_font();

        let names = self.options.clone();

        for child in self.base_mut().get_children().iter_shared() {

            if (child.get_name() != StringName::from("Choice"))
            {
                Gd::free(child);
            }
        }

        for (i, name) in names.iter_shared().enumerate() {
            let label = self.base_mut().get_node_as::<Label>("Choice");
            let mut copy_label  = label.duplicate().unwrap().cast::<Label>();
            copy_label.set_text(&name);

            let start_rads = i as f32 / self.options.len() as f32 * TAU;
            let end_rads = (i + 1) as f32 / self.options.len() as f32 * TAU;
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