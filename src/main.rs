#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::{port::{self, mode::Output}, hal::port::PB5};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut morse = Morse::new(pins.d13.into_output());

    let input = "tiffany";
    loop {
        morse.blink_morse(input);
        arduino_hal::delay_ms(2000);
    }
}

struct Morse {
    pin: port::Pin<Output, PB5>,
}

impl Morse {
    fn new(pin: port::Pin<Output, PB5>) -> Morse {
        Morse {
            pin
        }
    }

    fn blink_morse(&mut self, input: &str) {
        input.bytes().for_each(|c| {
            match c {
                b'a' => {
                    self.dot();
                    self.dash();
                }
                b'b' => {
                    self.dash();
                    self.dot();
                    self.dot();
                    self.dot();
                }
                b'c' => {
                    self.dash();
                    self.dot();
                    self.dash();
                    self.dot();
                }
                b'd' => {
                    self.dash();
                    self.dot();
                    self.dot();
                }
                b'e' => {
                    self.dot();
                }
                b'f' => {
                    self.dot();
                    self.dot();
                    self.dash();
                    self.dot();
                }
                b'g' => {
                    self.dash();
                    self.dash();
                    self.dot();
                }
                b'h' => {
                    self.dot();
                    self.dot();
                    self.dot();
                    self.dot();
                }
                b'i' => {
                    self.dot();
                    self.dot();
                }
                b'j' => {
                    self.dot();
                    self.dash();
                    self.dash();
                    self.dash();
                }
                b'k' => {
                    self.dash();
                    self.dot();
                    self.dash();
                }
                b'l' => {
                    self.dot();
                    self.dash();
                    self.dot();
                    self.dot();
                }
                b'm' => {
                    self.dash();
                    self.dash();
                }
                b'n' => {
                    self.dash();
                    self.dot();
                }
                b'o' => {
                    self.dash();
                    self.dash();
                    self.dash();
                }
                b'p' => {
                    self.dot();
                    self.dash();
                    self.dash();
                    self.dot();
                }
                b'q' => {
                    self.dash();
                    self.dash();
                    self.dot();
                    self.dash();
                }
                b'r' => {
                    self.dot();
                    self.dash();
                    self.dot();
                }
                b's' => {
                    self.dot();
                    self.dot();
                    self.dot();
                }
                b't' => {
                    self.dash();
                }
                b'u' => {
                    self.dot();
                    self.dot();
                    self.dash();
                }
                b'v' => {
                    self.dot();
                    self.dot();
                    self.dot();
                    self.dash();
                }
                b'w' => {
                    self.dot();
                    self.dash();
                    self.dash();
                }
                b'x' => {
                    self.dash();
                    self.dot();
                    self.dot();
                    self.dash();
                }
                b'y' => {
                    self.dash();
                    self.dot();
                    self.dash();
                    self.dash();
                }
                b'z' => {
                    self.dash();
                    self.dash();
                    self.dot();
                    self.dot();
                }
                b'1' => {
                    self.dot();
                    self.dash();
                    self.dash();
                    self.dash();
                    self.dash();
                }
                b'2' => {
                    self.dot();
                    self.dot();
                    self.dash();
                    self.dash();
                    self.dash();
                }
                b'3' => {
                    self.dot();
                    self.dot();
                    self.dot();
                    self.dash();
                    self.dash();
                }
                b'4' => {
                    self.dot();
                    self.dot();
                    self.dot();
                    self.dot();
                    self.dash();
                }
                b'5' => {
                    self.dot();
                    self.dot();
                    self.dot();
                    self.dot();
                    self.dot();
                }
                b'6' => {
                    self.dash();
                    self.dot();
                    self.dot();
                    self.dot();
                    self.dot();
                }
                b'7' => {
                    self.dash();
                    self.dash();
                    self.dot();
                    self.dot();
                    self.dot();
                }
                b'8' => {
                    self.dash();
                    self.dash();
                    self.dash();
                    self.dot();
                    self.dot();
                }
                b'9' => {
                    self.dash();
                    self.dash();
                    self.dash();
                    self.dash();
                    self.dot();
                }
                b'0' => {
                    self.dash();
                    self.dash();
                    self.dash();
                    self.dash();
                    self.dash();
                }
                _ => {}
            }
            arduino_hal::delay_ms(500);
        });
    }

    fn dot(&mut self) {
        self.pin.toggle();
        arduino_hal::delay_ms(200);
        self.pin.toggle();
        arduino_hal::delay_ms(200);
    }

    fn dash(&mut self) {
        self.pin.toggle();
        arduino_hal::delay_ms(500);
        self.pin.toggle();
        arduino_hal::delay_ms(200);
    }
}
