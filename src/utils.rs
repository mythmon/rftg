extern crate rand;

use std::{io, str, ops, fmt};
use std::iter::FromIterator;
use std::io::Write;

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

pub fn select<'a, T>(source: &'a Vec<T>) -> T
    where T: fmt::Display + Clone,
{
    for (i, option) in source.iter().enumerate() {
        println!("    {}) {}", i + 1, option);
    }
    let choice = get_num(1..(source.len() + 1)) - 1;
    (&source[choice]).clone()
}

pub fn select_optional<'a, T>(source: &'a Vec<T>) -> Option<T>
    where T: fmt::Display + Clone,
{
    for (i, option) in source.iter().enumerate() {
        println!("    {}) {}", i + 1, option);
    }
    println!("    0) None");

    let choice = get_num(0..(source.len() + 1));

    if choice > 0 {
        Some((&source[choice - 1]).clone())
    } else {
        None
    }
}

pub fn select_many<'a, T>(source: &'a Vec<T>, count: usize) -> Vec<T>
    where T: fmt::Display + Clone,
{
    if count > source.len() {
        panic!(format!("Can't satisfy count in select_many! Need {}, but only have {}.", count, source.len()));
    }

    for (i, option) in source.iter().enumerate() {
        println!("    {}) {}", i + 1, option);
    }

    let mut chosen = vec![];
    let mut available_numbers = Vec::from_iter(1..(source.len() + 1));

    while chosen.len() < count {
        let left = count - chosen.len();
        write!(&mut io::stdout(), "({} left) ", left).ok().expect("Could not write to stdout!");
        io::stdout().flush().ok().expect("Could not flush stdout!");

        let to_keep = get_num(&available_numbers) - 1;
        available_numbers.retain(|n| { *n != to_keep });
        chosen.push(to_keep);
    }

    source.iter()
        .enumerate()
        .filter(|&(index, _)| { chosen.contains(&index) })
        .map(|(_, item)| { item.clone() })
        .collect()
}

pub trait Variants {
    fn variants() -> Vec<Self>;
}
