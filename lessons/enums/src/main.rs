//enums: multiple variants of a type
//enums versus structs:
//struct fields have types
//enum variants have no types

// enum WeekDays {
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
//     Sunday,
// }

// fn main() {
//     let mut day = "Sunday";

//     let week_day = vec![
//         "Monday",
//         "Tuesday",
//         "Wednesday",
//         "Thursday",
//         "Friday",
//         "Saturday",
//         "Sunday",
//     ];

//     let day = week_day[6];

//     let day = WeekDays::Sunday;
// }

enum TravelType {
    Car(f32),
    Train(f32),
    Airplane(f32),
}

impl TravelType {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            TravelType::Car(miles) => miles * 2.0,
            TravelType::Train(miles) => miles * 3.0,
            TravelType::Airplane(miles) => miles * 5.0,
        };
        allowance
    }
}

fn main() {
    let participant = TravelType::Car(60.0);

    println!(
        "Allowance of prticipant: {}",
        participant.travel_allowance()
    )
}
