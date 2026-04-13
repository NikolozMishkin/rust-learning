mod graphics {

    pub use self::display::show_area; // Re-export the 'show_area' function for easier access
    pub use self::shapes::calculate_area; // Re-export the 'calculate_area' function for easier access

    pub mod shapes {
        pub fn calculate_area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }
    }
    pub mod display {
        pub fn show_area(shape: &str, area: f64) {
            println!("The area of the {} is: {}", shape, area);
        }
    }
}

use graphics::calculate_area;
use graphics::show_area; // fix this line // fix this line
fn main() {
    let radius = 3.0;
    let area = calculate_area(radius);

    show_area("circle", area);
}
