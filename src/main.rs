#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
// use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    // rtt_init_print!();
    // rprintln!("Hello, world!");
    const SYSAHBCLKCTRL: *mut u32 = 0x4004_8080 as *mut u32; // clock control register
    const GPIO_DIR: *mut u32 = 0x5000_8000 as *mut u32; // GPIO direction register
    const IOCON_PIO0_3: *mut u32 = 0x4004_400C as *mut u32; // IOCON register for pin PIO0_3
    const GPIO_DATA_MASK_ALL: *mut u32 = 0x5000_3FFC as *mut u32; // GPIO data mask register
    let mut led_on: bool = false;
    unsafe {
        // Enable GPIO clock
        let sysahbclkctrl: u32 = SYSAHBCLKCTRL.read_volatile();
        SYSAHBCLKCTRL.write_volatile(sysahbclkctrl | (1 << 6)); // Enable GPIO clock

        // Set PIO0_3 to GPIO function
        IOCON_PIO0_3.write_volatile(0); // Set PIO0_3 to GPIO function

        // Set GPIO direction for PIO0_3 to output
        let dir: u32 = GPIO_DIR.read_volatile();
        GPIO_DIR.write_volatile(dir | (1 << 3));
    }
    loop{
        unsafe {
            GPIO_DATA_MASK_ALL.write_volatile( (led_on as u32) << 3 ); // Set PIO0_3 to high or low
        }
        cortex_m::asm::delay(5_000_000);
        led_on = !led_on; // Toggle the LED state
    }
}
