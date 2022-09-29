// move_semantics2.rs
// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)

/**
 * 1. Make another, separate version of the data that's
 *    in `vec0` and pass that to `fill_vec` instead.
 */
// fn main() {
//     let vec0 = Vec::new();

//     let mut vec1 = fill_this_vec(vec0);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);
//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

/**
 * 2. Make `fill_vec` borrow its argument instead of taking ownership of it,
 *    and then copy the data within the function in order to return an owned
 *   `Vec<i32>`
 */
fn main() {
    let vec0 = Vec::new();
    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

/**
 * 3. Make `fill_vec` *mutably* borrow its argument (which will need to be
 *    mutable), modify it directly, then not return anything. Then you can get rid
 *    of `vec1` entirely -- note that this will change what gets printed by the
 *    first `println!`
 */
// fn main() {
//     let mut vec = Vec::new();

//     fill_this_vec(&mut vec);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);

//     vec.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec.len(), vec);
// }

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut filled = vec.clone();

    filled.push(22);
    filled.push(44);
    filled.push(66);

    filled
}

fn fill_this_vec(vec: &mut Vec<i32>) -> () {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
