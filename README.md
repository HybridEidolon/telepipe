# PSOGC telepipe

[![Travis Build Status](https://travis-ci.org/BygoneWorlds/telepipe.svg?branch=master)](https://travis-ci.org/BygoneWorlds/telepipe)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/BygoneWorlds/telepipe/branch/master?svg=true)](https://ci.appveyor.com/project/Furyhunter/telepipe)

A proxy for _Phantasy Star Online_ games for the GameCube.

## Compiling

Install the latest version of Rust and:

    $ cargo build --release

## Using

**TODO**

Configuration will be handled by a toml file which specifies the PSO server
to connect to and what interfaces to bind on.

This program is designed primarily to be used with the [Dolphin emulator] on
systems where [Broadband Adapter] emulation is supported (not macOS), however
it may be used as a normal proxy for real GameCubes and Wiis via PPP emulation
on capable GameCube kernel shims.

## License

Telepipe is licensed under the terms of the [MIT License].

[Dolphin emulator]: http://dolphin-emu.org/
[Broadband Adapter]: https://wiki.dolphin-emu.org/index.php?title=Broadband_Adapter
[MIT License]: http://spdx.org/licenses/MIT.html
