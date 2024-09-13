use std::io;

fn main() {
    'outer: loop {
        println!("To change from Celsius to Fahrenheit, press 1.");
        println!("To change from Fahrenheit to Celsius, press 2");

    //Get input and conver to iteger
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command: u32 = match command.trim().parse(){
        Ok(num)=> {
            //If input is invalid, let users try again
            if num != 1 && num != 2 {
                continue;
            }
            num
        },
        Err(_) => continue,
        };
        loop {
            println!("Input the temperature.");

            //Get input and convert to float
            let mut temperature = String::new();
            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line");

            let temperature: f32 = match temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            //Calculate converted temperature
            if command == 1 {
                println!("{} C = {} F", temperature, temperature*1.8 +32.0);
            } else {
                println!("{} F = {} C", temperature, (temperature - 32.0) / 1.8);
            }
            break 'outer;
        }
    }
}