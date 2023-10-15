#![no_std]
#![no_main]

use embedded_hal::digital::v2::{PinState, OutputPin};
use panic_halt as _;
use arduino_hal::{port::{mode::Output, Pin}, hal::port::Dynamic};

#[arduino_hal::entry]
fn main() -> ! {

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */
    let mut segment = Segments::new();
    let mut i = 0;

    loop {
        segment.display(i);
        i = (i + 1) % 10;
    }
}

struct Segments {
    pins: [Pin<Output, Dynamic>; 7],
}

impl Segments {
    fn new() -> Segments {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        let pin_arr: [Pin<Output, Dynamic>; 7] = [
            pins.d4.into_output().downgrade(),// g
            pins.d5.into_output().downgrade(), // f
            pins.d13.into_output().downgrade(), // e
            pins.d12.into_output().downgrade(), // d
            pins.d10.into_output().downgrade(), // c
            pins.d8.into_output().downgrade(), // b
            pins.d7.into_output().downgrade(), // a
        ];

        Segments {
            pins: pin_arr
        }
    }

    fn display(&mut self, i: u32) {
        let bits = Segments::map_bits(i);
        for n in 0..7 {
            let _ = self.pins[n].set_state(PinState::from(bits & 1 << n != 0));
        }
        arduino_hal::delay_ms(200);
    }

    fn map_bits(i: u32) -> u8 {
        return match i {
            0 => 0b01111110,
            1 => 0b00110000,
            2 => 0b01101101,
            3 => 0b01111001,
            4 => 0b00110011,
            5 => 0b01011011,
            6 => 0b01011111,
            7 => 0b01110000,
            8 => 0b01111111,
            9 => 0b01111011,
            10.. => 0
        }
    }
}
