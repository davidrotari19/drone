use std::error::Error;

///Based on the https://www.nesdev.org/wiki/Mapper

//The trait here implements an interface which will be the base for all the Mapper implementations

//Working principle of the Mapper:
//The mapper does not actually read the data from the ROM, it only tells the CPU where to read the data from
//It has an address as input and returns the address of the ROM where the data is stored
//The CPU will then read the data from the ROM
pub trait Mappable: Send + Sync {
    //The function here will be used to read ROM from the CPU
    fn map_read_cpu(&self, address: u16) -> Result<(u32,u8),Box<dyn Error>>;
    //ROM can not be written so this functions is an interface for the mapper to implement
    fn map_write_cpu(&mut self, address: u16,data:u8)-> Result<(), Box<dyn Error>>;
    //The function here will be used to read ROM from the PPU
    fn map_read_ppu(&self, address: u16) -> Result<(u32,u8),Box<dyn Error>>;
    //ROM can not be written so this functions is an interface for the mapper to implement
    fn map_write_ppu(&mut self, address: u16,data:u8)-> Result<(),Box<dyn Error>>;
}