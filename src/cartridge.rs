use std::error::Error;
use crate::mmapper::Mappable;
use crate::utils::MAPPERS;
use crate::mmc1::MMC1;
use crate::nrom::NROM;

///Based on documentation here https://www.nesdev.org/wiki/INES
//An iNES file consists of the following sections, in order:
//1.Header (16 bytes)
//2.Trainer, if present (0 or 512 bytes)
//3.PRG ROM data (16384 * x bytes)
//4.CHR ROM data, if present (8192 * y bytes)
//5.PlayChoice INST-ROM, if present (0 or 8192 bytes)
//6.PlayChoice PROM, if present (16 bytes Data, 16 bytes CounterOut) (this is often missing, see PC10 ROM-Images)
//7.PRG RAM, if present (8192 * z bytes)
//8. Mapper, if present (0 or variable size)

///Header format
//1.0-3: Constant $4E $45 $53 $1A ("NES" followed by MS-DOS end-of-file)
//4: Size of PRG ROM in 16 KB units
//5: Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)
//6: Flags 6 - Mapper, mirroring, battery, trainer
//7: Flags 7 - Mapper, VS/Playchoice, NES 2.0
//8: Flags 8 - PRG-RAM size (rarely used extension)
//9: Flags 9 - TV system (rarely used extension)
//10: Flags 10 - TV system, PRG-RAM presence (unofficial, rarely used extension)
//11-15: Unused padding (should be filled with zero, but some rippers put their name across bytes 7-15)



pub struct Cartridge {
    //program memory
    pub prg_rom: Vec<u8>,
    //character memory this can be ROM or RAM
    pub chr_rom: Vec<u8>,
    //mapper
    pub mapper_type:MAPPERS,
    //ROM got as input
    pub rom: Vec<u8>,
    //mapper id
    pub mapper_id: u8,
    //number of program banks
    pub prg_rom_size: u8,
    //number of character banks
    pub chr_rom_size: u8,
    //mirroring
    pub mirroring: u8,
    //mapper object
    pub mapper: Option<Box<dyn Mappable>>,
}
impl Cartridge{
    //This function will be used to load the ROM into the emulator
    pub fn new()->Self{
        Cartridge{
            prg_rom: vec![],
            chr_rom: vec![],
            mapper_type: MAPPERS::NROM,
            rom: vec![],
            mapper_id: 0,
            prg_rom_size: 0,
            chr_rom_size: 0,
            mirroring: 0,
            mapper: None,
        }
    }

    pub fn load_rom(&mut self, rom: &[u8])->Result<(), Box<dyn Error>>{

        println!("Size of th whole R0M {}",rom.len());
        //Header file check
        //rom is in .nes format so we need to check if the rom is valid,look above for the format
        if rom[0..4] != [0x4E, 0x45, 0x53, 0x1A] {
            return Err("Invalid ROM file".into());
        }
        //load the rom into the struct
        self.rom = rom.to_vec();

        //Size of the PRG ROM in 16 KB units
        self.prg_rom_size = rom[4];

        println!("PRG ROM size: {}", self.prg_rom_size);

        //Size of the CHR ROM in 8 KB units
        self.chr_rom_size = rom[5];

        println!("CHR ROM size: {} KB", self.chr_rom_size);
        if self.chr_rom_size == 0 {
            println!("This ROM uses CHR RAM");
        }

        //Flags 6 - Mapper, mirroring, battery, trainer
        let flags_6 = rom[6];

        println!("Flags 6: {:08b}", flags_6);

        //Flags 7 - Mapper, VS/Playchoice, NES 2.0
        let flags_7 = rom[7];

        println!("Flags 7: {:08b}", flags_7);

        //Flags 8 - PRG-RAM size (rarely used extension)
        let _flags_8 = rom[8];

        //Flags 9 - TV system (rarely used extension)
        let _flags_9 = rom[9];

        //Flags 10 - TV system, PRG-RAM presence (unofficial, rarely used extension)
        let _flags_10 = rom[10];

        //Mapper ID
        self.mapper_id = (flags_7 & 0xF0) | (flags_6 >> 4);

        log::info!("Mapper ID: {}", self.mapper_id);

        println!("Mapper ID: {}", self.mapper_id);


        //set the mapper
        self.get_mapper();
        //Mirroring
        self.mirroring = flags_6 & 0x01 | ((flags_6 >> 3) & 0x01);
        //Check if it contains trainer
        let _trainer = flags_6 & 0x04;
        //load the program rom
        self.prg_rom = rom[16..(16 + (self.prg_rom_size as usize * 16384))].to_vec();

        //load the character rom

        //check the type of file
        //if the file is NES 2.0 then the character rom size is in the flags 8
        if (flags_7 & 0x0C) == 0x08
        {println!("This is a NES 2.0 ROM");
        } //if the file is of type NES 1.0 then the character rom size is in the flags 5
        else{
            println!("This is a NES 1.0 ROM");
            if self.chr_rom_size == 0 {
                //If the ROM uses CHR RAM, the size is 8 KB
                self.chr_rom = vec![0; 8192];
                println!("Size of the remaining ROM {}",rom.len() - (16 + (self.prg_rom_size as usize * 16384)));
                if rom.len()- (16 + (self.prg_rom_size as usize * 16384)) > 0 {
                    //load the remaining data into the character ram
                    self.chr_rom[0..rom.len()- (16 + (self.prg_rom_size as usize * 16384))].copy_from_slice(&rom[(16 + (self.prg_rom_size as usize * 16384))..]);
                }
            } else {
                //If the ROM uses CHR ROM, the size is 8 KB * chr_rom_size
                self.chr_rom = rom[(16 + (self.prg_rom_size as usize * 16384))..(16 + (self.prg_rom_size as usize * 16384) + (self.chr_rom_size as usize * 8192))].to_vec();
            }
        }
        Ok(())
    }

    pub fn get_mapper(&mut self){
        match self.mapper_id {
            0 => {
                self.mapper_type = MAPPERS::NROM;
                self.mapper = Some(Box::new(NROM::new(self.prg_rom_size, self.chr_rom_size)));
            }
           1 => {
                self.mapper_type = MAPPERS::MMC1;
                self.mapper = Some(Box::new(MMC1::new(self.prg_rom_size, self.chr_rom_size)));
            }
            /*
            4 => {
                self.mapper_type = MAPPERS::MMC3;
                self.mapper = Some(MMC3::new(self.prg_rom_size, self.chr_rom_size));
            }

            */
            _ => {
                panic!("Mapper not supported");
            }
        }
    }


    pub fn read_cpu(&self,address:u16) -> Result<u8,Box<dyn Error>>{
        //check if the mapper is loaded
        match &self.mapper {
            Some(mapper) => {
                //read the cpu
                match mapper.map_read_cpu(address) {
                    Ok(data) =>{
                        if data.0==0xFFFFFFFF {
                            //the mapper already provides the data
                            return Ok(data.1);
                        }
                        //log::info!("Reading from PRG ROM at address: {:04X}", data.0);
                        Ok(self.prg_rom[data.0 as usize])
                    }
                    Err(e) => {
                        println!("Error reading CPU : {}", e);
                        Err(e)
                    }
                }
            }
            None => {
                Err("Mapper not loaded".into())
            }
        }
    }

    pub fn write_cpu(&mut self,address:u16,data:u8) -> Result<(),Box<dyn Error>> {
        //check if the mapper is loaded
        match &mut self.mapper {
            Some(mapper) => {
                //write the cpu
                match mapper.map_write_cpu(address,data) {
                    Ok(_) => {
                        //write the cpu

                        Ok(())
                    }
                    Err(e) => {
                        Err(e)
                    }
                }
            }
            None => {
                Err("Mapper not loaded".into())
            }
        }

    }

    pub fn read_ppu(&self,address:u16) -> Result<u8,Box<dyn Error>>{

        match &self.mapper {
            Some(mapper) => {
                //read the ppu
                match mapper.map_read_ppu(address) {
                    Ok(data) =>{
                        if data.0==0xFFFFFFFF {
                            //the mapper already provides the data
                            return Ok(data.1);
                        }
                        //log::info!("Reading from CHR ROM at address: {:04X}", data.0);
                        if self.chr_rom_size==0 {
                            //if the rom has no chr rom then return 0
                             //println!("No character rom, the board used character RAM");
                            Ok(0)
                        }
                        else{
                            Ok(self.chr_rom[data.0 as usize])
                        }
                    }
                    Err(e) => {
                        println!("Error reading PPU : {}", e);
                        Err(e)
                    }
                }
            }
            None => {
                Err("Mapper not loaded".into())
            }
        }
    }

    pub fn write_ppu(&mut self,address:u16,data:u8) -> Result<(),Box<dyn Error>>{
        match &mut self.mapper {
            Some(mapper) => {
                //write the ppu
                match mapper.map_write_ppu(address,data) {
                    Ok(_) => {
                        //write the ppu

                        Ok(())
                    }
                    Err(e) => {
                        Err(e)
                    }
                }
            }
            None => {
                Err("Mapper not loaded".into())
            }
        }
    }


}

//impliment Default for Cartridge
impl Default for Cartridge {
    fn default() -> Self {
        Self {
            prg_rom_size: 0,
            chr_rom_size: 0,
            mapper_id: 0,
            mirroring: 0,
            prg_rom: Vec::new(),
            chr_rom: Vec::new(),
            mapper: None,
            mapper_type: MAPPERS::NROM,
            rom: vec![]
        }
    }
}