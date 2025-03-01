/*
 * Information  :   Variable shadowing means redeclaring a variable.
 *                  The original variable is "replaced" by the new one.
 */

fn main(){
    let grams_of_protein : &str = "100.345"; // Text Shadowing
    // Conv. &str -> f64
    let grams_of_protein : f64  = 100.345;   // Float
    // Conv. f64  -> i32
    let grams_of_protein : i32  = 100;

}


