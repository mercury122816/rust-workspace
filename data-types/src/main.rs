fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    //     let x = 5;
    //   |         -
    //   |         |
    //   |         first assignment to `x`
    //   |         help: consider making this binding mutable: `mut x`
    // 3 |     println!("The value of x is: {}", x);
    // 4 |     x = 6;
    //   |     ^^^^^ cannot assign twice to immutable variable

    // There are multiple trade-offs to consider in addition to the prevention of bugs. 
    // For example, in cases where you’re using large data structures, mutating an instance in 
    // place may be faster than copying and returning newly allocated instances. With smaller 
    // data structures, creating new instances and writing in a more functional programming 
    // style may be easier to think through, so lower performance might be a worthwhile penalty 
    // for gaining that clarity.
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // First, you aren’t allowed to use mut with constants. 
    // Constants aren’t just immutable by default—they’re always immutable.
    // You declare constants using the const keyword instead of the let keyword, and the 
    // type of the value must be annotated. 

    const MAX_POINTS: u32 = 100_000;

    // Constants are valid for the entire time a program runs, within the 
    // scope they were declared in, making them a useful choice for values in your application 
    // domain that multiple parts of the program might need to know about.

    // you can declare a new variable with the same name as a previous variable. 
    // Rustaceans say that the first variable is shadowed by the second, which means that the 
    // second variable’s value is what appears when the variable is used. We can shadow a variable 
    // by using the same variable’s name and repeating the use of the let keyword

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    // Shadowing is different from marking a variable as mut, because we’ll get a compile-time error 
    // if we accidentally try to reassign to this variable without using the let keyword. By using let, 
    // we can perform a few transformations on a value but have the variable be immutable after those 
    // transformations have been completed.

    let spaces = "   ";
    //Error: type cannot be changed.
    let spaces = spaces.len(); 

    // Keep in mind that Rust is a statically typed language, which means that 
    // it must know the types of all variables at compile time. 
    // The compiler can usually infer what type we want to use based on the value and how we use it.
}
