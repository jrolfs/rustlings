// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This becomes problematic
// for recursive types, where a value can have as part of itself another value of the same type.
// To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
// which also allows us to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a data structure
// frequently found in functional programming languages. Each item in a cons list contains two
// elements: the value of the current item and the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `unimplemented!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` for hints :)

use std::mem;

use List::{Cons, Nil};

// I AM NOT DONE

#[derive(PartialEq, Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    // TODO: implement reference iterator
    // fn iter(&self) -> ListIterator<&T> {

    // }

    fn into_iter(self) -> ListIterator<T> {
        ListIterator::new(self)
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::Nil
    }
}

pub struct ListIterator<T> {
    current: List<T>,
}

impl<T> ListIterator<T> {
    fn new(list: List<T>) -> Self {
        Self {
            current: list,
        }
    }
}

impl<T> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = mem::take(&mut self.current);

        match current {
            List::Cons(value, mut next) => {
                mem::swap(&mut self.current, next.as_mut());

                return Option::Some(value);
            }
            Nil => Option::None,
        }
    }
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List<i32> {
    Nil
}

pub fn create_non_empty_list() -> List<i32> {
    Cons(5, Box::new(Cons(4, Box::new(Nil))))
}

#[derive(Debug)]
struct NumberHolder {
    number: i32,
}

impl NumberHolder {
    fn new(number: i32) -> Self {
        Self { number }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(List::Nil, create_non_empty_list());
        assert_ne!(create_empty_list(), create_non_empty_list())
    }

    #[test]
    fn test_list_iterator() {
        assert_eq!(
            vec![5, 4],
            create_non_empty_list().into_iter().collect::<Vec<i32>>()
        )
    }

    #[test]
    fn test_list_iterator_control() {
        assert_eq!(
            vec![&5, &4],
            vec![5, 4].iter().collect::<Vec<&i32>>()
        )
    }

    #[test]
    fn test_list_into_iterator_control() {
        assert_eq!(
            vec![5, 4],
            vec![5, 4].into_iter().collect::<Vec<i32>>()
        )
    }

    #[test]
    fn test_list_iterator_no_copy() {
        assert_eq!(
            vec![5,4 ],
            Cons(
                NumberHolder::new(5),
                Box::new(Cons(NumberHolder::new(4), Box::new(Nil)))
            )
            .into_iter()
            .map(|holder| holder.number)
            .collect::<Vec<i32>>()
        )
    }
}
