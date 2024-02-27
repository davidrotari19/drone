use std::error::Error;
//use
use crate::mmapper::Mappable;
///MMC002
pub struct MMC2{
    //number of prg rom banks in terms of 16kb
    pub prg_rom_size: u8,
    //number of chr rom banks in terms of 8kb
    pub num_chr_banks: u8,
    //program ram also called static ram
    pub prog_ram: Vec<u8>,
    //private variables for program rom swithcing
    //first bank
    prg_rom_bank_0: u8,
    //second bank
    prg_rom_bank_1: u8,
}

impl MMC2{
    //constructor
    pub fn new(prg_rom_size:u8,num_chr_banks:u8)->Self{
        Self{
            prg_rom_size,
            num_chr_banks,
            prog_ram:vec![255;8*1024],
            prg_rom_bank_0:0,
            prg_rom_bank_1:prg_rom_size-1,
        }
    }
}

impl Mappable for MMC2{

    //read the cpu address
    fn map_read_cpu(&self, address: u16) -> Result<(u32,u8),Box<dyn Error>>{
        //check if the address is in the range of the PRG ROM
        if (0x6000..=0x7FFF).contains(&address) {
            //return the data from the program ram
            Ok((0xFFFFFFFF, self.prog_ram[(address & 0x1FFF) as usize]))
        }
        //If the address is between $8000 and $BFFF, the first bank is loaded
        else if (0x8000..=0xBFFF).contains(&address){
           Ok((((address & 0x3FFF) as u32 + (self.prg_rom_bank_0 as u32 * 0x4000)) as u32, 0))
       }
        //If the address is between $C000 and $FFFF, the second bank is loaded
       else if address >=0xC000{
           Ok((((address & 0x3FFF) as u32 + (self.prg_rom_bank_1 as u32 * 0x4000)) as u32, 0))
       }
       else{
           Err("Address out of range".into())
       }

    }

    //write to the cpu address
    fn map_write_cpu(&mut self, address: u16,data:u8)-> Result<(), Box<dyn Error>>{
         //check if the address is in the range of the PRG ROM
        if (0x6000..=0x7FFF).contains(&address) {
            //write to the program ram
            self.prog_ram[(address & 0x1FFF) as usize]=data;
           Ok(())}
        //Update the first bank offset
        else if address >=0x8000{
            //get the bank number from the data
            self.prg_rom_bank_0 = data & 0x0F;
            Ok(())
        }
        else{
            Err("Address out of range".into())
        }

    }

    //read the ppu address
    fn map_read_ppu(&self, address: u16) -> Result<(u32,u8),Box<dyn Error>>{
        //check if the address is in the range of the CHR ROM
        if address < 0x2000 {
            Ok((address as u32,0))
        }
        else{
            Err("Address out of range".into())
        }

    }

    //write to the ppu address
    fn map_write_ppu(&mut self, address: u16,_data:u8)-> Result<(),Box<dyn Error>>{
        //check if the address is in the range of the CHR ROM
        if address < 0x2000 {
            Ok(())
        }
        else{
            Err("Address out of range".into())
        }
       }
}