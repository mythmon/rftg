extern crate rand;

use std::{io, str, ops, fmt};

pub trait Contains<T> {
    fn contains(&self, needle: &T) -> bool;
}

impl<T: PartialOrd> Contains<T> for ops::Range<T> {
    fn contains(&self, needle: &T) -> bool {
        *needle >= self.start && *needle < self.end
    }
}

impl<T: PartialOrd> Contains<T> for ops::RangeFrom<T> {
    fn contains(&self, needle: &T) -> bool {
        *needle >= self.start
    }
}

impl<T: PartialOrd> Contains<T> for ops::RangeTo<T> {
    fn contains(&self, needle: &T) -> bool {
        *needle < self.end
    }
}

impl<T> Contains<T> for ops::RangeFull {
    fn contains(&self, _: &T) -> bool {
        true
    }
}

impl<'a, T: Eq> Contains<T> for &'a Vec<T> {
    fn contains(&self, needle: &T) -> bool {
        for hay in self.iter() {
            if hay == needle {
                return true;
            }
        }
        false
    }
}

pub fn get_num<T, U>(valid: U) -> T
    where T: str::FromStr + fmt::Debug + PartialOrd,
          U: Contains<T> + fmt::Debug,
{
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Error reading stdin.");

        let num: T = match input.trim().parse().ok() {
            Some(num) => num,
            None => {
                println!("That's not a number!");
                continue;
            }
        };

        if !valid.contains(&num) {
            println!("That number is not in the range {:?}!", valid);
            continue;
        }

        return num
    };
}

pub fn select<'a, T>(source: &'a Vec<T>) -> &'a T
    where T: fmt::Display + Copy
{
    for (i, option) in source.iter().enumerate() {
        println!("{}) {}", i + 1, option);
    }
    let choice = get_num(1..(source.len() + 1)) - 1;
    &source[choice]
}

pub trait Variants {
    fn variants() -> Vec<Self>;
}
