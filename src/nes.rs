use std::error::Error;
//use
use crate::alu::ALU;
use crate::cartridge::Cartridge;
// use crate::ppu::Ppu;
use tudelft_nes_ppu::{Ppu, PpuRegister};
// use crate::ppu::registers::PpuRegister;
//Define the bus struct

//Optional is not yet needed, but will be used later

pub struct Bus{
    //Connected devices
    pub alu: Option<ALU>,
    //Memory
    //The 2kb of RAM on the NES
    pub ram: [u8; 2048],
    pub cartridge: Option<Cartridge>,
    //ppu
    pub ppu: Option<Ppu>,

    //Controller working principle

    //There are two controllers on the NES, each with 8 buttons
    //The buttons are read by the CPU by reading the value of a specific memory address (0x4016 and 0x4017)

    //Standard reading procedure:
    //1. Write 1 to $4016 to signal the controller to poll its input
    //2. Write 0 to $4016 to finish the poll
    //3. Read polled data one bit at a time from $4016 or $4017

    //Controller design:
    //Keep two vectors of size 2 (for each of the controllers) of 8 bits (for each of the buttons)
    //The first vector is the current state of the buttons
    //The second vector is the state of the buttons at the last poll

    //Writing to $4016 or $4017:
    //Just assign the value in the first vector(instantaneous data) to the second vector (last polled data)

    //Reading from $4016 or $4017:
    //Read the first bit of the second vector (last polled data) and shift the vector to the right



    //Controller
    //this is a public interface to the controller
    //this register is update on whenever a button is pressed or released
    pub controller: Vec<u8>,

    //Controller snapshot (internal)
    //this is a private interface to the controller
    //this register makes a snapshot of the controller register when you write to the controller register
    //this is done to prevent the controller register from being updated while you are reading it
    controller_snapshot: Vec<u8>,
}

//Implement the bus struct
impl Bus{
    pub fn new() -> Self{
        Self {
            alu: None,
            ram: [0; 2048],
            cartridge: None,
            controller: vec![0;2],
            controller_snapshot: vec![0;2],
            ppu: None,
        }
    }


    //Connect the ALU to the BUS the system can function without ALU so we just return a warning
    pub fn connect_alu(&mut self, alu: ALU){
        self.alu = Some(alu);
    }
    //Connect the Cartridge to the BUS the system cant exists without Cartridge so we use the panic! macro
    pub fn connect_cartridge(&mut self, cart: Cartridge)->Result<(), Box<dyn Error>>{
        //Check if the Cartridge is already connected
        match self.cartridge{
            //If the Cartridge is already connected we panic
            Some(_) => Err("Cartridge already connected".into()),
            //If the Cartridge is not connected we connect it
            None => {
                self.cartridge = Some(cart);
                Ok(())
            }
        }
    }


    //Read a byte from the CPU bus,
    pub fn read_cpu(&mut self,ppu:Option<&mut Ppu>,address: u16) -> Result<u8, &str>{
        //if the address is in the RAM range

        if address<=0x07FF {
            //Return the value from the RAM
            return Ok(self.ram[address as usize]);
        }
        //if the address is in the RAM mirror range
        else if (0x0800..=0x1FFF).contains(&address) {
            //Return the value from the RAM
            return Ok(self.ram[(address & 0x07FF) as usize]);
        }
        //if the address is in the PPU range
        else if (0x2000..=0x3FFF).contains(&address) {
            return match ppu {
                Some(ppu) => {
                    //Return the value from the PPU
                    match address & 0x007 {
                        0 => Ok(ppu.read_ppu_register_interface(PpuRegister::Controller, self)),
                        1 => Ok(ppu.read_ppu_register_interface(PpuRegister::Mask, self)),
                        2 => Ok(ppu.read_ppu_register_interface(PpuRegister::Status, self)),
                        3 => Ok(ppu.read_ppu_register_interface(PpuRegister::OamAddress, self)),
                        4 => Ok(ppu.read_ppu_register_interface(PpuRegister::OamData, self)),
                        5 => Ok(ppu.read_ppu_register_interface(PpuRegister::Scroll, self)),
                        6 => Ok(ppu.read_ppu_register_interface(PpuRegister::Address, self)),
                        7 => Ok(ppu.read_ppu_register_interface(PpuRegister::Data, self)),
                        _ => Err("Address out of range"),
                    }
                }
                None => {
                    Err("PPU not connected")
                }
            }
        }
        //if the address is in the Cartridge range
        else if address>=0x4020 {
            //Return the value from the Cartridge
            if (0x4020..=0x5FFF).contains(&address){
                return Ok(0);
            }

            return match self.cartridge{
                Some(ref cart) => {
                    match cart.read_cpu(address){
                        Ok(data) => {
                            Ok(data)
                        }
                        Err(_e) => {
                            Err("Cartridge error")
                        }
                    }
                }
                None => {
                    Err("Cartridge not connected")
                }
            }
        }
        //if the address is in the APU range
        else if (0x4000..=0x4015).contains(&address) {
            //Error because the APU is not yet implemented
            return Ok(0);
            //return Err("APU not yet implemented");
        }
        //if the address is in the Controller range
        else if (0x4016..=0x4017).contains(&address) {
            //Read one bit from the controller
           let data = if self.controller_snapshot[(address & 0x0001) as usize] & 0x80>0 {1} else {0};
            //Shift the controller snapshot
            self.controller_snapshot[(address & 0x0001) as usize] <<= 1;
            return Ok(data);
        }
        //In case is out of range we return an error
        Err("Invalid address")

    }
    //Write a byte to the CPU bus
    pub fn write_cpu(&mut self,ppu:Option<&mut Ppu>,address: u16, data: u8)-> Result<(), &str>{



        //if the address is in the RAM range
        if address<=0x07FF {
            //Write the value to the RAM
            self.ram[address as usize] = data;
            return Ok(());
        }
        //if the address is in the RAM mirror range
        else if (0x0800..=0x1FFF).contains(&address) {
            //Write the value to the RAM
            self.ram[(address & 0x07FF) as usize] = data;
            return Ok(());
        }
        //if the address is in the PPU range
        else if (0x2000..=0x3FFF).contains(&address) {
            //Return the error because the PPU is not yet implemented

              match ppu{
                Some(ppu) => {
                    match address & 0x0007{
                        0 => {
                            ppu.write_ppu_register(PpuRegister::Controller,data);
                        }
                        1 => {
                            ppu.write_ppu_register(PpuRegister::Mask,data);
                        }
                        2 => {
                            ppu.write_ppu_register(PpuRegister::Status,data);
                        }
                        3 => {
                            ppu.write_ppu_register(PpuRegister::OamAddress,data);
                        }
                        4 => {
                            ppu.write_ppu_register(PpuRegister::OamData,data);
                        }
                        5 => {
                            ppu.write_ppu_register(PpuRegister::Scroll,data);
                        }
                        6 => {
                            ppu.write_ppu_register(PpuRegister::Address,data);
                        }
                        7 => {
                            ppu.write_ppu_register(PpuRegister::Data,data);
                        }
                        _ => {
                            return Err("Invalid address");
                        }
                    }
                }
                  None=> {
                      return Err("PPU not connected");
                  }
              }
            return Ok(());
        }
        //if the address is in the Cartridge range
        else if address>=0x4020{
            //Write the value to the Cartridge
            if (4020..=0x5FFF).contains(&address){
                return Ok(());
            }
            return match self.cartridge {
                Some(ref mut cart) => {
                    match cart.write_cpu(address, data) {
                        Ok(()) => {
                            Ok(())
                        }
                        Err(_e) => {
                            Err("Cartridge error")
                        }
                    }
                }
                None => {
                    Err("Cartridge not connected")
                }
            }
        }
        //if the address is in the APU range
        else if (0x4000..=0x4015).contains(&address){
            //Error because the APU is not yet implemented
            return Ok(());
            //return Err("APU not yet implemented");
        }
        //if the address is in the Controller range
        else if (0x4016..=0x4017).contains(&address) {
            //Get a snapshot of the controller state
            self.controller_snapshot[(address & 0x0001) as usize] = self.controller[(address & 0x0001) as usize];
            return Ok(());
            // return Err("Controller not yet implemented");
        }
        //In case is out of range we return an error
        Err("Invalid address")
    }


    //Read a byte from the PPU bus
    pub fn read_ppu(&self, address: u16) -> Result<u8,&str>{
          //if the address is in the PPU range
          if address<=0x3FFF{
                //Return the value from the Cartridge
              return match self.cartridge {
                  Some(ref cart) => {
                      match cart.read_ppu(address) {
                          Ok(data) => {
                              Ok(data)
                          }
                          Err(_e) => {
                              Err("Cartridge error")
                          }
                      }
                  }
                  None => {
                      Err("Cartridge not connected")
                  }
              }
          }
          //if the address is in the RAM range
          else if (0x2000..=0x3EFF).contains(&address){
                //Return the value from the RAM
                return Ok(self.ram[(address-0x2000) as usize]);
          }
          //if the address is in the RAM mirror range
          else if (0x3F00..=0x3FFF).contains(&address) {
                //Return the value from the RAM
                return Ok(self.ram[(address-0x3F00) as usize]);
          }
          //In case is out of range we return an error
          Err("Invalid address")
    }
    //Write a byte to the PPU bus
    pub fn write_ppu(&mut self, address: u16, data: u8)-> Result<(), &str>{

            //if the address is in the PPU range
            if address<=0x3FFF{
                    //Write the value to the Cartridge
                   return match self.cartridge{
                     Some(ref mut cart) => {
                        match cart.write_ppu(address, data){
                                Ok(()) => {
                                 Ok(())
                                }
                                Err(_e) => {
                                Err("Cartridge error")
                                }
                        }
                     }
                     None => {
                        Err("Cartridge not connected")
                     }
                    }
            }
            //if the address is in the RAM range
            else if (0x2000..=0x3EFF).contains(&address) {
                    //Write the value to the RAM
                    self.ram[(address-0x2000) as usize] = data;
                    return Ok(());
            }
            //if the address is in the RAM mirror range
            else if (0x3F00..=0x3FFF).contains(&address) {
                    //Write the value to the RAM
                    self.ram[(address-0x3F00) as usize] = data;
                    return Ok(());
            }
            //In case is out of range we return an error
            Err("Invalid address")

    }

}

trait PpuBusInterface {
    fn read_ppu_register_interface(&mut self, register: PpuRegister, bus: &mut Bus) -> u8;
}

impl PpuBusInterface for Ppu {
    fn read_ppu_register_interface(&mut self, register: PpuRegister, bus: &mut Bus) -> u8 {
        match register {
            PpuRegister::Controller => {}
            PpuRegister::Mask => {}
            PpuRegister::Status => {
                let value = self.status_register.read();
                self.bus &= 0b0001_1111;
                self.bus |= value;
                self.scroll_addr_latch = true;
            }
            PpuRegister::OamAddress => {}
            PpuRegister::OamData => {}
            PpuRegister::Scroll => {}
            PpuRegister::Address => {}
            PpuRegister::Data => {
                self.bus = match self.addr.addr {
                    a @ 0..=0x1fff => {
                        let result = self.data_buffer;
                        self.data_buffer = bus.read_ppu(a).unwrap();
                        result
                    }
                    a @ 0x2000..=0x2fff => {
                        let result = self.data_buffer;
                        self.data_buffer = self.vram[self.mirror_address(a) as usize - 0x2000];
                        result
                    }
                    a @ 0x3000..=0x3eff => {
                        let result = self.data_buffer;
                        self.data_buffer =
                            self.vram[self.mirror_address(a - 0x1000) as usize - 0x2000];
                        result
                    }
                    a @ (0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c) => {
                        self.palette_table[a as usize - 0x3f10]
                    }
                    a @ 0x3f00..=0x3fff => self.palette_table[(a as usize - 0x3f00) & 31],
                    x => panic!("address written to data register out of bounds for ppu memory (too big): 0x{x:x}"),
                };

                self.addr.addr += self.controller_register.vram_increment;
                if self.addr.addr > 0x3fff {
                    self.addr.addr &= 0x3fff;
                }
            }
        }
        self.bus
    }
}

//impliment the Default trait for the Bus
impl Default for Bus {
    fn default() -> Self {
        Self {
            alu: None,
            ram: [0; 0x0800],
            cartridge: None,
            ppu: None,
            controller: vec![0; 2],
            controller_snapshot: vec![0; 2],
        }
    }
}