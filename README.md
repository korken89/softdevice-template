# Softdevice template

This template is for when you want to make your own `softdevice`. That is, a blob of code that is
not part of the main application but is accessed via a ABI.

Useful for sharing code between multiple binaries or having code that should not be part of an
application. E.g.:

- radio drivers that both a bootloader and application can access
- large cryptography functions shared between different firmwares to not have multiple copies

## Implementing a softdevice

1. Update the `.cargo/config.toml` with the correct target for your application.
2. Update the `memory.x` according to the comments in it. 
  * Do note that the RAM is set to 0 size. If you need RAM, make sure it is NOT overlapping with application RAM.
3. Add the methods you need to `Softdevice` in `src/lib.rs`.
4. Implement the methods in `src/bin/softdevice.rs`

## Using the softdevice

1. Copy the "userspace" example in `memory.x` to you application linker script.
2. Use the softdevice you created as a library in your application to access the `Softdevice` struct.
3. Remember to flash the softdevice before the application.

## Generating a flashable softdevice

Run `cargo build-softdevice`, this will generate an `softdevice.elf` in the project root.
It has the correct address and size info for direct flashing.

In case a `binary` file is desired there is the `cargo build-softdevice-bin` alias that will
generate the binary file.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
