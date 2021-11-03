


use std::borrow::Borrow;

use tools::Address;

fn print_number(num: &u32) {
    dbg!(num);
    dbg!(num.address());
}

#[derive(Default, Debug)]
struct Sok(String, u32);

impl Sok {}

fn main() {
    let spesen: Sok = Default::default();

    dbg!(spesen);

    let number: u32 = 5544;

    dbg!(number.borrow().address());

    print_number(&number);

    let sudo = sudo::check();

    dbg!(sudo);
}
