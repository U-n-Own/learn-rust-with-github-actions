//Variables book chapters


const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;



fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("{} second in an hour", THREE_HOURS_IN_SECONDS);

    shadowing_1();

    println!("Printing the first 10 fibonacci number sequence");


    for number_in_sequence in 0..10 {
    println!("Number {number_in_sequence}: {}", fibo(number_in_sequence))
    }

    //Tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);
}


fn shadowing_1() {

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn fibo(n : i32) -> i32 {

    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    fibo(n-1) + fibo(n-2)
}