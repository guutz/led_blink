#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use lpc8n04_pac::Peripherals;

#[entry]
fn main() -> ! {
    let p: Peripherals = Peripherals::take().unwrap();

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
        p.gpio.gpiodata(0x3ffc).write(|w| unsafe {
            w.data().bits( if led_on { 0x3ffc } else { 0 } ) // Set PIO0_3 high or low
        });        
        cortex_m::asm::delay(100_000);
        led_on = !led_on; // Toggle the LED state
    }
}
