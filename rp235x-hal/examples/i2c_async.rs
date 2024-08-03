//! # I²C Example
//!
//! This application demonstrates how to talk to I²C devices with an RP235x.
//! in an Async environment.
//!
//! It may need to be adapted to your particular board layout and/or pin assignment.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp235x_hal as hal;

// Some things we need
use hal::{
    fugit::RateExtU32,
    gpio::bank0::{Gpio20, Gpio21},
    gpio::{FunctionI2C, Pin, PullUp},
    i2c::Controller,
    pac::interrupt,
    Clock, I2C,
};

// Import required types & traits.
use embedded_hal_async::i2c::I2c;

/// Tell the Boot ROM about our application
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

/// Bind the interrupt handler with the peripheral
#[interrupt]
unsafe fn I2C0_IRQ() {
    use hal::async_utils::AsyncPeripheral;
    I2C::<hal::pac::I2C0, (Gpio20, Gpio21), Controller>::on_interrupt();
}

/// The function configures the RP235x peripherals, then performs a single I²C
/// write to a fixed address.
async fn demo() {
    let mut pac = hal::pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure two pins as being I²C, not GPIO
    let sda_pin: Pin<_, FunctionI2C, PullUp> = pins.gpio20.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, PullUp> = pins.gpio21.reconfigure();

    // Create the I²C drive, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let mut i2c = hal::I2C::new_controller(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        clocks.system_clock.freq(),
    );

    // Unmask the interrupt in the NVIC to let the core wake up & enter the interrupt handler.
    // Each core has its own NVIC so these needs to executed from the core where the IRQ are
    // expected.
    unsafe {
        hal::pac::NVIC::unpend(hal::pac::Interrupt::I2C0_IRQ);
        hal::pac::NVIC::unmask(hal::pac::Interrupt::I2C0_IRQ);
    }

    // Asynchronously write three bytes to the I²C device with 7-bit address 0x2C
    i2c.write(0x76u8, &[1, 2, 3]).await.unwrap();

    // Demo finish - just loop until reset
    core::future::pending().await
}

/// Entry point to our bare-metal application.
#[hal::entry]
fn main() -> ! {
    let runtime = nostd_async::Runtime::new();
    let mut task = nostd_async::Task::new(demo());
    let handle = task.spawn(&runtime);
    handle.join();
    unreachable!()
}
