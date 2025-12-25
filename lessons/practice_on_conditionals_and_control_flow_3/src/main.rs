fn main() {
    println!("{}", total_production(6, 5) as i32); // to round the values we use i32. just ignore for mow
    println!("{}", cars_produced_per_minutes(6, 5) as i32); // to round the values we use i32. just ignore for mow
}

fn total_production(hours: u8, speed: u8) -> f32 {
    let success_rate: f32;

    /* Your code below this line*/
    if speed >= 1 && speed <= 4 {
        success_rate = 1.0;
    } else if speed >= 5 && speed <= 8 {
        success_rate = 0.9;
    } else if speed >= 9 && speed <= 10 {
        success_rate = 0.77;
    } else {
        success_rate = 0.0;
    }
    let cars_in_hour = 221;
    let total_cars = ((speed as f32 * cars_in_hour as f32) * hours as f32) * success_rate;
    return total_cars;
}

fn cars_produced_per_minutes(hours: u8, speed: u8) -> f32 {
    let success_rate: f32;
    /* Your code below this line*/
    if speed >= 1 && speed <= 4 {
        success_rate = 1.0;
    } else if speed >= 5 && speed <= 8 {
        success_rate = 0.9;
    } else if speed >= 9 && speed <= 10 {
        success_rate = 0.77;
    } else {
        success_rate = 0.0;
    }
    let cars_in_hour = 221;
    let cars_per_minut = (cars_in_hour as f32 * speed as f32 * success_rate * hours as f32)
        / (60 as f32 * hours as f32);
    return cars_per_minut;
}
