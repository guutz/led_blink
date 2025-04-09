#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use lpc8n04_pac::Peripherals;
// use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    let p: Peripherals = Peripherals::take().unwrap();
    // rtt_init_print!();

    // Enable the GPIO clock
    p.syscon.sysahbclkctrl().modify(|_, w| {
        w.gpio().bit(true)
    });

    // Set the GPIO function for PIO0_3 to GPIO
    p.iocon.pio03().modify(|_, w| {
        w.func().set(0)
    });

    // Set the GPIO direction for PIO0_3 to output
    p.gpio.dir().modify(|r, w| unsafe { w.io().bits(
        r.io().bits() | (1 << 3) 
    ) });

    let mut led_on: bool = false;
    loop{
        p.gpio.gpiodata(0xFF).write(|w| unsafe {
            w.data().bits( (led_on as u16) << 3 )
        });
        cortex_m::asm::delay(1_000_000);
        led_on = !led_on;
    }
}
