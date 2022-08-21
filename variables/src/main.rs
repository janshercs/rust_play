fn main() {
    // Consts
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The number of seconds in 3 hours is: {THREE_HOURS_IN_SECONDS}");
    
    // Mutable variables
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // Shadowing and scope
    let y = 5;
    let y = y + 1; // re-declaring y again; called shadowing!
                   // Shadowing is very different from making a variable mutable! Shadowing is like
                   // redeclaring a particular variable
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y in the outer scope is : {y}");

    let guess: i32 = "-52".parse().expect("Not a number!");
    println!("{guess}");
    
    // Char types are like rune types in Go, representing 1 codepoint in UTF-8
    let c = 'z';
    let z: char = '😻'; 

    println!("normal char: {c}, emoji char works too: {z}");
    
    // tuples (again! yay!) just like python

    let tup: (i32, f64, u8) = (-20, 3.2, 255); // you may declare the types when assigning it
    let undeclared_tup = (500, 6.4, 1); // Or just leave it to the compiler!

    let (a, b, c) = undeclared_tup; // Unpacking tuples like this

    println!("Unpacked variables: {a}, {b}, {c}");

    let tup_zero =  tup.0; // accessing elements in tuple
    let tup_one =  tup.1;
    println!("Accessing values with tuple index: {tup_zero}, {tup_one}");


    // Arrays: Brackets again and no changing of array sizes!
    
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // can be initiated the old fashion way
    let a = [3; 5]; // is the same as [3, 3, 3, 3, 3]

    let zeroth = arr[0];
    let first = arr[1];
    println!("Accessing arrays with index: {}, {}", zeroth, first);

    let impossible = a[10]; // Out of index errors are only found out during runtime!
                            // At least it checks the array length and protects against invalid
                            // memory access!

    println!("{impossible}");
}
