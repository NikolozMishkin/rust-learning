mod shapes {
    pub struct Circle {
        radius: f32,
    }
    impl Circle {
        pub fn new(radius: f32) -> Circle {
            Circle { radius }
        }

        pub fn new_1(radius: f32) -> Result<Circle, String> {
            if radius >= 0.0 {
                Ok(Circle { radius })
            } else {
                Err(String::from("radius should be positive"))
            }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

#[cfg(test)]
mod tests {
    use std::arch::x86_64::_mm_sha1nexte_epu32;

    use super::*;

    #[test]
    fn larger_cicle_should_contain_smaller() {
        let larger_cicle = shapes::Circle::new(5.0);
        let smaller_cicle = shapes::Circle::new(2.0);
        assert_eq!(
            larger_cicle.contains(&smaller_cicle),
            true,
            "Custom failure message"
        );

        assert_ne!(larger_cicle.contains(&smaller_cicle), false);
        assert!(larger_cicle.contains(&smaller_cicle));
    }

    #[test]
    fn smaller_cicle_should_not_contain_larger() {
        let larger_cicle = shapes::Circle::new(5.0);
        let smaller_cicle = shapes::Circle::new(2.0);
        assert_eq!(!smaller_cicle.contains(&larger_cicle), true);
    }

    #[test]
    fn should_not_create_circle() -> Result<(), String> {
        let some_circle = shapes::Circle::new_1(-1.0)?;
        Ok(())
    }
}
