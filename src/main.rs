#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use embedded_hal::digital::v2::{PinState, OutputPin};
use panic_halt as _;
use arduino_hal::{port::{mode::{Output, Input, Floating}, Pin}, hal::port::{Dynamic, PD2}};
use arduino_hal::prelude::*;
use core::cell;
use embedded_hal::serial::Read;

// Possible Values:
//
// ╔═══════════╦══════════════╦═══════════════════╗
// ║ PRESCALER ║ TIMER_COUNTS ║ Overflow Interval ║
// ╠═══════════╬══════════════╬═══════════════════╣
// ║        64 ║          250 ║              1 ms ║
// ║       256 ║          125 ║              2 ms ║
// ║       256 ║          250 ║              4 ms ║
// ║      1024 ║          125 ║              8 ms ║
// ║      1024 ║          250 ║             16 ms ║
// ╚═══════════╩══════════════╩═══════════════════╝
const PRESCALER: u32 = 1024;
const TIMER_COUNTS: u32 = 125;
const DEBOUNCE_WAIT_TIME: u32 = 100;

const MILLIS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / 16000;

static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

fn millis_init(tc0: arduino_hal::pac::TC0) {
    // Configure the timer for the above interval (in CTC mode)
    // and enable its interrupt.
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| w.bits(TIMER_COUNTS as u8));
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });
}

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    })
}

fn millis() -> u32 {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
}

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

    segment.display();
}

struct Segments {
    i: u8,
    pins: [Pin<Output, Dynamic>; 7],
    button: Pin<Input<Floating>, PD2>,
}

impl Segments {
    fn new() -> Segments {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        millis_init(dp.TC0);

        // Enable interrupts globally
        unsafe { avr_device::interrupt::enable() };

        let pin_arr: [Pin<Output, Dynamic>; 7] = [
            pins.d4.into_output().downgrade(),  // g
            pins.d5.into_output().downgrade(),  // f
            pins.d13.into_output().downgrade(), // e
            pins.d12.into_output().downgrade(), // d
            pins.d10.into_output().downgrade(), // c
            pins.d8.into_output().downgrade(),  // b
            pins.d7.into_output().downgrade(),  // a
        ];
        let button = pins.d2.into_floating_input();

        Segments {
            i: 0,
            pins: pin_arr,
            button,
        }
    }

    fn display(&mut self) -> ! {
        loop {
            let bits = Segments::map_bits(self.i);
            for n in 0..7 {
                let _ = self.pins[n].set_state(PinState::from(bits & 1 << n != 0));
            }
            self.i = (self.i + 1) % 10;
            let mut time = millis();
            let mut now = time;
            while now - time < DEBOUNCE_WAIT_TIME {
                if self.button.is_high() {
                    time = millis();
                }
                now = millis();
            }
            // block and wait for input
            while self.button.is_low() {}
        }
    }

    fn map_bits(i: u8) -> u8 {
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
