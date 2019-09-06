# Hedera C SDK
> Hedera SDK for C

## License

Licensed under Apache License, 
Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.


## Compiling
In order to compile the following dependencies must be met:
    OS Level:
        - musl-tools
    Rust Level:
        - rustup
        - rustc
        - cargo
        - nightly-toolchain
        - target x86_64-unknown-linux-musl
        - target x86_64-apple-darwin
        - target x86_64-pc-windows-gnu
    Protobuf:
        - protoc

If the above dependencies are met ignore the instructions below.

### Install Dependencies (Unix)
    1. Install Protobuf Compiler:
        - Follow instructions here: https://github.com/protocolbuffers/protobuf/blob/master/src/README.md
    2. Install Rust:
        - `curl https://sh.rustup.rs -sSf | sh`
        - `rustup update`
        - `rustup toolchain install nightly`
        - `rustup target add x86_64-unknown-linux-musl --toolchain=nightly`
            - NOTE: If other toolchains are installed you may need to repeat the above
            step for each of the toolchains. List toolchains with this command:
            `rustup toolchain list`
        - `rustup target add x86_64-apple-darwin --toolchain=nightly`
            - NOTE: If other toolchains are installed you may need to repeat the above
            step for each of the toolchains. List toolchains with this command:
            `rustup toolchain list`
        - `rustup target add target x86_64-pc-windows-gnu --toolchain=nightly`
            - NOTE: If other toolchains are installed you may need to repeat the above
            step for each of the toolchains. List toolchains with this command:
            `rustup toolchain list`
    3. Install Musl-Tools
        - Debian (based): `sudo apt install musl-tools`
        - Mac: `brew install FiloSottile/musl-cross/musl-cross`

### Compile For Testing:
    1. `cd hedera-sdk-c`
    2. `python3 x.py`

### Compile For Release:
    1. `cd hedera-sdk-c`
    2. `python3 x.py -R`
