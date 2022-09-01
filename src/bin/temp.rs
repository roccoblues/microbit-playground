#![no_main]
#![no_std]

use microbit::hal::temp;
use microbit_playground as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();
    let mut t = temp::Temp::new(board.TEMP);

    let data = t.measure();
    defmt::println!("Temperature: {}", defmt::Display2Format(&data));

    microbit_playground::exit()
}
