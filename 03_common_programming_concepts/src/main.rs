fn main() {
    let mut x = 5;
    println!("the value is x is: {}", x);
    x = 6;
    println!("the value of x is:{}", x);

    // Shadowning
    let y = 32;
    println!("the value of y is: {}", y);
    let y = "this is an string";
    println!("the value of y is: {}", y);

    // Integers

    let a = 32; // integers have signed and unsigned integers like i8, i16, i32, i64, i128 and for unsigned integers are u8, u16,   u32, u64, u128

    //floating numbers

    let floating = 3.4;

    // Booleans
    let t = true;
    let f = false;

    // Characters

    let c = 'Z';

    // Compound types

    //tuples
    let tuples = ("this is an string", "value", 1000);

    let (first, second, third) = tuples;

    // One way to access tuples
    println!("first value: {}", first);
    println!("second value: {}", second);
    println!("third value: {}", third);

    // Second way to access values
    println!("first value: {}", tuples.0);
    println!("second value: {}", tuples.1);
    println!("third value: {}", tuples.2);

    // Arrays

    let arr = [200, 404, 500];
    let not_found = arr[1];
    let arr_of_size_8 = [0; 8];
    println!("{:?}", arr_of_size_8);


    let even = is_even(2);

    if even {
        println!("The number is even");
    }else{
        println!("The number is not even");

    }
    my_function();
    let sum = add(32, 8);
    println!("the sum is {}",sum);
}

// Functions

fn is_even(num: u64) -> bool {
    return num % 2 == 0;
}

fn my_function(){
    println!("Another functions")
}

fn add (x:u32,y:u32) -> u32{
println!("The vale of x is {}",x);
println!("The vale of y is {}",y);
   let sum = x+y;
   return sum;
}
