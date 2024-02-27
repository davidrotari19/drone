//Declare modules
pub mod mycpu;
pub mod nes;
pub mod alu;
pub mod cartridge;
pub mod mmapper;
pub mod nrom;
pub mod mmc1;
pub mod mmc3;
pub mod utils;
pub mod screen;
pub mod run;
pub mod cpu;
pub mod ppu;
pub mod mmc2;
pub mod mmc4;


use nes::Bus;
use cartridge::Cartridge;


///Use
use log::LevelFilter;
//use tudelft_nes_ppu::{Mirroring, run_cpu};
use tudelft_nes_ppu::{run_cpu, Mirroring};
// use crate::run::run_cpu;
//use tudelft_nes_test::TestableCpu;
use crate::mycpu::CPU;
// use crate::ppu::mirroring::Mirroring;
fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    //Flow of the program initialization
    //1. Load the ROM into the cartridge
    //2.Create a bus and connect the cartridge to it
    //3. Create a CPU and connect it to the bus

    //Get a cartridge from the ROM
    let mut cartridge =Cartridge::new();
    let roms=include_bytes!("pac_man.nes");

    //Load the ROM into the cartridge
    match cartridge.load_rom(roms){
        Ok(_) => println!("ROM loaded"),
        Err(e) => println!("Error loading ROM: {}",e),
    }
    //Create a bus
    let mut bus = Bus::new();
    //Load the cartridge into the bus
    match bus.connect_cartridge(cartridge){
        Ok(_) => println!("Cartridge loaded"),
        Err(e) => println!("Error loading cartridge: {}",e),
    }
    //Connect the bus to the CPU
    let mut cpu = CPU::new();
    match cpu.connect_bus(bus){
        Ok(_) => println!("Bus connected"),
        Err(e) => println!("Error connecting bus: {}",e),
    }
    //Reset the CPU
    cpu.reset();
    //Run the CPU
    run_cpu(cpu, Mirroring::Horizontal);
}

#[cfg(test)]
mod tests {
    use crate::mycpu::CPU;
    use log::LevelFilter;
    use tudelft_nes_test::{run_tests, TestSelector};

    /// This test fails in the template, since you didn't implement the cpu yet.
    #[test]
    fn test_all() {
        //print something
        env_logger::builder().filter_level(LevelFilter::Info).init();

        if let Err(e) = run_tests::<CPU>(TestSelector::ALL) {
            log::error!("TEST FAILED: {e}");
            assert!(false);
        }
    }
}
