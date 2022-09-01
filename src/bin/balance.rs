#![no_main]
#![no_std]

use lsm303agr::{AccelOutputDataRate, Lsm303agr};
use microbit::{
    display::blocking::Display, hal::twim, hal::Timer, pac::twim0::frequency::FREQUENCY_A,
};
use microbit_playground as _;

const THRESHOLD: i32 = 25;
const INTERVAL: u32 = 100;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();

    let mut leds = [[0; 5]; 5];
    let mut x = 0;
    let mut y = 0;

    loop {
        leds[y][x] = 0;
        let data = sensor.accel_data().unwrap();
        if data.x > THRESHOLD && x < 4 {
            x += 1;
        } else if data.x < -THRESHOLD && x > 0 {
            x -= 1;
        }
        if data.y < -THRESHOLD && y < 4 {
            y += 1;
        } else if data.y > THRESHOLD && y > 0 {
            y -= 1;
        }
        leds[y][x] = 1;
        display.show(&mut timer, leds, INTERVAL);
    }
}
