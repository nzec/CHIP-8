# CHIP-8 Interpreter/Emulator
A CHIP-8 Interpreter/Emulator written on [Rust](https://www.rust-lang.org/) :heart:

## Run a ROM
Download from [Releases](/../../releases) and run using
```
chip8.exe <path_to_rom>
```

or if you have [Rust](https://www.rust-lang.org/) installed, clone the repository and run using
```
cargo run <path_to_rom>
```

See [Resources](#Resources) for a ROMs.

## Information

Uses [rust_minifb](https://github.com/emoon/rust_minifb) to for display and [rodio](https://github.com/RustAudio/rodio) for sound.<br>
Timers and screeen updates at the rate of 60 Hz regardless of the Update Rate.

```
 1 2 3 C                          1 2 3 4
 4 5 6 D     is mapped to --->    Q W E R
 7 8 9 E                          A S D F
 A 0 B F                          Z X C V
```

See [#1](/../../issues/1) for Problems and Todos.

## Resources:
- [CHIP-8 Wikipedia Page](https://en.wikipedia.org/wiki/CHIP-8)
- [Cowgod's Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Mastering CHIP-8 [Matthew Mikolay]](http://mattmik.com/files/chip8/mastering/chip8.html)
- [How to write an emulator (CHIP-8 interpreter) [Multigesture]](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
- [Zophar's CHIP-8 Games Pack](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html)
- [CHIP-8 Archive](https://johnearnest.github.io/chip8Archive/)
- [Cowgod's CHIP-8 Emulator Page](http://devernay.free.fr/hacks/chip8/)
- Revival Studios CHIP-8 ROMs [Website](http://www.revival-studios.com/other.php) | [Compilation](https://github.com/dmatlack/chip8/tree/master/roms)
- [Skosulor's CHIP-8 Tester](https://github.com/Skosulor/c8int/tree/master/test)
- [corax89's chip-8-test-rom](https://github.com/corax89/chip8-test-rom)
- [massung's CHIP-8 Emulator](https://github.com/massung/CHIP-8)