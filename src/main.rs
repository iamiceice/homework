use crate::homework41::hw41;
use crate::homework42::hw42;
use crate::homework43::hw43;

macro_rules! minus {
    ($a:expr,$b:expr) => {$a-$b};
}
mod garden;
mod homework41;
mod homework42;
mod homework43;

fn main() {
    garden::gfn1();
    hw41();
    hw42();
    hw43();
    let x=minus!(3,2);
    println!("{x}");
}
