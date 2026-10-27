use std::rc::Rc;

trait DrawableWithGenericFn {
    fn draw_with_generic<T: std::fmt::Debug>(&self, color: T);
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}
impl DrawableWithGenericFn for Circle {
    fn draw_with_generic<T: std::fmt::Debug>(&self, color: T) {
        println!(
            "Drawing a circle at ({}, {}), having radius {}, with color {:?}",
            self.x, self.y, self.radius, color
        );
    }

    // vtable of DrawableWithGenericFn for Circle (which method to point to.
    // draw_with_generic_str or draw_with_generic_tuple)
    // fn draw_with_generic_str<T: std::fmt::Debug>(&self, color: T) {
    //     println!(
    //         "Drawing a circle at ({}, {}), having radius {}, with color {:?}",
    //         self.x, self.y, self.radius, color
    //     );
    // }
    // fn draw_with_generic_tuple<T: std::fmt::Debug>(&self, color: T) {
    //     println!(
    //         "Drawing a circle at ({}, {}), having radius {}, with color {:?}",
    //         self.x, self.y, self.radius, color
    //     );
    // }
}

trait GenericDrawable<T: std::fmt::Debug> {
    fn draw(&self, color: T);
}
// impl<T: std::fmt::Debug> GenericDrawable<T> for Circle {
//     fn draw(&self, color: T) {
//         println!(
//             "Drawing a circle at ({}, {}), having raduis {} with color {:?}",
//             self.x, self.y, self.radius, color
//         );
//     }
// }

impl GenericDrawable<&str> for Circle {
    fn draw(&self, color: &str) {
        println!(
            "Drawing a circle at ({}, {}), having raduis {} with color {:?}",
            self.x, self.y, self.radius, color
        );
    }
}

impl GenericDrawable<(i32, i32, i32)> for Circle {
    fn draw(&self, color: (i32, i32, i32)) {
        println!(
            "Drawing a circle at ({}, {}), having raduis {} with color {:?}",
            self.x, self.y, self.radius, color
        );
    }
}

impl GenericDrawable<String> for Circle {
    fn draw(&self, color: String) {
        println!(
            "Drawing a circle at ({}, {}), having raduis {} with color {:?}",
            self.x, self.y, self.radius, color
        );
    }
}

// vtable 1: GenericDrawable<&str> for Circle
// vtable 2: GenericDrawable<(i32,i32,i32)> for Circle

// fn draw<T: std::fmt::Debug>(obj: &dyn DrawableWithGenericFn, color: T) {
//     obj.draw_with_generic(color);
// }

// fn draw<T: std::fmt::Debug>(obj: &dyn DrawableWithGenericFn, color: T) {
//     // here we need correction
//     obj.draw_with_generic(color);
// }

fn draw_with_rc<T: std::fmt::Debug, O: DrawableWithGenericFn>(obj: &O, color: T) {
    // here we need correction
    obj.draw_with_generic(color);
}

// here we need correction
// The following is not mention and needs to recorded again (bound on T)
// fn draw_str(obj: &dyn DrawableWithGenericFn, color: T) {
//     obj.draw_with_generic(color);
// }

// here we  need correction
// fn draw_(i32,i32,i32)(obj: &dyn DrawableWithGenericFn, color: T) {
// obj.draw_with_generic(color); }

fn draw<T: std::fmt::Debug>(obj: &dyn GenericDrawable<T>, color: T) {
    obj.draw(color);
}

// fn draw_str(obj: &dyn GenericDrawable<&str>, color: &str) {
//     obj.draw(color);
// }

// fn draw_tuple<T: std::fmt::Debug>(obj: &dyn GenericDrawable<T>, color: T) {
//     obj.draw(color);
// }

fn main() {
    let circle_1 = Circle {
        x: 0.0,
        y: 0.0,
        radius: 5.0,
    };

    let circle_2 = Circle {
        x: 10.0,
        y: 10.0,
        radius: 2.0,
    };
    draw(&circle_1, "red");
    draw(&circle_1, "red".to_string());
    draw(&circle_2, (255, 0, 0));
}
