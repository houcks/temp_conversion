fn main() {
    loop {
        let mut user_temp = String::new();
        let mut conversion_type = String::new();
        println!("Enter a temperature to convert.");

        std::io::stdin()
            .read_line(&mut user_temp)
            .expect("Failed to read line");
        let user_temp: f32 = match user_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Would you like to convert from Farenheit (f) or celsius (c)?");
        std::io::stdin()
            .read_line(&mut conversion_type)
            .expect("Failed to read line.");
        let conversion_type = conversion_type.trim();

        match conversion_type {
            "f" => println!("{}", f_to_c(user_temp)),
            "c" => println!("{}", c_to_f(user_temp)),
            _ => println!("Enter valid conversion type."),
        }
    }
}

fn f_to_c(user_temp: f32) -> f32 {
    (user_temp - 32.0) * (5.0 / 9.0)
}

fn c_to_f(user_temp: f32) -> f32 {
    user_temp * (9.0 / 5.0) + 32.0
}
