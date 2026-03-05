use std::f32::consts::TAU;
use godot::builtin::{Color, PackedArray, Vector2};
use godot::classes::{Control, IControl, Node2D};
use godot::global::godot_print;
use godot::obj::{Base, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=Control, tool)]
struct Wheel {
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



    base: Base<Control>
}

#[godot_api]
impl IControl for Wheel {
    fn init(base: Base<Control>) -> Self {
        Self {
            bg_color: Color::BLACK,
            line_color: Color::WHITE,
            line_width: 4,
            outer_radius: 256,
            inner_radius: 64,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        self.base_mut().queue_redraw();
    }

    fn draw(&mut self) {
        let outer_radius = self.outer_radius as f32;
        let bg_color = self.bg_color;
        let inner_radius = self.inner_radius as f32;
        let line_color = self.line_color;
        let line_width = self.line_width as f32;


        self.base_mut().draw_circle(Vector2::ZERO, outer_radius, bg_color);
        self.base_mut()
            .draw_arc_ex(Vector2::ZERO, inner_radius, 0.0, TAU, 256, line_color)
            .width(line_width)
            .antialiased(true)
            .done();
    }
}