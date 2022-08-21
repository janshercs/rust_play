fn main() {
    // Statements and Expressions (again!)
    // Statement returns nothing and is just that; a statement!
    // Expressions always evaluate to something

    let x = 5; // classic statement
    println!("x is: {x}"); // another statement

    let y = {
        let x = 3;
        x + 1 // not having a semicolon ; makes it a return value!
    }; // within this braces block is an expression and it returns the final value!
      
    println!("y is: {y}");

    let z = {
        let _ = 3;
        20
    }; // within this braces block is an expression and it returns the final value!
      
    println!("z is: {z}");

    print_measurement(z, "randoms");
    print_measurement(get_some(), "randoms");
}

fn print_measurement(value: i32, unit: &str){ // how you add parameters to a function
    println!("The measurement is {value} {unit}");
}

fn get_some() -> i32{ // how you add return values
    5
}
