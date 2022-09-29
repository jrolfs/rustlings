// vec2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute the command `rustlings hint vec2` if you need
// hints.

fn vec_loop(v: Vec<i32>) -> Vec<i32> {
    // return v.iter().map(|x| x * 2).collect();

    let mut loopd = Vec::new();

    for i in v.iter() {
        loopd.push(i * 2);
    }

    loopd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let range = (1..);
        let v: Vec<i32> = range.filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        let expected = v.iter().map(|x| x * 2).collect::<Vec<i32>>();

        assert_eq!(ans, expected);
    }
}
