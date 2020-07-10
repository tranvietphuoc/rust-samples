fn main() {
    // by default, variables in Rust is immutable
    // the immutable variables are bound to a name, you can't change the that value
    // to change the value of a variable, you can use mut keyword
    //
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x after asigned the new value is {}", x);

    // difference between variables and constants
    // like immutable variables, constants are values that are bound to a name and are not allowed
    // to change. But they are different
    // first: you aren't allowed to use mut keyword with constants.
    // the type of the value must be annotated
    // constants may be set only to a constant expression, not a function call or any other value
    // that could only be computed at runtime
    const MAX_POINTS: u32 = 100_000;

    // Shadowing
    // you can declare a new variable with the same name as a previous variable, and the new
    // variable shadows the previous variable. Rustaceans say that the first variable is shadowed
    // by the second
    let y = 5;
    let y = y + 1;
    let y = y * 3;
    println!("The value of x is {}", y);
    // Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this     //variable without using the let keyword.
    // By using let, we can perform a few transformations on a value but have the variable be immutable
    // after those transformations have been completed.
    // the other difference between mut and shadowing is that because we're effectively creating a
    // new variable when we use the let keyword again
    //
}
