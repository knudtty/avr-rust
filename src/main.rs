#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::{port::{mode::Output, Pin}, hal::port::{PB5, PB4, PB2, PB1, PB0, PD4, PD5, PD7}};

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
    e: Pin<Output, PB5>,
    d: Pin<Output, PB4>,
    c: Pin<Output, PB2>,
    dp: Pin<Output, PB1>,
    b: Pin<Output, PB0>,
    a: Pin<Output, PD7>,
    f: Pin<Output, PD5>,
    g: Pin<Output, PD4>,
}

impl Segments {
    fn new() -> Segments {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);

        Segments {
            e: pins.d13.into_output(),
            d: pins.d12.into_output(),
            c: pins.d10.into_output(),
            dp: pins.d9.into_output(),
            b: pins.d8.into_output(),
            a: pins.d7.into_output(),
            f: pins.d5.into_output(),
            g: pins.d4.into_output(),
        }
    }

    fn display(&mut self, i: u32) {
        self.clear_display();
        match i {
            0 => {
                self.a.set_high();
                self.b.set_high();
                self.c.set_high();
                self.d.set_high();
                self.e.set_high();
                self.f.set_high();
                //self.g.set_high();
            }
            1 => {
                //self.a.set_high();
                self.b.set_high();
                self.c.set_high();
                //self.d.set_high();
                //self.e.set_high();
                //self.f.set_high();
                //self.g.set_high();
            }
            2 => {
                self.a.set_high();
                self.b.set_high();
                //self.c.set_high();
                self.d.set_high();
                self.e.set_high();
                //self.f.set_high();
                self.g.set_high();
            }
            3 => {
                self.a.set_high();
                self.b.set_high();
                self.c.set_high();
                self.d.set_high();
                //self.e.set_high();
                //self.f.set_high();
                self.g.set_high();
            }
            4 => {
                //self.a.set_high();
                self.b.set_high();
                self.c.set_high();
                //self.d.set_high();
                //self.e.set_high();
                self.f.set_high();
                self.g.set_high();
            }
            5 => {
                self.a.set_high();
                //self.b.set_high();
                self.c.set_high();
                self.d.set_high();
                //self.e.set_high();
                self.f.set_high();
                self.g.set_high();
            }
            6 => {
                self.a.set_high();
                //self.b.set_high();
                self.c.set_high();
                self.d.set_high();
                self.e.set_high();
                self.f.set_high();
                self.g.set_high();
            }
            7 => {
                self.a.set_high();
                self.b.set_high();
                self.c.set_high();
                //self.d.set_high();
                //self.e.set_high();
                //self.f.set_high();
                //self.g.set_high();
            }
            8 => {
                self.a.set_high();
                self.b.set_high();
                self.c.set_high();
                self.d.set_high();
                self.e.set_high();
                self.f.set_high();
                self.g.set_high();
            }
            9 => {
                self.a.set_high();
                self.b.set_high();
                self.c.set_high();
                self.d.set_high();
                //self.e.set_high();
                self.f.set_high();
                self.g.set_high();
            }
            _ => {}
        };
        arduino_hal::delay_ms(1000);
    }

    fn clear_display(&mut self) {
        self.e.set_low();
        self.d.set_low();
        self.c.set_low();
        self.dp.set_low();
        self.b.set_low();
        self.a.set_low();
        self.f.set_low();
        self.g.set_low();
    }
}
