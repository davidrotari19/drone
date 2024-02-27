

## Important Notes
There are some important notes about this emulator:
The latest version of the emulator is in the branch `main`. This branch contains 
a version of ppu derived from the  tudelft-nes-ppu. This is the main version of the emulator.
But because the tudelft-nes-test requires the old version of ppu, we also have a branch `only_test` which 
contains the same version of the CPU, but it is compatible with the old version of ppu.
So very important, if you want to run the emulator, you should use the branch `main` and if you want to run the test with the tudelft-nes-test , 
you should use the branch `only_test`. In case your test has a graphical display you can use the branch `main` to run the test, just pass the rom to the cargo.
Example would be nestest.nes test, whose results can be seen on the screen. So final note is that the branch `only_test` is only for the test with the tudelft-nes-test.
If you want to test DEFAULT,OFFICIAL_INSTRUCTIONS,ALL_INSTRUCTIONS, you should use the branch `only_test'.
## Usage

```
cargo run
```

in case of testing with tudelft_nes-test, you should use the branch `only_test` and run the test with the following command:

```
cargo test
```
## Test passed
- [x] nestest.nes <br />
 ![nestest](/pictures/nested_test_official.png) <br />
 ![nestest](/pictures/nested_test_all.png) <br />
- [x] nrom.nes <br />
 ![nrom](/pictures/nrom_test.png) <br />
- [x] official_only.nes <br />
 ![official_only](/pictures/official_isntruction_test.png) <br />
- [x] all_instrs.nes <br />
 ![all_instrs](/pictures/unofficial_instruction_test.png) <br />


## Games played
- [x] Pac-man <br />
![Alt text](/pictures/pac_man.png "Pac-Man")


## Supported Mappers
- [x] NROM
- [x] MMC1
- [x] UxROM
- [x] CNROM

## Features
- [x] CPU
- [x] PPU(based on tudelft-nes-ppu)
- [x] Cartridge
- [x] Mapper
- [x] Controller

## Operations
- [x] Load ROM in cartridge
- [x] Create a new bus and connect the cartridge to it
- [x] Create a new CPU and connect the bus to it
- [x] Run the CPU

## TODO
- [ ] APU
- [ ] Save state


## Issues
- [ ] For some MMC1(Godzilla -Monster of Monsters) games, the ppu gets in overflow mode and the screen is black. Maybe the problem is in the mapper.

## References
- [NESdev](https://wiki.nesdev.com/w/index.php/Nesdev_Wiki)
- [NESdev wiki](https://wiki.nesdev.com/w/index.php/Nesdev_Wiki)
- [NESdev wiki CPU](https://wiki.nesdev.com/w/index.php/CPU)
- [NESdev wiki PPU](https://wiki.nesdev.com/w/index.php/PPU)
- [NESdev wiki Cartridge](https://wiki.nesdev.com/w/index.php/Cartridge)
- [NESdev wiki Mapper](https://wiki.nesdev.com/w/index.php/Mapper)
- [NESdev wiki Controller](https://wiki.nesdev.com/w/index.php/Standard_controller)
- [NESdev wiki Test ROMs](https://wiki.nesdev.com/w/index.php/Emulator_tests)

