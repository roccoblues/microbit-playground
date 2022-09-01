#![deny(unsafe_code)]
#![no_main]
#![no_std]

use lsm303agr::{AccelOutputDataRate, AccelScale, Lsm303agr};
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};

use microbit::hal::prelude::*;
use microbit::hal::timer::Timer;
use nb::Error;

use microbit_playground as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    const THRESHOLD: f32 = 0.5;

    let board = microbit::Board::take().unwrap();
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut countdown = Timer::new(board.TIMER0);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor.set_accel_scale(AccelScale::G16).unwrap();

    let mut max_g = 0.;
    let mut measuring = false;

    loop {
        while !sensor.accel_status().unwrap().xyz_new_data {}
        // x acceleration in g
        let g_x = sensor.accel_data().unwrap().x as f32 / 1000.0;

        if measuring {
            // Check the status of our countdown
            match countdown.wait() {
                // countdown isn't done yet
                Err(Error::WouldBlock) => {
                    if g_x > max_g {
                        max_g = g_x;
                    }
                }
                // countdown is done
                Ok(_) => {
                    // Report max value
                    defmt::println!("Max acceleration: {}g", max_g);

                    // Reset
                    max_g = 0.;
                    measuring = false;
                }
                // Since the nrf52 and nrf51 HAL have Void as an error type
                // this path cannot occur, as Void is an empty type
                Err(Error::Other(_)) => {
                    unreachable!()
                }
            }
        } else {
            // If acceleration goes above a threshold, we start measuring
            if g_x > THRESHOLD {
                defmt::println!("START!");

                measuring = true;
                max_g = g_x;
                // The documentation notes that the timer works at a frequency
                // of 1 Mhz, so in order to wait for 500 milliseconds we have to
                // set it to 500_000 ticks.
                countdown.start(500_000_u32);
            }
        }
    }
}
