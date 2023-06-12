// Syntaxing
// TypeParamBounds     |    TypeParamBound  |   TraitBound   |   LifetimeBound

// TypeParamBound
// // A generic function that accepts a type `T` implementing the `Display` trait
fn print_and_return<T: std::fmt::Display>(value: T) -> T {
    println!("Value: {}", value);
    value
}

// TypeParamBounds
// A generic function that accepts two values of different types
// The types must implement both the same `std::fmt::Display` and `std::cmp::PartialOrd` traits
fn compare_and_print<T: std::fmt::Display + std::cmp::PartialOrd>(value1: T, value2: T) {
    if value1 > value2 {
        println!("{} is greater than {}", value1, value2);
    } else if value1 < value2 {
        println!("{} is less than {}", value1, value2);
    } else {
        println!("{} is equal to {}", value1, value2);
    }
}

// TraitBound
// A generic function that accepts a value of a type implementing the `Printable` trait
fn print_value<T: Printable>(value: T) {
    value.print();
}
// Creates a struct that holds a generic value of type T
struct Container<T> {
    value: T,
}
// Create a trait that holds a generic method
pub trait Printable {
    fn print(&self);
}
impl Printable for i32 {
    fn print(&self) {
        println!("Print a i32 type: {}", self);
    }
}

impl Printable for str {
    fn print(&self) {
        println!("Print a str type: {}", self);
    }
}

// LifetimeBound
// from rustbook: https://doc.rust-lang.org/reference/trait-bounds.html
// where a outlives b
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32)
where
    'a: 'b,
{
    y = x; // &'a i32 is a subtype of &'b i32 because 'a: 'b
    let r: &'b &'a i32 = &&0; // &'b &'a i32 is well formed because 'a: 'b
    println!("assigined lifetime value: {}", r);
}

fn main() {
    println!("Hello, world!");
    // TypeParamBound
    _ = print_and_return(1);
    _ = print_and_return("helo");
    // TypeParamBounds
    compare_and_print(3, 5);
    // TraitBound
    let int_container = Container { value: 3 };
    let str_container = Container { value: "hello" };
    int_container.value.print();
    str_container.value.print();
    // direct call
    print_value(32);
    // LifetimeBound, not able to showcase properly yet...
    let lt_a = 1;
    let lt_b = 2;
    f(&lt_a, &lt_b);
}
