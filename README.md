# Embedded Rust and Embassy and a display

I was looking for a ready-to-use [Embassy](https://embassy.dev/) example using a SSD1306 via I2C on a RP2040. Could not find one, so I created this repo.

## Hardware Requirements

* RP2040
* I2C SSD1306 OLED display (128x64 pixel)
* RPi Pico Probe (for downloading and status messages from the RP2040)

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
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.04s
   text    data     bss     dec     hex filename
  16832       0  103960  120792   1d7d8 led-multicore
```

## Running it

I am using GPIO4 and GPIO5 for I2C SDA and SCL

```
$ cargo run
   Compiling led-multicore v0.1.0 (/home/harald/git/rp2040-led-multicore-embassy)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.82s
     Running `probe-rs run --chip RP2040 --protocol swd target/thumbv6m-none-eabi/debug/led-multicore`
      Erasing ✔ [00:00:00] [#########################################################################] 20.00 KiB/20.00 KiB @ 50.74 KiB/s (eta 0s )
  Programming ✔ [00:00:01] [#########################################################################] 20.00 KiB/20.00 KiB @ 19.73 KiB/s (eta 0s )    Finished in 1.45s
INFO  Hello from core 0
└─ led_multicore::__core0_task_task::{async_fn#0} @ src/main.rs:48  
INFO  Hello from core 1
└─ led_multicore::__core1_task_task::{async_fn#0} @ src/main.rs:59
```

Use ^C to stop, but the program will continue to run on the RPi Pico.

