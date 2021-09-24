use std::borrow::Borrow;

use tools::Address;

fn print_number(num: &u32) {
    dbg!(num);
    dbg!(num.address());
}

fn main() {
    let number: u32 = 5544;

    dbg!(number.borrow().address());

    print_number(&number);
}
