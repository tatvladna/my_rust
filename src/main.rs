mod basics;
mod streams;
mod data_structures;
mod generic_types;
mod ownership_borrowing;
mod text;
mod oop;

use basics::basics;
use streams::streams;
use data_structures::data_structures;
use generic_types::generic_types;
use ownership_borrowing::ownership_borrowing;
use text::text;
use oop::oop;


fn main() {
    basics();
    streams();
    data_structures();
    generic_types();
    ownership_borrowing();
    text();
    oop();
}
