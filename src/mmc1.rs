use std::error::Error;
///Use
use crate::mmapper::Mappable;
///Based on the https://www.nesdev.org/wiki/MMC1
///Overview:
//PRG ROM capacity : 256 KiB(512 kiB for MMC1A)
//PRG ROM bank size : 16 KiB+16 KiB(fixed) or 32 KiB(fixed)
//PRG RAM capacity : 32 KiB
//PRG RAM bank size : 8 KiB
//CHR ROM capacity : 128 KiB
//CHR ROM bank size :4 KiB+4 KiB(fixed) or 8 KiB(fixed)

///Banks
//CPU $6000-$7FFF: 8 KB PRG RAM bank, (optional)
//CPU $8000-$BFFF: 16 KB PRG ROM bank, either switchable or fixed to the first bank
//CPU $C000-$FFFF: 16 KB PRG ROM bank, either fixed to the last bank or switchable
//PPU $0000-$0FFF: 4 KB switchable CHR bank
//PPU $1000-$1FFF: 4 KB switchable CHR bank


///Registers
//Load register ($8000-$FFFF)
//7  bit  0
// ---- ----
// Rxxx xxxD
// |       |
// |       +- Data bit to be shifted into shift register, LSB first
// +--------- 1: Reset shift register and write Control with (Control OR $0C),
//               locking PRG ROM at $C000-$FFFF to the last bank.

//Control (internal, $8000-$9FFF)
//4bit0
// -----
// CPPMM
// |||||
// |||++- Mirroring (0: one-screen, lower bank; 1: one-screen, upper bank;
// |||               2: vertical; 3: horizontal)
// |++--- PRG ROM bank mode (0, 1: switch 32 KB at $8000, ignoring low bit of bank number;
// |                         2: fix first bank at $8000 and switch 16 KB bank at $C000;
// |                         3: fix last bank at $C000 and switch 16 KB bank at $8000)
// +----- CHR ROM bank mode (0: switch 8 KB at a time; 1: switch two separate 4 KB banks)


//CHR bank 0 (internal, $A000-$BFFF)
//4bit0
// -----
// CCCCC
// |||||
// +++++- Select 4 KB or 8 KB CHR bank at PPU $0000 (low bit ignored in 8 KB mode)


//CHR bank 1 (internal, $C000-$DFFF)
//4bit0
// -----
// CCCCC
// |||||
// +++++- Select 4 KB CHR bank at PPU $1000 (ignored in 8 KB mode)


//PRG bank (internal, $E000-$FFFF)
//4bit0
// -----
// RPPPP
// |||||
// |++++- Select 16 KB PRG ROM bank (low bit ignored in 32 KB mode)
// +----- MMC1B and later: PRG RAM chip enable (0: enabled; 1: disabled; ignored on MMC1A)
//        MMC1A: Bit 3 bypasses fixed bank logic in 16K mode (0: affected; 1: bypassed)





///MMC1 struct
pub struct MMC1{
    //Number of PRG ROM banks in form of 16KiB
    pub prg_rom_size: u8,
    //Number of CHR ROM banks in form of 8KiB
    pub chr_rom_size: u8,

   //Load register
   pub load_register: u8,
   //Control register
   pub control_register: u8,


   //The program banks are 16kb in size
   //The first program bank offset
    pub prg_bank_0: u8,
    //The second program bank offset
    pub prg_bank_1: u8,

    //in case of 32KB mode
    pub prg_bank_32: u8,

    //The character banks are 4kb in size
    //The first character bank offset
    pub chr_bank_0: u8,
    //The second character bank offset
    pub chr_bank_1: u8,

    //in case of 8KB mode
    pub chr_bank_8: u8,

    //Mirror mode
    pub mirror_mode: u8,

    //The write is done serial by writing to $8000-$FFFF so we need to keep track of the number of writes
    pub write_count: u8,

    //static ram
    pub prg_ram:Vec<u8>
}

impl MMC1{
    pub fn new(prg_rom_size:u8, chr_rom_size:u8)->Self{
        Self{
            prg_rom_size,
            chr_rom_size,
            load_register: 0,
            control_register: 0x1C,
            prg_bank_0: 0,
            prg_bank_1: prg_rom_size-1,
            prg_bank_32: 0,
            chr_bank_0: 0,
            chr_bank_1: 0,
            chr_bank_8: 0,
            mirror_mode: 0,
            write_count: 0,
            prg_ram: vec![0;8*1024]
        }
    }
}

impl Mappable for MMC1 {
    //The function here will be used to read ROM from the CPU
    fn map_read_cpu(&self, address: u16) -> Result<(u32,u8),Box<dyn Error>>{


        //Optional PRG RAM
        if (0x6000..=0x7FFF).contains(&address) {
            //The address is in the range of the PRG RAM so we need to return the value from the PRG RAM
            return Ok((0xFFFFFFFF,self.prg_ram[(address - 0x6000) as usize] as u8));
        }
        //PRG ROM address
        else if address>=0x8000 {
            //if the mode is 16KB
            if self.control_register & 0b01000!=0 {

                 //The first 16kb of the program bank
                 if (0x8000..=0xBFFF).contains(&address) {
                     return Ok(((self.prg_bank_0 as u32 )*0x4000 + (address & 0x3FFF) as u32, 0));
                 }

                 //The second 16kb of the program bank is fixed
                 else if address >= 0xC000 {
                     return Ok((((self.prg_bank_1 as u32)* 0x4000) + (address & 0x3FFF) as u32, 0));
                 }

             }else {
                 //in case of 32KB mode
                 return Ok((((self.prg_bank_32 as u32)* 0x8000) + (address & 0x7FFF) as u32,0));

             }
        }

            Err("Invalid address".into())


    }
    //The function here will be used to write to ROM from the CPU
    fn map_write_cpu(&mut self, address: u16,data:u8)-> Result<(), Box<dyn Error>>{



        //Optional PRG RAM
        if (0x6000..=0x7FFF).contains(&address) {
            //Update the PRG RAM
            self.prg_ram[(address - 0x6000) as usize]=data;
            return Ok(());
        }
        //PRG ROM
        else if address>=0x8000 {
            //The write is done serial by writing to $8000-$FFFF so we need to keep track of the number of writes
            //The first write is to the load register
            //if the data is $80 or $00 then the load register is reset
            if data & 0x80 != 0 {
                //Reset the load register
                self.load_register = 0;
                //Reset the write count
                self.write_count = 0;
                //Reset the control register
                self.control_register |= 0x0C;
                return Ok(());
            }
            else{
                 // Load data in serially into load register
                 // It arrives LSB first, so implant this at
                // bit 5. After 5 writes, the register is ready
                // to be used
                self.load_register>>=1;
                self.load_register|=(data&0x01)<<4;
                self.write_count += 1;

                // If we have 5 writes, then we can use the
                // load register to set the control register and the bank registers
                if self.write_count==5 {
                    // selected by bits 14 and 13 of the address which will offer information about the register to be updated
                    let target_register = (address>>13)&0x03;

                    match target_register{
                        //Control register
                        0=>{
                            //the adress 0x8000 - 0x9FFF
                            self.control_register = self.load_register & 0x1F;
                            //Update the mirror mode
                            self.mirror_mode = self.control_register & 0b11;
                        },
                        1=>{ //the adress 0xA000 - 0xBFFF
                            //Update the first character bank
                            if self.control_register & 0b10000 != 0 {
                                self.chr_bank_0 = self.load_register & 0x1F;
                            }else{
                                //select 8kb mode
                                self.chr_bank_8 = self.load_register & 0x1E;
                            }
                        },
                        2=>{
                            //the adress 0xC000 - 0xDFFF
                            if self.control_register & 0b10000 != 0 {
                                self.chr_bank_1 = self.load_register & 0x1F;
                            }
                        },
                        3=>{
                            //the adress 0xE000 - 0xFFFF
                            //configure the program bank
                            //get the last 2 bits of the control register
                            let last_2_bits = (self.control_register>>2) & 0x03;
                            match last_2_bits{
                                0 | 1 =>
                                    //32KB mode
                                    {self.prg_bank_32 = (self.load_register & 0x0E) >> 1;},
                                2=>{
                                       //fix the first 16kb bank to the 0x8000 of the CPU
                                        self.prg_bank_0 = 0;
                                        // Set the second 16kb bank to the last 4 bits of the load register
                                        self.prg_bank_1 = self.load_register & 0x0F;
                                    },
                                3=>{
                                        //fix the last 16kb bank to the last bank of the ROM
                                        self.prg_bank_1 = self.prg_rom_size - 1;
                                        // Set the first 16kb bank to the last 4 bits of the load register
                                        self.prg_bank_0 = self.load_register & 0x0F;
                                    },
                                _=>panic!("Invalid last 2 bits"),
                            }

                        },
                        _=>{}
                    }


                    //if we have 5 writes then we reset the write count
                    self.write_count=0;
                    //Reset the load register
                    self.load_register=0;

                }
            }
        }
        Ok(())

    }
    //The function here will be used to read ROM from the PPU
    fn map_read_ppu(&self, address: u16) -> Result<(u32,u8),Box<dyn Error>>{
        //CHR ROM
        if address<=0x1FFF {
            //8KB mode
            return if self.chr_rom_size == 0 {
                Ok((address as u32, 0))
            } else if self.control_register & 0b10000 != 0 {
                //4kb CHR Bank Mode
                if address <= 0x0FFF {
                    //The first 4kb of the character bank
                    Ok(((self.chr_bank_0 as u32 * 0x1000) + (address & 0x0FFF) as u32, 0))
                } else {
                    //The second 4kb of the character bank
                    Ok(((self.chr_bank_1 as u32 * 0x1000) + (address & 0x0FFF) as u32, 0))
                }
            } else {
                //8K CHR Bank Mode
                Ok(((self.chr_bank_8 as u32 * 0x2000) + (address & 0x1FFF) as u32, 0))
            }
        }

        Err("Invalid address".into())

    }
    //ROM can not be written so this functions is just a dummy
    fn map_write_ppu(&mut self, address: u16,_data:u8)-> Result<(),Box<dyn Error>>{
        //CHR ROM
        if address<=0x1FFF {
            //8KB mode
            if self.control_register & 0b1000 == 0 {
                return Err("CHR ROM is not writable".into());
            }
            //4KB mode
            else {
                //The first 4kb of the character bank
                if address <= 0x0FFF {
                    return Err("CHR ROM is not writable in 4k mode".into());
                }
                //The second 4kb of the character bank
                else if address >= 0x1000 {
                    return Err("CHR ROM is not writable".into());
                }
            }
        }
        Err("Invalid address".into())

    }

}