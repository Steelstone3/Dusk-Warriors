# Dusk Warriors

Dusk warriors is a 2D fighting game written in rust using the bevy framework.

## Developing The Game

### Running The Game

> cd ~/Dusk-Warriors
>
> cargo build
>
> cargo run

Or using bacon

> cd ~/Dusk-Warriors
>
> bacon
>
> r

### Testing The Game

> cd ~/Dusk-Warriors
>
> cargo test

Or using bacon

> cd ~/Dusk-Warriors
>
> bacon
>
> t

### Linting The Game

> cd ~/Dusk-Warriors
>
> cargo clippy --fix --allow-dirty

Or using bacon

> cd ~/Dusk-Warriors
>
> bacon
>
> c

### Formatting The Game

> cd ~/Dusk-Warriors
>
> cargo fmt

Or using bacon

> cd ~/Dusk-Warriors
>
> bacon
>
> f

## Dependencies

Follow the steps for installing rustc runtime for your given operating system.

> <https://www.rust-lang.org/tools/install>

Install bacon

> cargo install bacon

Install the dependency packages

> bash setup_dependencies.sh

Please use good op-sec habits and check the contents of both .sh files before running them as these will modify your system

### Trouble-Shooting

If there is a compilation issue whereby bevy itself won't compile.

Check dependencies are installed. For example

> "Missing ALSA-Lib"

Check the version of the toolchain

> rustup toolchain list
>
> rustup install 1.76.0 (for example)
>
> cargo +1.76.0 run (for example)
>
> rustup default 1.76.0 (for example)

Reset the toolchain

> rustup toolchain list
>
> rustup toolchain remove stable
>
> rustup toolchain add stable
