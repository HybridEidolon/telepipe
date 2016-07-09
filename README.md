# PSOGC telepipe

[![Travis Build Status]](https://travis-ci.org/BygoneWorlds/telepipe)
[![AppVeyor Build Status]](https://ci.appveyor.com/project/Furyhunter/telepipe)

A proxy for _Phantasy Star Online_ games for the GameCube.

This program is designed primarily to be used with the [Dolphin emulator] on
systems where [Broadband Adapter] emulation is supported (not macOS), however
it may be used as a normal proxy for real GameCubes and Wiis via PPP emulation
on capable GameCube kernel shims.

## Compiling

Install the latest stable version of Rust and:

    $ cargo build --release

Omitting the `--release` will build in debug mode; all optimizations will be
omitted, and integer overflow checks will be added.

## Usage

    Telepipe 1.0
    Eidolon (@HybridEidolon)
    A proxy for Phantasy Star Online games for the GameCube.

    USAGE:
        telepipe.exe [OPTIONS]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -c, --config <PATH>    Sets the config file. [default: telepipe.toml]

An example config file is located in `telepipe.toml` in the project.

## License

Telepipe is licensed under the terms of the [MIT License].

[Dolphin emulator]: http://dolphin-emu.org/
[Broadband Adapter]: https://wiki.dolphin-emu.org/index.php?title=Broadband_Adapter
[MIT License]: http://spdx.org/licenses/MIT.html
[Travis Build Status]: https://travis-ci.org/BygoneWorlds/telepipe.svg?branch=master
[AppVeyor Build Status]: https://ci.appveyor.com/api/projects/status/github/BygoneWorlds/telepipe?svg=true&branch=master
