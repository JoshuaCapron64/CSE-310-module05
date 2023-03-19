/*
INTRODUCTION:

Hello, and welcome to my tutorial on the Rust programming language. I
will be demonstrating various aspects of programming as they are
executed within Rust, including mutable and immutable variables, loops,
conditionals, expressions, and functions. While Cargo is an important
aspect of creating and manipulating directories and files that use Rust,
I have chosen to manually create my files and directories within my
own folders in Windows File Explorer. 
*/

/* VARIABLES AND EXPRESSIONS:

Unlike most other programs, Rust categorizes its variables as either
mutable or immutable. By default, variables in Rust are immutable,
meaning that once a value is assigned to that variable, it cannot be
changed or altered in any way. Mutable variables do allow their assigned
values to change. Rust is also capable of most of the same mathematical
expressions as other languages. */

//notice how the variables passed into the funciton must have their data
//type specified, along with whether they are mutable.
fn variables_and_expressions(x: i32, mut y: i32, mut z: f64)
{
    println!("VARIABLES AND EXPRESSIONS: ");
//trying to change the value assigned to immutable variables will result
//in errors.
    println!("Immutable variable x = {x}");
    println!("\nMutable variable y =   {y}");
    y = y + x;
    println!("Now, y =               {y}");
//other than that, rust utilizes all the same data types other languages
//use, such as integers, booleans, floting-points, and even strings,
//tuples and arrays

    println!("\nFloating-point z =   {z}");
    z = z - 0.14;
    println!("Now, z =                {z}");
//notice how this unassigned number must be given a decimal value to be
//recognized as a floating-point variable, and thus be used with another
//floating-point variable.
    z = z / 2.0;
    println!("Now, z =              {z}");
    z = z * 10.0;
    println!("Now, z =               {z}");
    z = z % 4.0;
    println!("Now, z =                {z}");
//also notice how the output won't display whole floating-point values
//with a .0 at the end like other languages. 
}


/* CONDITIONALS AND LOOPS:

Conditionals work in Rust very similarly to how other languages utilize
them, even with very similar syntax as used by C and its variants. Its
use of loops, however is quite unique. The "loop{}" keyword declares an
infinite loop that will run whatever is within the square brackets over
and over. One can use additional keywords that will alter the loop's
functionality. For instance, "break" will cause the loop to end. It's
best to ensure that this is placed within a separate condition so that
the loop only ends once the condition is met. */

fn conditionals_and_loops(mut value: u32, mut exponent: u32)
{
    println!("\nCONDITIONALS AND LOOPS: ")
    println!("\nBeginning of loop:");
    loop
    {
        print!("{value}  ");
        if value == 8
        {
            println!("\nHey look, the minimum bitsize for variables!");
        }
        if value == 64
        {
            println!("\nHey look, the bit technology used by my favorite classic gaming console!");
        }
        if value >= 512
        {
            println!("\nOkay, let's not destroy the computer. We'll call it here.");
            break;
        }
        value = value * 2;
    }

//the same loop as before, but done with Rust's exponent function
    println!("\nSyke! We're gonna continue instead!:");
    loop
    {
        value = 2;
        let new_value = value.pow(exponent);
        print!("{new_value}  ");
        if exponent >= 20
        {
            println!("\nOkay now we're done. For real this time.");
            break;
        }
        exponent = exponent + 1;
    }
//overflows are possible, and they will cause the program to error out,
//so make sure that the outputs can be handled by the memory of the
//required variable type (usually i32 or u32 for integers).
}

//the main() function will bring everything together by declaring the
//necessary variables and passing them into each called function.
fn main()
{
    let ten_i = 10;
    let sixty_four = 64;
    let three_point_one_four = 3.14;
    variables_and_expressions(ten_i, sixty_four, three_point_one_four);
    let ten_u = 10;
    let two = 2;
    conditionals_and_loops(two, ten_u);
    println!("\nEnd of program.");
}