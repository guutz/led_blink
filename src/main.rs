#![no_std]
#![no_main]

mod vector_table;
use cortex_m_rt::entry;
use panic_halt as _;
use lpc8n04_pac::Peripherals;
// use rtt_target::{rtt_init_print, rprintln};

const GPIODATA_MASK_PIO0_3: usize = 1 << 3;

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

    loop{
        dit(&p);
        dit(&p);
        dit(&p);
        dah(&p);
        dah(&p);
        dah(&p);
        dit(&p);
        dit(&p);
        dit(&p);
        space();
    }
}

fn toggle_led(p: &lpc8n04_pac::Peripherals) {
    // Toggle the LED state
    p.gpio.gpiodata(GPIODATA_MASK_PIO0_3).modify(|r, w| unsafe {
        // Read the current state of the GPIO pin
        let current_state = r.data().bits();
        // Toggle the state
        w.data().bits(current_state ^ GPIODATA_MASK_PIO0_3 as u16)
    });
}

const DIT_DELAY: u32 = 100_000;
const DAH_DELAY: u32 = 300_000;
const SPACE_DELAY: u32 = 100_000;
const WORD_SPACE_DELAY: u32 = 700_000;
const MULTIPLIER: u32 = 5;

fn dit(p: &lpc8n04_pac::Peripherals) {
    toggle_led(&p);
    cortex_m::asm::delay(DIT_DELAY / MULTIPLIER);
    toggle_led(&p);
    cortex_m::asm::delay(SPACE_DELAY / MULTIPLIER);
}

fn dah(p: &lpc8n04_pac::Peripherals) {
    toggle_led(&p);
    cortex_m::asm::delay(DAH_DELAY / MULTIPLIER);
    toggle_led(&p);
    cortex_m::asm::delay(SPACE_DELAY / MULTIPLIER);
}

fn space() {
    cortex_m::asm::delay(WORD_SPACE_DELAY / MULTIPLIER);
}