#[derive(Debug)]
struct Point {
    x: u64,
    y: u64
}

#[derive(Debug)]
struct Square {
    point: Point,
    size: u64
}

#[derive(Debug)]
struct Rectangle {
    point: Point,
    x_size: u64,
    y_size: u64
}

trait Volume {
	fn get_volume(&self) -> u64;
}

impl Volume for Square {
	fn get_volume(&self) -> u64 {
		self.size * self.size
	}
}

impl Volume for Rectangle {
	fn get_volume(&self) -> u64 {
		self.x_size * self.y_size
	}
}

#[derive(Debug)]
struct Money<T> {
    amount: T,
    currency: String
}

use std::ops::Add;
use std::fmt::{Display, Formatter, Result};

// impl<T: Add<T,Output=T>> Add for Money<T> {
impl<T> Add for Money<T> where T: Add<Output=T> {
    type Output = Money<T>;

    fn add(self,other:Money<T>) -> Money<T> {
        let added= self.amount+other.amount;
        Money{ amount:added, currency:self.currency}
    }
}

impl<T> Display for Money<T> where T: Display {
    fn fmt(&self, f:&mut Formatter) -> Result {
    	write!(f, "{} {}", self.amount, self.currency)
    }
}

fn main() {
    let whole_euros: Money<u8> = Money { amount: 42, currency: "EUR".to_string() };
    let floating_euros: Money<f32> = Money { amount: 24.312, currency: "EUR".to_string() };

    println!("Whole euros: {:?}", whole_euros);
    println!("Floating euros: {:?}", floating_euros);

    let a = Money {amount: 43, currency: "EUR".to_string()};
    let b = Money {amount: 43, currency: "EUR".to_string()};

    println!("{:?}", a+b);
}