use std::io;

fn main() {
    println!("Calculates a moving average. Enter 'x' to quit. Input your first value:");

    let mut average: f64 = 0.0;
    let mut count: i32 = 0;

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        if input.trim() == "x" {
            println!("Last average: {average}");
            return;
        }

        match input.trim().replace(",", ".").parse::<f64>() {
            Ok(num) => {
                count += 1;
                average = (average * (count - 1) as f64 + num) / count as f64;
                println!("Updated average: {average}");
            }
            Err(_) => println!("Invalid input. Please input a floating point number."),
        };
    }
}
