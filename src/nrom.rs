use std::error::Error;
///Use
use crate::mmapper::Mappable;

///Based on the https://www.nesdev.org/wiki/NROM

///Overview:
//PRG ROM size: 16 KiB for NROM-128, 32 KiB for NROM-256 (DIP-28 standard pinout)
//PRG ROM bank size: Not bankswitched
//PRG RAM: 2 or 4 KiB, not bankswitched, only in Family Basic (but most emulators provide 8)
//CHR ROM size: 8 KiB
//CHR bank size: Not bankswitched, see CNROM
//Nametable mirroring: Solder pads select vertical or horizontal mirroring
//Subject to bus conflicts: Yes, but irrelevant

///Banks
//CPU $6000-$7FFF: Family Basic only: PRG RAM, mirrored as necessary to fill entire 8 KiB window, write protectable with an external switch
//CPU $8000-$BFFF: First 16 KiB of PRG ROM
//CPU $C000-$FFFF: Last 16 KiB of PRG ROM


//NROM is the simplest mapper, it has no bank switching
//Because PRG ROM can 16 KiB or 32 KiB, the mapper will have to check the size of the ROM and load the appropriate amount of data
//This number can be found in the header of the ROM, specifically in the program bank size
pub struct NROM{
    pub num_prg_banks: u8,
    pub num_chr_banks: u8,
    pub prog_ram: Vec<u8>,

}
impl NROM{
    pub fn new(prg_rom_size:u8,num_chr_banks:u8)->Self{
        Self{
            num_prg_banks: prg_rom_size,
            num_chr_banks,
            prog_ram:vec![255;8*1024]

        }
    }
}
//
//Mapper fundament
impl Mappable for NROM{
    //NROM is the simplest mapper, it has no bank switching but because PRG ROM can 16 KiB or 32 KiB, the mapper will have to check the size of the ROM and load the appropriate amount of data
    //CPU has access to the entire 32 KiB of PRG ROM from $8000 to $FFFF
    //The data is arranged in two banks of 16k each, the first bank is loaded at $8000 and the second bank is loaded at $C000
    fn map_read_cpu(&self, address: u16) -> Result<(u32,u8),Box<dyn Error>>{

        //check if the address is in the range of the PRG ROM
        if (0x6000..=0x7FFF).contains(&address) {
            //If the address is between $6000 and $7FFF, the PRG RAM is loaded
            Ok(((address - 0x6000) as u32, self.prog_ram[(address & 0x1FFF) as usize]))
        }
        //If the address is between $8000 and $BFFF, the first bank is loaded
        else if address>=0x8000{
            //If the address is between $C000 and $FFFF, the second bank is loaded
            //The data is arranged in two banks of 16k each, the first bank is loaded at $8000 and the second bank is loaded at $C000
            //if the size of the PRG ROM is 16 KiB, the second bank is mirrored
            Ok(((address & (if self.num_prg_banks > 1 { 0x7FFF } else { 0x3FFF })) as u32, 0))
        }
        else
        {
            Err("Address out of range".into())
        }

    }
    fn map_write_cpu(&mut self, address: u16,data:u8)-> Result<(), Box<dyn Error>>{

        //check if the address is in the range of the PRG ROM
        if (0x6000..=0x7FFF).contains(&address) {
           //if address is in the range of the PRG ROM, write the data to the PRG RAM
            self.prog_ram[(address & 0x1FFF) as usize]=data;
            Ok(())
        }
        //NROM has no write access to the CPU
        else if address>=0x8000{
            Ok(())
        }
        else {
            Err("Address out of range".into())
        }

    }
    fn map_read_ppu(&self, address: u16) -> Result<(u32,u8),Box<dyn Error>>{
        //if the address is in the range of the CHR ROM, return the address of the CHR ROM
        if address<=0x1FFF{
            //The address of the CHR ROM is the same as the address of the PPU
            Ok((address as u32,0))
        }
        else{
            Err("Address out of range".into())
        }

    }
    fn map_write_ppu(&mut self, address: u16,_data:u8)-> Result<(),Box<dyn Error>>{
        //NROM has no write access to the PPU because it has no bank switching,so do nothing
        if  address<=0x1FFF {
            Err("NROM has no write access to the PPU".into())
        }
        else{
            Err("Address out of range".into())
        }
    }
}