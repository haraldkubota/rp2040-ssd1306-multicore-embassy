// This example shows how to use a SSD1306 OLED display via I2C to display text
// GPIO4/5 used for this (I2C0 default pins)
// 
// Core 0 does measurements and communicates via a CHANNEL to Core 1
// Core 1 does display/LED I/O 

#![no_std]
#![no_main]

use core::fmt::Write;
use core::u8;

use defmt::*;
use embassy_executor::Executor;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::i2c::{self, Async, Config, InterruptHandler};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::{I2C0, I2C1};
// use embedded_hal_async::i2c::I2c;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Timer, Instant};
use ssd1306::mode::DisplayConfig;
use ssd1306::prelude::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
static CHANNEL: Channel<CriticalSectionRawMutex, DisplayMessage, 1> = Channel::new();

enum DisplayMessage {
    LedOn,
    LedOff,
    PAndT(f64, f64),
}

bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
    I2C1_IRQ => InterruptHandler<I2C1>;
});

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());
    let led = Output::new(p.PIN_26, Level::Low);

    // Set up I2C0 for the SSD1306 OLED Display
    let i2c0 = i2c::I2c::new_async(p.I2C0, p.PIN_5, p.PIN_4, Irqs, Config::default());
    // Set up I2C1 for the VL53L0X
    let i2c1 = i2c::I2c::new_async(p.I2C1, p.PIN_3, p.PIN_2, Irqs, Config::default());

    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
        move || {
            let executor1 = EXECUTOR1.init(Executor::new());
            executor1.run(|spawner| unwrap!(spawner.spawn(core1_task(led, i2c0))));
        },
    );

    let executor0 = EXECUTOR0.init(Executor::new());

    executor0.run(|spawner| unwrap!(spawner.spawn(core0_task(i2c1))));

}

// This task measures

#[embassy_executor::task]
async fn core0_task(i2c1: embassy_rp::i2c::I2c<'static, I2C1, Async>) {
    info!("Hello from core 0");

    let mut bmp280 = bmp280_ehal::BMP280::new(i2c1).expect("BMP280 not found");
    let _ = bmp280.pressure_one_shot(); // Without this, BMP280 is not  well initialized (I get about 650 hPa)
    
    loop {
        CHANNEL.send(DisplayMessage::LedOn).await;
        let pressure = bmp280.pressure_one_shot();
        let temp = bmp280.temp();
        CHANNEL.send(DisplayMessage::LedOff).await;
        CHANNEL.send(DisplayMessage::PAndT(pressure, temp)).await;
        Timer::after_millis(997).await;    // You can measure how much time the above takes by comparing
        // now.as_millis() with counting how many times this loop ran.
        // Above code takes about 3ms including the after_millis() overhead.
    }
}


// This task does I/O

const ROW_PRESSURE: u8 = 2;
const ROW_TEMP: u8 = 3;
const ROW_TIME: u8 = 4;
const ROW_COUNTER: u8 = 5;
const COL_DATA: u8 = 9;

#[embassy_executor::task]
async fn core1_task(mut led: Output<'static>, i2c0: embassy_rp::i2c::I2c<'static, I2C0, Async>) {
    info!("Hello from core 1");
    
    let interface = I2CDisplayInterface::new(i2c0);
    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();

    let mut counter = 0;

    display.init().unwrap();
    let mut buffer = itoa::Buffer::new();

    display.clear().unwrap();
    let _ = display.write_str("P&T Display\n");
    display.set_position(0, ROW_PRESSURE).unwrap();
    let _ = display.write_str("Pressure:");
    display.set_position(0, ROW_TEMP).unwrap();
    let _ = display.write_str("Temp:");
    display.set_position(0, ROW_TIME).unwrap();
    let _ = display.write_str("Time:");
    display.set_position(0, ROW_COUNTER).unwrap();
    let _ = display.write_str("Counter:");

    loop {
        match CHANNEL.receive().await {
            DisplayMessage::LedOn => led.set_high(),
            DisplayMessage::LedOff => led.set_low(),
            DisplayMessage::PAndT(p, t) => {
                info!("p={}, t={}", p, t);
                let s: &str = buffer.format((p/100.0) as u32); // use hPa (typical range: 990-1040)
                display.set_position(COL_DATA, ROW_PRESSURE).unwrap();
                let _ = display.write_str(s);
                let _ = display.write_str(" ");
                let s: &str = buffer.format(t as u16);
                let after_comma_digits = ((t*100.0) as u16) % 100;
                display.set_position(COL_DATA, ROW_TEMP).unwrap();
                let _ = display.write_str(s);
                let _ = display.write_str(".");
                let s = buffer.format(after_comma_digits);
                let _ = display.write_str(s);
                let _ = display.write_str(" ");    
                let now = Instant::now();
                let s: &str = buffer.format(now.as_millis());
                display.set_position(COL_DATA, ROW_TIME).unwrap();
                let _ = display.write_str(s);
                let s: &str = buffer.format(counter);
                display.set_position(COL_DAT, ROW_COUNTER).unwrap();
                let _ = display.write_str(s);
                counter += 1;                
            }
        }
    }
}
