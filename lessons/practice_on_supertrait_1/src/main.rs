trait Drawable {
    fn draw(&self);
}

trait AnimatedDrawable: Drawable {
    fn animate(&self);
}

struct Circle;

impl AnimatedDrawable for Circle {
    fn animate(&self) {
        println!("Animating a circle");
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Draw a circle");
    }
}

fn main() {
    let circle = Circle;
    circle.draw();
    circle.animate();
}
