use std::io;
fn main() {
    println!("\nWelcome to my program of converting mph to kph!!!!\n--------------------------------------------------\n\nWhat would you like to convert to kp/h from mp/h? >");

    let mut mph_input = String::new();

    io::stdin()
        .read_line(&mut mph_input)
        .expect("Failed to read line");

    let mph_input: f64 = mph_input
        .trim()
        .parse::<f64>()
        .expect("Failed to read line");

    let formula = mph_input * 1.61;

    println!("{mph_input} mp/h is {formula} kp/h\nThank you for using my program, and If you find issues, email me at kigangadarell@gmail.com");
}
