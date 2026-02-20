use godot::prelude::*;
use godot::prelude::Node2D;
use godot::prelude::INode2D;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base=Node2D, tool)]
struct Wheel {
    speed: f64,
    angular_speed: f64,

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

#[godot_api]
impl INode2D for Wheel {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn draw(&mut self) {
        let points = float_array_to_vector2_array(&COORDS_HEAD);
        let godot_blue = Color::from_string("478cbf");
    self.base_mut().draw_colored_polygon(&points, godot_blue.expect("hardcoded color"));
    }
}

fn float_array_to_vector2_array(coords: &[[f64;2]]) -> PackedVector2Array {
    let mut array = PackedVector2Array::new();
    for v in coords {
        array.extend_one(Vector2::new(v[0] as real, v[1] as real));
    }

    return array;
}