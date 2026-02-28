use godot::classes::ThemeDb;
use godot::global::HorizontalAlignment;
use godot::meta::ref_to_arg;
use godot::prelude::*;
use godot::prelude::Node2D;
use godot::prelude::INode2D;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base=Node2D, tool)]
struct Wheel {
    time: f32,
    #[export]
    angular_speed: f64,
    mouth_width: f32,
    #[export]
    max_width: f32,
    mouth: Option<PackedArray<Vector2>>,
    head: Option<PackedArray<Vector2>>,
    base: Base<Node2D>
}

const COORDS_HEAD: [[f64; 2]; 22] = [
    [ 22.952, 83.271 ],  [ 28.385, 98.623 ],
    [ 53.168, 107.647 ], [ 72.998, 107.647 ],
    [ 99.546, 98.623 ],  [ 105.048, 83.271 ],
    [ 105.029, 55.237 ], [ 110.740, 47.082 ],
    [ 102.364, 36.104 ], [ 94.050, 40.940 ],
    [ 85.189, 34.445 ],  [ 85.963, 24.194 ],
    [ 73.507, 19.930 ],  [ 68.883, 28.936 ],
    [ 59.118, 28.936 ],  [ 54.494, 19.930 ],
    [ 42.039, 24.194 ],  [ 42.814, 34.445 ],
    [ 33.951, 40.940 ],  [ 25.637, 36.104 ],
    [ 17.262, 47.082 ],  [ 22.973, 55.237 ]
];

const COORDS_MOUTH: [[f64;2]; 10] = [
    [ 22.817, 81.100 ], [ 38.522, 82.740 ],
	[ 39.001, 90.887 ], [ 54.465, 92.204 ],
	[ 55.641, 84.260 ], [ 72.418, 84.177 ],
	[ 73.629, 92.158 ], [ 88.895, 90.923 ],
	[ 89.556, 82.673 ], [ 105.005, 81.100 ]
];

#[godot_api]
impl INode2D for Wheel {

    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            time: 0.0,
            angular_speed: std::f64::consts::PI,
            mouth_width: 4.4,
            max_width: 7.0,
            mouth: Some(float_array_to_vector2_array(&COORDS_HEAD)),
            head: Some(float_array_to_vector2_array(&COORDS_MOUTH)),
            base,
        }
    }

    fn ready(&mut self) {
        self.head = Some(float_array_to_vector2_array(&COORDS_HEAD));
        self.mouth = Some(float_array_to_vector2_array(&COORDS_MOUTH));
        self.base_mut().set_rotation(0.0);
        self.base_mut().set_position(Vector2{x: 60.0, y: 60.0});
    }

    fn draw(&mut self) {
        self.base_mut().draw_set_transform(Vector2 { x: -60.0, y: -60.0 });

        let godot_blue = Color::from_string("478cbf").expect("hardcoded color");
        let white = Color::WHITE;
        let grey = Color::from_string("414042").expect("hardcoded color");

        let default_font = ThemeDb::singleton().get_fallback_font();

        let head = self.head.as_ref().unwrap().clone();
        let mouth = self.mouth.as_ref().unwrap().clone();
        let width = self.mouth_width;

        self.base_mut().draw_colored_polygon(&head, godot_blue);
        self.base_mut().draw_polyline_ex(&mouth, white)
            .width(width)
            .done();

        //Four circles for the 2 eyes: 2 white, 2 grey.
        self.base_mut().draw_circle(Vector2{x: 42.479, y: 65.4825}, 9.3905, white);
        self.base_mut().draw_circle(Vector2{x: 85.524, y: 65.4825}, 9.3905, white);
        self.base_mut().draw_circle(Vector2{x: 43.423, y: 65.92}, 6.246, grey);
        self.base_mut().draw_circle(Vector2{x: 84.626, y: 66.008}, 6.246, grey);

        // draw a short but thick vertical line for the nose
        self.base_mut().draw_line_ex(Vector2 { x: 64.273, y: 60.564 }, Vector2 { x: 64.273, y: 74.349 }, white)
            .width(5.8)
            .done();

        // GODOT text below the logo
        self.base_mut().draw_string_ex(ref_to_arg(&default_font), Vector2 { x: 20.0, y: 130.0 }, "GODOT")
            .alignment(HorizontalAlignment::CENTER)
            .font_size(22)
            .width(90.0)
            .done();        
    }
    
    fn process(&mut self, delta: f64) {
        //activate_rotation(self, delta);
        self.time += delta as f32;
        self.mouth_width = f32::abs(f32::sin(self.time) * self.max_width);
        self.base_mut().queue_redraw();
    }
}

fn float_array_to_vector2_array(coords: &[[f64;2]]) -> PackedVector2Array {
    let mut array = PackedVector2Array::new();
    for v in coords {
        array.extend_one(Vector2::new(v[0] as real, v[1] as real));
    }

    return array;
}

fn activate_rotation(wheel: &mut Wheel, delta: f64) {
    let radians = (wheel.angular_speed * delta) as f32;
    let rotation = wheel.base().get_rotation();
    wheel.base_mut().set_rotation(rotation - radians);
}