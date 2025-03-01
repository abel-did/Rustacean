fn main() {
    let apples  : i32 = 50;
    let oranges : i32 = 14 + 6;
    let _fruits  : i32 = apples + oranges;

    println!("This year, my garden has {} apples and {} oranges", 
             apples, 
             oranges
            );
    println!("This year, my garden has {0} apples and {1} oranges. I can't believe I have {1} oranges.",
             apples, 
             oranges
            );
}
