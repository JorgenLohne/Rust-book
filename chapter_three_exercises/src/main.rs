fn main() {
    println!("Fibernaci nr is {}", fib(1));
    println!("-40f in celcius is: {}", (fahrenheit_to_celsius(-40.0)));
    println!("100c in fahrenheit is: {}", (celsius_to_fahrenheit(100.0)));
    println!("sum of numbers from 0 to 100 is: {}", count_sum(100));
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    5.0 / 9.0 * (f - 32.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fib(x: u32) -> u32 {
    if x <= 0 {
        panic!("Cannot be less or equal to 0!")
    }

    if x == 1 || x == 2 {
        return 1;
    }

    fib(x - 1) + fib(x - 2)
}

fn count_sum(x: i32) -> i32 {
    let mut sum = 0;
    for i in 0..=x {
        sum += i;
    }

    sum
}
