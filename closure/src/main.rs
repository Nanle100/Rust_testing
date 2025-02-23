// closures are also known as anonymous functions.

fn main(){

// closure syntax: it is like a function:
// || {code}

// example_1
let closure = || "Hello, World";

println!("{:?}", closure());


// example_2
let add = |a, b| a + b;

println!("{}", add(3, 4));


// example_3
let hello: String = String::from("Hello, ");
let world: String = String::from("World");

let add2 = |a, b| a + b;
println!("{}", add2(hello, &world));


// Capturing variables with closures
// there are 3 different ways to capture variable with closures:
// 1. Borrowing a variable imutably
// 2. Borrowing a variable mutably
// 3. Taking ownership of a variable.


// example_4 capturing by borrowing
let x = 300;

let print_x = || println!("{}", x);

print_x();


// example_5 - capturing by mutable borrowing

let mut y = 200;
let mut _printer = || {
    y += 1;
    println!("{}", y);
};



// example_6 -- capturing by taking ownership
let p: String = String::from("Hello");

let print_p = move || {
    println!("{}", p);

    drop(p);
};

print_p();



// closure traits
// fn: captures  variables by mutable reference (&mut T)
// fnOnce: captures variables by value (T)


// example_7 - closures as function parameter
let double = |x| x * 2;
apply(double); 



}


// example_7 - closures as function parameter
fn apply<F>(f: F) where F: Fn(i32) -> i32 {
    println!("{}", f(10)); // 20
}


/*



*/