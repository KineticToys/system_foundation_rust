use system_foundation_rust::{
    algorithms::convex_hull::convex_hull_algorithm::get_convex_hull,
    math::numerics::vector2d::Vector2D,
};

fn main() {
    convex_hull();
}

fn convex_hull() {
    let points = vec![
        Vector2D { x: 0_f64, y: 1_f64 },
        Vector2D {
            x: -1_f64,
            y: 0_f64,
        },
        Vector2D { x: 0_f64, y: 0_f64 },
        Vector2D { x: 1_f64, y: 0_f64 },
        Vector2D {
            x: 0_f64,
            y: -1_f64,
        },
    ];
    let rim = get_convex_hull(&points[..]);
    println!("Rim: {:?}", &rim.unwrap());
}
