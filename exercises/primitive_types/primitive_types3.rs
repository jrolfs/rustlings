// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    let a = [0; 100].iter().fold(Vec::new(), |acc: Vec<i32>, value| {
        let last = &acc.last().unwrap_or(&value);
        let next = vec![*last + 1];

        [acc, next].concat()
    });
    
    println!("{:?}", a);

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
