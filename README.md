# CHIP-8 Interpreter/Emulator
A CHIP-8 Interpreter/Emulator written on [Rust](https://www.rust-lang.org/) :heart:

## Running
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

## ROMs

I have included a lot of ROMs in the `/roms` from the following sources

- GAMES - [SVision-8 (devernay.free.fr)](http://devernay.free.fr/hacks/chip8/GAMES.zip)
- c8games - [Zophar's Domain Chip-8 Games Pack (zophar.net)](http://devernay.free.fr/hacks/chip8/GAMES.zip)
- revival - Revival Studios Chip-8 Program Pack [Website (revival-studios.com)](http://www.revival-studios.com/other.php) | [dmatlack/chip8/roms](https://github.com/dmatlack/chip8/tree/master/roms)
- chip8-test-rom - [corax89/chip-8-test-rom](https://github.com/corax89/chip8-test-rom)
- c8_test - [Skosulor/c8int/test](https://github.com/Skosulor/c8int/tree/master/test)
- chip8Archive - [JohnEarnest/chip8Archive/roms](https://github.com/JohnEarnest/chip8Archive/tree/master/roms)

## Resources

### Learning
- [CHIP-8 Wikipedia Page](https://en.wikipedia.org/wiki/CHIP-8)
- [Cowgod's Technical Reference (devernay.free.fr)](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Mastering CHIP-8 (mattmik.com)](http://mattmik.com/files/chip8/mastering/chip8.html)
- [How to write an emulator (CHIP-8 interpreter) (multigesture.net)](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)

### Tools & Further References
- [SVision-8 (devernay.free.fr)](http://devernay.free.fr/hacks/chip8/) lots of links to helpful documentation
- [massung/CHIP-8](https://github.com/massung/CHIP-8) an emulator for CHIP-8 and its derivaties with many nice features
- [JohnEarnest/Octo](https://github.com/JohnEarnest/Octo) a high level assembler for CHIP-8 with lots of helpful guides and ROMs
- OctoJam [1](http://www.awfuljams.com/octojam-i) [2](http://www.awfuljams.com/octojam-ii) [3](http://www.awfuljams.com/octojam-iii) [4](http://www.awfuljams.com/octojam-iv) [5](http://www.awfuljams.com/octojam-v) [6](https://itch.io/jam/octojam-6) an Octo-centric game jam held every October (running since 2014)
- [Chromatophore/HP48-Superchip](https://github.com/Chromatophore/HP48-Superchip) research into behaviour of SCHIP
- [r/EmuDev](https://reddit.com/r/EmuDev/) community interested in Emulator Development