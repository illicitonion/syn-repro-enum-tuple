extern crate syn_repro_enum_tuple;

use syn_repro_enum_tuple::UsingSyn;

#[derive(Debug, UsingSyn)]
#[repr(u8)]
enum Number {
    Zero = (0, 1).0,
}

fn main() {
    println!("{:?}", Number::Zero);
}
