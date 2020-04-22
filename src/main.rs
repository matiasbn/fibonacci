use std::io;

fn fibonacci(value: u32) -> String {
    let mut val = vec![1];
    for number in 1..value {
        if number == 1 {
            val.push(number);
        } else {
            let first = val[number as usize - 2];
            let second = val[number as usize - 1];
            val.push(first + second);
        }
    }
    return loop {
        let mut string = String::new();
        for member in val.iter() {
            let member_string = format!("{} ", member);
            string.push_str(&member_string);
        }
        string.pop();
        break string;
    };
}

fn main() {
    println!("Fibonacci sequence");
    println!("Enter the nth value");
    loop {
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed at read line");
        let value: u32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a number", value.trim());
                continue;
            }
        };
        break println!(
            "The Fibonacci sequence up to {}th member is [{}]",
            value,
            fibonacci(value)
        );
    }
}
