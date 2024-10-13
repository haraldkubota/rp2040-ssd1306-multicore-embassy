# Embedded Rust and Embassy and a display

BMP280 test

## Hardware Requirements

* RP2040
* RPi Pico Probe (for downloading and status messages from the RP2040)
* I2C SSD1306 OLED display (128x64 pixel)
* BMP280 for pressure and temperature

## Software Requirements

* Install target compiler:

```
$ rustup target add thumbv6m-none-eabi
```

* To use the Pico probe:

```
$ cargo binstall probe-rs
```

* To use various tools, e.g. to see the code size of the generated ELF (e.g. `cargo size`):

```
$ cargo binstall cargo-binutils
$ cargo size
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.05s
   text    data     bss     dec     hex filename
  42356       0  103960  146316   23b8c rp-ssd1306
```

## Running it

The OLED is connected to I2C0 (GPIO 4 and 5). The BMP280 is connected to I2C1 (GPIO 2 and 3).

```
$ cargo run
   Compiling rp-bmp280 v0.1.0 (/home/harald/src/rust/rp2040/bmp280)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.90s
     Running `probe-rs run --chip RP2040 --protocol swd target/thumbv6m-none-eabi/debug/rp-bmp280`
      Erasing ✔ [00:00:00] [##############################################################################] 40.00 KiB/40.00 KiB @ 51.87 KiB/s (eta 0s )
  Programming ✔ [00:00:02] [##############################################################################] 40.00 KiB/40.00 KiB @ 19.05 KiB/s (eta 0s )    Finished in 2.964s
INFO  Hello from core 0
└─ rp_bmp280::__core0_task_task::{async_fn#0} @ src/main.rs:76  
INFO  Hello from core 1
└─ rp_bmp280::__core1_task_task::{async_fn#0} @ src/main.rs:104 
INFO  p=96619.25891922315, t=25.559689272218385
└─ rp_bmp280::__core1_task_task::{async_fn#0} @ src/main.rs:131 
INFO  p=100917.20994759991, t=25.62633829004044
└─ rp_bmp280::__core1_task_task::{async_fn#0} @ src/main.rs:131 
INFO  p=100944.19696621434, t=25.574500160483876
└─ rp_bmp280::__core1_task_task::{async_fn#0} @ src/main.rs:131
```

Use ^C to stop, but the program will continue to run on the RPi Pico.
