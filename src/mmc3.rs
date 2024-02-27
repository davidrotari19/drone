///Use
use crate::mmapper::Mappable;
//load the error trait
use std::error::Error;
///Overview
//PRG ROM size: 16 KiB or 32 KiB
//PRG ROM bank size: Not bankswitched
//PRG RAM: None
//CHR capacity: Up to 2048 KiB ROM
//CHR bank size: 8 KiB
//Nametable mirroring: Horizontal or vertical mirroring, selectable by writing to $8000-$9FFF
//Subject to bus conflicts: Yes, but irrelevant


pub struct MMC3{
    pub num_prg_banks: u8,
    pub num_chr_banks: u8,
    pub prog_ram: Vec<u8>,
    //private variables for character rom swithcing
    chr_rom_bank_0: u8,

}
impl MMC3{
    pub fn new(prg_rom_size:u8,num_chr_banks:u8)->Self{
        Self{
            num_prg_banks: prg_rom_size,
            num_chr_banks,
            prog_ram:vec![255;8*1024],
            chr_rom_bank_0:0,

        }
    }
}


impl Mappable for MMC3{

    fn map_read_cpu(&self, address: u16) -> Result<(u32,u8),Box<dyn Error>>{
        //check if the address is in the range of the PRG ROM
        if (0x6000..=0x7FFF).contains(&address) {
            Ok((0xFFFFFFFF, self.prog_ram[(address & 0x1FFF) as usize]))
        }
        //If the address is between $8000 and $BFFF, the first bank is loaded
            //If the address is between $C000 and $FFFF, the second bank is loaded
        else {
            Ok(((address & (if self.num_prg_banks > 1 { 0x7FFF } else { 0x3FFF })) as u32, 0))
        }
    }
    fn map_write_cpu(&mut self, address: u16,data:u8)-> Result<(), Box<dyn Error>>{
        //check if the address is in the range of the PRG ROM

        if (0x6000..=0x7FFF).contains(&address) {
            self.prog_ram[(address & 0x1FFF) as usize]=data;
           Ok(())
        }
        //Bank switching
        //7  bit  0
        //---- ----
        // cccc ccCC
        //|||| ||||
        // ++++-++++- Select 8 KB CHR ROM bank for PPU $0000-$1FFF

        else {
            self.chr_rom_bank_0 = data & 0x3;
            Ok(())
        }
    }
    fn map_read_ppu(&self, address: u16) -> Result<(u32,u8),Box<dyn Error>>{
        //check if the address is in the range of the PRG ROM
        if address<=0x1FFF {
            Ok(((address as u32) + (self.chr_rom_bank_0 as u32 * 0x2000), 0))
        }
        else {
            Err("Address out of range".into())
        }
    }
    fn map_write_ppu(&mut self, _address: u16,_data:u8) -> Result<(),Box<dyn Error>>{
        //there is no write to the ppu
        Ok(())
    }
}