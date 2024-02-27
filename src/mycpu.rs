use std::error::Error;
use tudelft_nes_test::TestableCpu;
use tudelft_nes_ppu::{Cpu, Ppu};
// use crate::cpu::Cpu;
use crate::nes::Bus;
use crate::cartridge::Cartridge;
// use crate::ppu::Ppu;

//Define the FLAGS enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FLAGS {
    //Carry
    C,
    //Zero
    Z,
    //Interrupt
    I,
    //Decimal
    D,
    //Break
    B,
    //Unused
    U,
    //Overflow
    V,
    //Negative
    N,


}

//enums for addressing modes
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AddressingMode {
    //Implied
    IMP,
    //Immediate
    IMM,
    //Zero Page
    ZP0,
    //Zero Page X
    ZPX,
    //Zero Page Y
    ZPY,
    //Relative
    REL,
    //Absolute
    ABS,
    //Absolute X
    ABX,
    //Absolute Y
    ABY,
    //Indirect
    IND,
    //Indexed Indirect
    IZX,
    //Indirect Indexed
    IZY,
}

//enums for opcodes
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OPCODE {
    //ADC
    ADC,
    //AND
    AND,
    //ASL
    ASL,
    //BCC
    BCC,
    //BCS
    BCS,
    //BEQ
    BEQ,
    //BIT
    BIT,
    //BMI
    BMI,
    //BNE
    BNE,
    //BPL
    BPL,
    //BRK
    BRK,
    //BVC
    BVC,
    //BVS
    BVS,
    //CLC
    CLC,
    //CLD
    CLD,
    //CLI
    CLI,
    //CLV
    CLV,
    //CMP
    CMP,
    //CPX
    CPX,
    //CPY
    CPY,
    //DEC
    DEC,
    //DEX
    DEX,
    //DEY
    DEY,
    //EOR
    EOR,
    //INC
    INC,
    //INX
    INX,
    //INY
    INY,
    //JMP
    JMP,
    //JSR
    JSR,
    //LDA
    LDA,
    //LDX
    LDX,
    //LDY
    LDY,
    //LSR
    LSR,
    //NOP
    NOP,
    //ORA
    ORA,
    //PHA
    PHA,
    //PHP
    PHP,
    //PLA
    PLA,
    //PLP
    PLP,
    //ROL
    ROL,
    //ROR
    ROR,
    //RTI
    RTI,
    //RTS
    RTS,
    //SBC
    SBC,
    //SEC
    SEC,
    //SED
    SED,
    //SEI
    SEI,
    //STA
    STA,
    //STX
    STX,
    //STY
    STY,
    //TAX
    TAX,
    //TAY
    TAY,
    //TSX
    TSX,
    //TXA
    TXA,
    //TXS
    TXS,
    //TYA
    TYA,
    //XXX
    Unknown,
    //Unofficial opcodes
    //SKB
    SKB,
    //IGN
    IGN,
    //ISB
    ISB,
    //DCP
    DCP,
    AXS,
    LAS,
    LAX,
    AHX,
    SAX,
    XAA,
    SXA,
    RRA,
    TAS,
    SYA,
    ARR,
    SRE,
    ALR,
    RLA,
    ANC,
    SLO,
    DOP,

}

//add the copy
#[derive(Debug, PartialEq, Clone, Copy)]
struct Instruction{
    //The name of the instruction
    instr_name: &'static str,
    //The opcode of the instruction
    opcode: u8,
    //The addressing mode of the instruction
    addressing_mode: AddressingMode,
    //The number of cycles the instruction takes
    cycles: u8,
    //The function that executes the instruction
    execute: OPCODE,
}


///Define a lookup table for the instructions
// For some addresses there is not an instruction, so we use Unknown for those
//Based on the table from https://www.masswerk.at/6502/6502_instruction_set.html


///Dont even fucking ask me how I got this table, I have never felt so pathetic in my life
///
/// Just get a life dude...

//Format:
//instr_name : name of instruction
//opcode : is the hex code of instruction
//addressing_mode : is the addressing mode of instruction (check the necessary enum for context)
//cycles: is the number of cycles the instruction takes(this does not take into account page crossing)
//execute: is the function that executes the instruction

const LOOKUP : [Instruction;256] = [Instruction{
    instr_name: "BRK",
    opcode: 0x00,
    addressing_mode: AddressingMode::IMP,
    cycles: 7,
    execute: OPCODE::BRK,
},Instruction{
    instr_name: "ORA",
    opcode: 0x01,
    addressing_mode: AddressingMode::IZX,
    cycles: 6,
    execute: OPCODE::ORA,
},Instruction{
    instr_name: "Unknown",
    opcode: 0x02,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::Unknown,
},Instruction{
    instr_name: "SLO",
    opcode: 0x03,
    addressing_mode: AddressingMode::IZX,
    cycles: 8,
    execute: OPCODE::SLO,
},Instruction{
    instr_name: "NOP",
    opcode: 0x04,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "ORA",
    opcode: 0x05,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::ORA,
},Instruction{
    instr_name: "ASL",
    opcode: 0x06,
    addressing_mode: AddressingMode::ZP0,
    cycles: 5,
    execute: OPCODE::ASL,
},Instruction{
    instr_name: "SLO",
    opcode: 0x07,
    addressing_mode: AddressingMode::ZP0,
    cycles: 5,
    execute: OPCODE::SLO,
},Instruction{
    instr_name: "PHP",
    opcode: 0x08,
    addressing_mode: AddressingMode::IMP,
    cycles: 3,
    execute: OPCODE::PHP,
},Instruction{
    instr_name: "ORA",
    opcode: 0x09,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::ORA,
},Instruction{
    instr_name: "ASL",
    opcode: 0x0A,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::ASL,
},Instruction{
    instr_name: "ANC",
    opcode: 0x0B,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::ANC,
},Instruction{
    instr_name: "NOP",
    opcode: 0x0C,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "ORA",
    opcode: 0x0D,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::ORA,
},Instruction{
    instr_name: "ASL",
    opcode: 0x0E,
    addressing_mode: AddressingMode::ABS,
    cycles: 6,
    execute: OPCODE::ASL,
},Instruction{
    instr_name: "SLO",
    opcode: 0x0F,
    addressing_mode: AddressingMode::ABS,
    cycles: 6,
    execute: OPCODE::SLO,
},Instruction{
    instr_name: "BPL",
    opcode: 0x10,
    addressing_mode: AddressingMode::REL,
    cycles: 2,
    execute: OPCODE::BPL,
},Instruction{
    instr_name: "ORA",
    opcode: 0x11,
    addressing_mode: AddressingMode::IZY,
    cycles: 5,
    execute: OPCODE::ORA,
},Instruction{
    instr_name: "Unknown",
    opcode: 0x12,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::Unknown,
},Instruction{
    instr_name: "SLO",
    opcode: 0x13,
    addressing_mode: AddressingMode::IZY,
    cycles: 8,
    execute: OPCODE::SLO,
},Instruction{
    instr_name: "NOP",
    opcode: 0x14,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "ORA",
    opcode: 0x15,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::ORA,
},Instruction{
    instr_name: "ASL",
    opcode: 0x16,
    addressing_mode: AddressingMode::ZPX,
    cycles: 6,
    execute: OPCODE::ASL,
},Instruction{
    instr_name: "SLO",
    opcode: 0x17,
    addressing_mode: AddressingMode::ZPX,
    cycles: 6,
    execute: OPCODE::SLO,
},Instruction{
    instr_name: "CLC",
    opcode: 0x18,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::CLC,
},Instruction{
    instr_name: "ORA",
    opcode: 0x19,
    addressing_mode: AddressingMode::ABY,
    cycles: 4,
    execute: OPCODE::ORA,
},Instruction{
    instr_name: "NOP",
    opcode: 0x1A,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "SLO",
    opcode: 0x1B,
    addressing_mode: AddressingMode::ABY,
    cycles: 7,
    execute: OPCODE::SLO,
},Instruction{
    instr_name: "IGN",
    opcode: 0x1C,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::IGN,
},Instruction{
    instr_name: "ORA",
    opcode: 0x1D,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::ORA,
},Instruction{
    instr_name: "ASL",
    opcode: 0x1E,
    addressing_mode: AddressingMode::ABX,
    cycles: 7,
    execute: OPCODE::ASL,
},Instruction{
    instr_name: "SLO",
    opcode: 0x1F,
    addressing_mode: AddressingMode::ABX,
    cycles: 7,
    execute: OPCODE::SLO,
},Instruction{
    instr_name: "JSR",
    opcode: 0x20,
    addressing_mode: AddressingMode::ABS,
    cycles: 6,
    execute: OPCODE::JSR,
},Instruction{
    instr_name: "AND",
    opcode: 0x21,
    addressing_mode: AddressingMode::IZX,
    cycles: 6,
    execute: OPCODE::AND,
},Instruction{
    instr_name: "Unknown",
    opcode: 0x22,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::Unknown,
},Instruction{
    instr_name: "RLA",
    opcode: 0x23,
    addressing_mode: AddressingMode::IZX,
    cycles: 8,
    execute: OPCODE::RLA,
},Instruction{
    instr_name: "BIT",
    opcode: 0x24,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::BIT,
},Instruction{
    instr_name: "AND",
    opcode: 0x25,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::AND,
},Instruction{
    instr_name: "ROL",
    opcode: 0x26,
    addressing_mode: AddressingMode::ZP0,
    cycles: 5,
    execute: OPCODE::ROL,
},Instruction{
    instr_name: "RLA",
    opcode: 0x27,
    addressing_mode: AddressingMode::ZP0,
    cycles: 5,
    execute: OPCODE::RLA,
},Instruction{
    instr_name: "PLP",
    opcode: 0x28,
    addressing_mode: AddressingMode::IMP,
    cycles: 4,
    execute: OPCODE::PLP,
},Instruction{
    instr_name: "AND",
    opcode: 0x29,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::AND,
},Instruction{
    instr_name: "ROL",
    opcode: 0x2A,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::ROL,
},Instruction{
    instr_name: "ANC",
    opcode: 0x2B,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::ANC,
},Instruction{
    instr_name: "BIT",
    opcode: 0x2C,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::BIT,
},Instruction{
    instr_name: "AND",
    opcode: 0x2D,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::AND,
},Instruction{
    instr_name: "ROL",
    opcode: 0x2E,
    addressing_mode: AddressingMode::ABS,
    cycles: 6,
    execute: OPCODE::ROL,
},Instruction{
    instr_name: "RLA",
    opcode: 0x2F,
    addressing_mode: AddressingMode::ABS,
    cycles: 6,
    execute: OPCODE::RLA,
},Instruction{
    instr_name: "BMI",
    opcode: 0x30,
    addressing_mode: AddressingMode::REL,
    cycles: 2,
    execute: OPCODE::BMI,
},Instruction{
    instr_name: "AND",
    opcode: 0x31,
    addressing_mode: AddressingMode::IZY,
    cycles: 5,
    execute: OPCODE::AND,
},Instruction{
    instr_name: "Unknown",
    opcode: 0x32,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::Unknown,
},Instruction{
    instr_name: "RLA",
    opcode: 0x33,
    addressing_mode: AddressingMode::IZY,
    cycles: 8,
    execute: OPCODE::RLA,
},Instruction{
    instr_name: "NOP",
    opcode: 0x34,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "AND",
    opcode: 0x35,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::AND,
},Instruction{
    instr_name: "ROL",
    opcode: 0x36,
    addressing_mode: AddressingMode::ZPX,
    cycles: 6,
    execute: OPCODE::ROL,
},Instruction{
    instr_name: "RLA",
    opcode: 0x37,
    addressing_mode: AddressingMode::ZPX,
    cycles: 6,
    execute: OPCODE::RLA,
},Instruction{
    instr_name: "SEC",
    opcode: 0x38,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::SEC,
},Instruction{
    instr_name: "AND",
    opcode: 0x39,
    addressing_mode: AddressingMode::ABY,
    cycles: 4,
    execute: OPCODE::AND,
},Instruction{
    instr_name: "NOP",
    opcode: 0x3A,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "RLA",
    opcode: 0x3B,
    addressing_mode: AddressingMode::ABY,
    cycles: 2,
    execute: OPCODE::RLA,
},Instruction{
    instr_name: "NOP",
    opcode: 0x3C,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "AND",
    opcode: 0x3D,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::AND,
},Instruction{
    instr_name: "ROL",
    opcode: 0x3E,
    addressing_mode: AddressingMode::ABX,
    cycles: 7,
    execute: OPCODE::ROL,
},Instruction{
    instr_name: "RLA",
    opcode: 0x3F,
    addressing_mode: AddressingMode::ABX,
    cycles: 7,
    execute: OPCODE::RLA,
},Instruction{
    instr_name: "RTI",
    opcode: 0x40,
    addressing_mode: AddressingMode::IMP,
    cycles: 6,
    execute: OPCODE::RTI,
},Instruction{
    instr_name: "EOR",
    opcode: 0x41,
    addressing_mode: AddressingMode::IZX,
    cycles: 6,
    execute: OPCODE::EOR,
},Instruction{
    instr_name: "Unknown",
    opcode: 0x42,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::Unknown,
},Instruction{
    instr_name: "SRE",
    opcode: 0x43,
    addressing_mode: AddressingMode::IZX,
    cycles: 8,
    execute: OPCODE::SRE,
},Instruction{
    instr_name: "NOP",
    opcode: 0x44,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "EOR",
    opcode: 0x45,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::EOR,
},Instruction{
    instr_name: "LSR",
    opcode: 0x46,
    addressing_mode: AddressingMode::ZP0,
    cycles: 5,
    execute: OPCODE::LSR,
},Instruction{
    instr_name: "SRE",
    opcode: 0x47,
    addressing_mode: AddressingMode::ZP0,
    cycles: 5,
    execute: OPCODE::SRE,
},Instruction{
    instr_name: "PHA",
    opcode: 0x48,
    addressing_mode: AddressingMode::IMP,
    cycles: 3,
    execute: OPCODE::PHA,
},Instruction{
    instr_name: "EOR",
    opcode: 0x49,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::EOR,
},Instruction{
    instr_name: "LSR",
    opcode: 0x4A,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::LSR,
},Instruction{
    instr_name: "ALR",
    opcode: 0x4B,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::ALR,
},Instruction{
    instr_name: "JMP",
    opcode: 0x4C,
    addressing_mode: AddressingMode::ABS,
    cycles: 3,
    execute: OPCODE::JMP,
},Instruction{
    instr_name: "EOR",
    opcode: 0x4D,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::EOR,
},Instruction{
    instr_name: "LSR",
    opcode: 0x4E,
    addressing_mode: AddressingMode::ABS,
    cycles: 6,
    execute: OPCODE::LSR,
},Instruction{
    instr_name: "SRE",
    opcode: 0x4F,
    addressing_mode: AddressingMode::ABS,
    cycles: 6,
    execute: OPCODE::SRE,
},Instruction{
    instr_name: "BVC",
    opcode: 0x50,
    addressing_mode: AddressingMode::REL,
    cycles: 2,
    execute: OPCODE::BVC,
},Instruction{
    instr_name: "EOR",
    opcode: 0x51,
    addressing_mode: AddressingMode::IZY,
    cycles: 5,
    execute: OPCODE::EOR,
},Instruction{
    instr_name: "Unknown",
    opcode: 0x52,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::Unknown,
},Instruction{
    instr_name: "SRE",
    opcode: 0x53,
    addressing_mode: AddressingMode::IZY,
    cycles: 8,
    execute: OPCODE::SRE,
},Instruction{
    instr_name: "NOP",
    opcode: 0x54,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "EOR",
    opcode: 0x55,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::EOR,
},Instruction{
    instr_name: "LSR",
    opcode: 0x56,
    addressing_mode: AddressingMode::ZPX,
    cycles: 6,
    execute: OPCODE::LSR,
},Instruction{
    instr_name: "SRE",
    opcode: 0x57,
    addressing_mode: AddressingMode::ZPX,
    cycles: 6,
    execute: OPCODE::SRE,
},Instruction{
    instr_name: "CLI",
    opcode: 0x58,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::CLI,
},Instruction{
    instr_name: "EOR",
    opcode: 0x59,
    addressing_mode: AddressingMode::ABY,
    cycles: 4,
    execute: OPCODE::EOR,
},Instruction{
    instr_name: "NOP",
    opcode: 0x5A,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "SRE",
    opcode: 0x5B,
    addressing_mode: AddressingMode::ABY,
    cycles: 7,
    execute: OPCODE::SRE,
},Instruction{
    instr_name: "NOP",
    opcode: 0x5C,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "EOR",
    opcode: 0x5D,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::EOR,
},Instruction{
    instr_name: "LSR",
    opcode: 0x5E,
    addressing_mode: AddressingMode::ABX,
    cycles: 7,
    execute: OPCODE::LSR,
},Instruction{
    instr_name: "SRE",
    opcode: 0x5F,
    addressing_mode: AddressingMode::ABX,
    cycles: 7,
    execute: OPCODE::SRE,
},Instruction{
    instr_name: "RTS",
    opcode: 0x60,
    addressing_mode: AddressingMode::IMP,
    cycles: 6,
    execute: OPCODE::RTS,
},Instruction{
    instr_name: "ADC",
    opcode: 0x61,
    addressing_mode: AddressingMode::IZX,
    cycles: 6,
    execute: OPCODE::ADC,
},Instruction{
    instr_name: "Unknown",
    opcode: 0x62,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::Unknown,
},Instruction{
    instr_name: "RRA",
    opcode: 0x63,
    addressing_mode: AddressingMode::IZX,
    cycles: 8,
    execute: OPCODE::RRA,
},Instruction{
    instr_name: "NOP",
    opcode: 0x64,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "ADC",
    opcode: 0x65,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::ADC,
},Instruction{
    instr_name: "ROR",
    opcode: 0x66,
    addressing_mode: AddressingMode::ZP0,
    cycles: 5,
    execute: OPCODE::ROR,
},Instruction{
    instr_name: "RRA",
    opcode: 0x67,
    addressing_mode: AddressingMode::ZP0,
    cycles: 5,
    execute: OPCODE::RRA,
},Instruction{
    instr_name: "PLA",
    opcode: 0x68,
    addressing_mode: AddressingMode::IMP,
    cycles: 4,
    execute: OPCODE::PLA,
},Instruction{
    instr_name: "ADC",
    opcode: 0x69,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::ADC,
},Instruction{
    instr_name: "ROR",
    opcode: 0x6A,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::ROR,
},Instruction{
    instr_name: "ARR",
    opcode: 0x6B,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::ARR,
},Instruction{
    instr_name: "JMP",
    opcode: 0x6C,
    addressing_mode: AddressingMode::IND,
    cycles: 5,
    execute: OPCODE::JMP,
},Instruction{
    instr_name: "ADC",
    opcode: 0x6D,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::ADC,
},Instruction{
    instr_name: "ROR",
    opcode: 0x6E,
    addressing_mode: AddressingMode::ABS,
    cycles: 6,
    execute: OPCODE::ROR,
},Instruction{
    instr_name: "RRA",
    opcode: 0x6F,
    addressing_mode: AddressingMode::ABS,
    cycles: 6,
    execute: OPCODE::RRA,
},Instruction{
    instr_name: "BVS",
    opcode: 0x70,
    addressing_mode: AddressingMode::REL,
    cycles: 2,
    execute: OPCODE::BVS,
},Instruction{
    instr_name: "ADC",
    opcode: 0x71,
    addressing_mode: AddressingMode::IZY,
    cycles: 5,
    execute: OPCODE::ADC,
},Instruction{
    instr_name: "Unknown",
    opcode: 0x72,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::Unknown,
},Instruction{
    instr_name: "RRA",
    opcode: 0x73,
    addressing_mode: AddressingMode::IZY,
    cycles: 8,
    execute: OPCODE::RRA,
},Instruction{
    instr_name: "NOP",
    opcode: 0x74,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "ADC",
    opcode: 0x75,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::ADC,
},Instruction{
    instr_name: "ROR",
    opcode: 0x76,
    addressing_mode: AddressingMode::ZPX,
    cycles: 6,
    execute: OPCODE::ROR,
},Instruction{
    instr_name: "RRA",
    opcode: 0x77,
    addressing_mode: AddressingMode::ZPX,
    cycles: 6,
    execute: OPCODE::RRA,
},Instruction{
    instr_name: "SEI",
    opcode: 0x78,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::SEI,
},Instruction{
    instr_name: "ADC",
    opcode: 0x79,
    addressing_mode: AddressingMode::ABY,
    cycles: 4,
    execute: OPCODE::ADC,
},Instruction{
    instr_name: "NOP",
    opcode: 0x7A,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "RRA",
    opcode: 0x7B,
    addressing_mode: AddressingMode::ABY,
    cycles: 7,
    execute: OPCODE::RRA,
},Instruction{
    instr_name: "NOP",
    opcode: 0x7C,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "ADC",
    opcode: 0x7D,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::ADC,
},Instruction{
    instr_name: "ROR",
    opcode: 0x7E,
    addressing_mode: AddressingMode::ABX,
    cycles: 7,
    execute: OPCODE::ROR,
},Instruction{
    instr_name: "RRA",
    opcode: 0x7F,
    addressing_mode: AddressingMode::ABX,
    cycles: 7,
    execute: OPCODE::RRA,
},Instruction{
    instr_name: "NOP",
    opcode: 0x80,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "STA",
    opcode: 0x81,
    addressing_mode: AddressingMode::IZX,
    cycles: 6,
    execute: OPCODE::STA,
},Instruction{
    instr_name: "NOP",
    opcode: 0x82,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "SAX",
    opcode: 0x83,
    addressing_mode: AddressingMode::IZX,
    cycles: 6,
    execute: OPCODE::SAX,
},Instruction{
    instr_name: "STY",
    opcode: 0x84,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::STY,
},Instruction{
    instr_name: "STA",
    opcode: 0x85,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::STA,
},Instruction{
    instr_name: "STX",
    opcode: 0x86,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::STX,
},Instruction{
    instr_name: "SAX",
    opcode: 0x87,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::SAX,
},Instruction{
    instr_name: "DEY",
    opcode: 0x88,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::DEY,
},Instruction{
    instr_name: "NOP",
    opcode: 0x89,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "TXA",
    opcode: 0x8A,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::TXA,
},Instruction{
    instr_name: "XAA",
    opcode: 0x8B,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::XAA,
},Instruction{
    instr_name: "STY",
    opcode: 0x8C,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::STY,
},Instruction{
    instr_name: "STA",
    opcode: 0x8D,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::STA,
},Instruction{
    instr_name: "STX",
    opcode: 0x8E,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::STX,
},Instruction{
    instr_name: "SAX",
    opcode: 0x8F,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::SAX,
},Instruction{
    instr_name: "BCC",
    opcode: 0x90,
    addressing_mode: AddressingMode::REL,
    cycles: 2,
    execute: OPCODE::BCC,
},Instruction{
    instr_name: "STA",
    opcode: 0x91,
    addressing_mode: AddressingMode::IZY,
    cycles: 6,
    execute: OPCODE::STA,
},Instruction{
    instr_name: "Unknown",
    opcode: 0x92,
    addressing_mode: AddressingMode::IMP,
    cycles: 0,
    execute: OPCODE::Unknown,
},Instruction{
    instr_name: "AHX",
    opcode: 0x93,
    addressing_mode: AddressingMode::IZY,
    cycles: 6,
    execute: OPCODE::AHX,
},Instruction{
    instr_name: "STY",
    opcode: 0x94,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::STY,
},Instruction{
    instr_name: "STA",
    opcode: 0x95,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::STA,
},Instruction{
    instr_name: "STX",
    opcode: 0x96,
    addressing_mode: AddressingMode::ZPY,
    cycles: 4,
    execute: OPCODE::STX,
},Instruction{
    instr_name: "SAX",
    opcode: 0x97,
    addressing_mode: AddressingMode::ZPY,
    cycles: 4,
    execute: OPCODE::SAX,
},Instruction{
    instr_name: "TYA",
    opcode: 0x98,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::TYA,
},Instruction{
    instr_name: "STA",
    opcode: 0x99,
    addressing_mode: AddressingMode::ABY,
    cycles: 5,
    execute: OPCODE::STA,
},Instruction{
    instr_name: "TXS",
    opcode: 0x9A,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::TXS,
},Instruction{
    instr_name: "TAS",
    opcode: 0x9B,
    addressing_mode: AddressingMode::ABY,
    cycles: 5,
    execute: OPCODE::TAS,
},Instruction{
    instr_name: "SYA",
    opcode: 0x9C,
    addressing_mode: AddressingMode::ABX,
    cycles: 5,
    execute: OPCODE::SYA,
},Instruction{
    instr_name: "STA",
    opcode: 0x9D,
    addressing_mode: AddressingMode::ABX,
    cycles: 5,
    execute: OPCODE::STA,
},Instruction{
    instr_name: "SXA",
    opcode: 0x9E,
    addressing_mode: AddressingMode::ABY,
    cycles: 5,
    execute: OPCODE::SXA,
},Instruction{
    instr_name: "AHX",
    opcode: 0x9F,
    addressing_mode: AddressingMode::ABY,
    cycles: 5,
    execute: OPCODE::AHX,
},Instruction{
    instr_name: "LDY",
    opcode: 0xA0,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::LDY,
},Instruction{
    instr_name: "LDA",
    opcode: 0xA1,
    addressing_mode: AddressingMode::IZX,
    cycles: 6,
    execute: OPCODE::LDA,
},Instruction{
    instr_name: "LDX",
    opcode: 0xA2,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::LDX,
},Instruction{
    instr_name: "LAX",
    opcode: 0xA3,
    addressing_mode: AddressingMode::IZX,
    cycles: 6,
    execute: OPCODE::LAX,
},Instruction{
    instr_name: "LDY",
    opcode: 0xA4,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::LDY,
},Instruction{
    instr_name: "LDA",
    opcode: 0xA5,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::LDA,
},Instruction{
    instr_name: "LDX",
    opcode: 0xA6,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::LDX,
},Instruction{
    instr_name: "LAX",
    opcode: 0xA7,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::LAX,
},Instruction{
    instr_name: "TAY",
    opcode: 0xA8,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::TAY,
},Instruction{
    instr_name: "LDA",
    opcode: 0xA9,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::LDA,
},Instruction{
    instr_name: "TAX",
    opcode: 0xAA,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::TAX,
},Instruction{
    instr_name: "LAX",
    opcode: 0xAB,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::LAX,
},Instruction{
    instr_name: "LDY",
    opcode: 0xAC,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::LDY,
},Instruction{
    instr_name: "LDA",
    opcode: 0xAD,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::LDA,
},Instruction{
    instr_name: "LDX",
    opcode: 0xAE,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::LDX,
},Instruction{
    instr_name: "LAX",
    opcode: 0xAF,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::LAX,
},Instruction{
    instr_name: "BCS",
    opcode: 0xB0,
    addressing_mode: AddressingMode::REL,
    cycles: 2,
    execute: OPCODE::BCS,
},Instruction{
    instr_name: "LDA",
    opcode: 0xB1,
    addressing_mode: AddressingMode::IZY,
    cycles: 5,
    execute: OPCODE::LDA,
},Instruction{
    instr_name: "Unknown",
    opcode: 0xB2,
    addressing_mode: AddressingMode::IMP,
    cycles: 0,
    execute: OPCODE::Unknown,
},Instruction{
    instr_name: "LAX",
    opcode: 0xB3,
    addressing_mode: AddressingMode::IZY,
    cycles: 5,
    execute: OPCODE::LAX,
},Instruction{
    instr_name: "LDY",
    opcode: 0xB4,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::LDY,
},Instruction{
    instr_name: "LDA",
    opcode: 0xB5,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::LDA,
},Instruction{
    instr_name: "LDX",
    opcode: 0xB6,
    addressing_mode: AddressingMode::ZPY,
    cycles: 4,
    execute: OPCODE::LDX,
},Instruction{
    instr_name: "LAX",
    opcode: 0xB7,
    addressing_mode: AddressingMode::ZPY,
    cycles: 4,
    execute: OPCODE::LAX,
},Instruction{
    instr_name: "CLV",
    opcode: 0xB8,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::CLV,
},Instruction{
    instr_name: "LDA",
    opcode: 0xB9,
    addressing_mode: AddressingMode::ABY,
    cycles: 4,
    execute: OPCODE::LDA,
},Instruction{
    instr_name: "TSX",
    opcode: 0xBA,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::TSX,
},Instruction{
    instr_name: "Unknown",
    opcode: 0xBB,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::Unknown,
},Instruction{
    instr_name: "LDY",
    opcode: 0xBC,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::LDY,
},Instruction{
    instr_name: "LDA",
    opcode: 0xBD,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::LDA,
},Instruction{
    instr_name: "LDX",
    opcode: 0xBE,
    addressing_mode: AddressingMode::ABY,
    cycles: 4,
    execute: OPCODE::LDX,
},Instruction{
    instr_name: "LAX",
    opcode: 0xBF,
    addressing_mode: AddressingMode::ABY,
    cycles: 4,
    execute: OPCODE::LAX,
},Instruction{
    instr_name: "CPY",
    opcode: 0xC0,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::CPY,
},Instruction{
    instr_name: "CMP",
    opcode: 0xC1,
    addressing_mode: AddressingMode::IZX,
    cycles: 6,
    execute: OPCODE::CMP,
},Instruction{
    instr_name: "DOP",
    opcode: 0xC2,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::DOP,
},Instruction{
    instr_name: "DCP",
    opcode: 0xC3,
    addressing_mode: AddressingMode::IZX,
    cycles: 8,
    execute: OPCODE::DCP,
},Instruction{
    instr_name: "CPY",
    opcode: 0xC4,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::CPY,
},Instruction{
    instr_name: "CMP",
    opcode: 0xC5,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::CMP,
},Instruction{
    instr_name: "DEC",
    opcode: 0xC6,
    addressing_mode: AddressingMode::ZP0,
    cycles: 5,
    execute: OPCODE::DEC,
},Instruction{
    instr_name: "DCP",
    opcode: 0xC7,
    addressing_mode: AddressingMode::ZP0,
    cycles: 5,
    execute: OPCODE::DCP,
},Instruction{
    instr_name: "INY",
    opcode: 0xC8,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::INY,
},Instruction{
    instr_name: "CMP",
    opcode: 0xC9,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::CMP,
},Instruction{
    instr_name: "DEX",
    opcode: 0xCA,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::DEX,
},Instruction{
    instr_name: "AXS",
    opcode: 0xCB,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::AXS,
},Instruction{
    instr_name: "CPY",
    opcode: 0xCC,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::CPY,
},Instruction{
    instr_name: "CMP",
    opcode: 0xCD,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::CMP,
},Instruction{
    instr_name: "DEC",
    opcode: 0xCE,
    addressing_mode: AddressingMode::ABS,
    cycles: 6,
    execute: OPCODE::DEC,
},Instruction{
    instr_name: "DCP",
    opcode: 0xCF,
    addressing_mode: AddressingMode::ABS,
    cycles: 6,
    execute: OPCODE::DCP,
},Instruction{
    instr_name: "BNE",
    opcode: 0xD0,
    addressing_mode: AddressingMode::REL,
    cycles: 2,
    execute: OPCODE::BNE,
},Instruction{
    instr_name: "CMP",
    opcode: 0xD1,
    addressing_mode: AddressingMode::IZY,
    cycles: 5,
    execute: OPCODE::CMP,
},Instruction{
    instr_name: "Unknown",
    opcode: 0xD2,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::Unknown,
},Instruction{
    instr_name: "DCP",
    opcode: 0xD3,
    addressing_mode: AddressingMode::IZY,
    cycles: 8,
    execute: OPCODE::DCP,
},Instruction{
    instr_name: "NOP",
    opcode: 0xD4,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "CMP",
    opcode: 0xD5,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::CMP,
},Instruction{
    instr_name: "DEC",
    opcode: 0xD6,
    addressing_mode: AddressingMode::ZPX,
    cycles: 6,
    execute: OPCODE::DEC,
},Instruction{
    instr_name: "DCP",
    opcode: 0xD7,
    addressing_mode: AddressingMode::ZPX,
    cycles: 6,
    execute: OPCODE::DCP,
},Instruction{
    instr_name: "CLD",
    opcode: 0xD8,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::CLD,
},Instruction{
    instr_name: "CMP",
    opcode: 0xD9,
    addressing_mode: AddressingMode::ABY,
    cycles: 4,
    execute: OPCODE::CMP,
},Instruction{
    instr_name: "NOP",
    opcode: 0xDA,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "DCP",
    opcode: 0xDB,
    addressing_mode: AddressingMode::ABY,
    cycles: 7,
    execute: OPCODE::DCP,
},Instruction{
    instr_name: "NOP",
    opcode: 0xDC,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "CMP",
    opcode: 0xDD,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::CMP,
},Instruction{
    instr_name: "DEC",
    opcode: 0xDE,
    addressing_mode: AddressingMode::ABX,
    cycles: 7,
    execute: OPCODE::DEC,
},Instruction{
    instr_name: "DCP",
    opcode: 0xDF,
    addressing_mode: AddressingMode::ABX,
    cycles: 7,
    execute: OPCODE::DCP,
},Instruction{
    instr_name: "CPX",
    opcode: 0xE0,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::CPX,
},Instruction{
    instr_name: "SBC",
    opcode: 0xE1,
    addressing_mode: AddressingMode::IZX,
    cycles: 6,
    execute: OPCODE::SBC,
},Instruction{
    instr_name: "NOP",
    opcode: 0xE2,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "ISB",
    opcode: 0xE3,
    addressing_mode: AddressingMode::IZX,
    cycles: 8,
    execute: OPCODE::ISB,
},Instruction{
    instr_name: "CPX",
    opcode: 0xE4,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::CPX,
},Instruction{
    instr_name: "SBC",
    opcode: 0xE5,
    addressing_mode: AddressingMode::ZP0,
    cycles: 3,
    execute: OPCODE::SBC,
},Instruction{
    instr_name: "INC",
    opcode: 0xE6,
    addressing_mode: AddressingMode::ZP0,
    cycles: 5,
    execute: OPCODE::INC,
},Instruction{
    instr_name: "ISB",
    opcode: 0xE7,
    addressing_mode: AddressingMode::ZP0,
    cycles: 5,
    execute: OPCODE::ISB,
},Instruction{
    instr_name: "INX",
    opcode: 0xE8,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::INX,
},Instruction{
    instr_name: "SBC",
    opcode: 0xE9,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::SBC,
},Instruction{
    instr_name: "NOP",
    opcode: 0xEA,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "SBC",
    opcode: 0xEB,
    addressing_mode: AddressingMode::IMM,
    cycles: 2,
    execute: OPCODE::SBC,
},Instruction{
    instr_name: "CPX",
    opcode: 0xEC,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::CPX,
},Instruction{
    instr_name: "SBC",
    opcode: 0xED,
    addressing_mode: AddressingMode::ABS,
    cycles: 4,
    execute: OPCODE::SBC,
},Instruction{
    instr_name: "INC",
    opcode: 0xEE,
    addressing_mode: AddressingMode::ABS,
    cycles: 6,
    execute: OPCODE::INC,
},Instruction{
    instr_name: "ISB",
    opcode: 0xEF,
    addressing_mode: AddressingMode::ABS,
    cycles: 6,
    execute: OPCODE::ISB,
},Instruction{
    instr_name: "BEQ",
    opcode: 0xF0,
    addressing_mode: AddressingMode::REL,
    cycles: 2,
    execute: OPCODE::BEQ,
},Instruction{
    instr_name: "SBC",
    opcode: 0xF1,
    addressing_mode: AddressingMode::IZY,
    cycles: 5,
    execute: OPCODE::SBC,
},Instruction{
    instr_name: "Unknown",
    opcode: 0xF2,
    addressing_mode: AddressingMode::IMP,
    cycles: 0,
    execute: OPCODE::Unknown,
},Instruction{
    instr_name: "ISB",
    opcode: 0xF3,
    addressing_mode: AddressingMode::IZY,
    cycles: 8,
    execute: OPCODE::ISB,
},Instruction{
    instr_name: "NOP",
    opcode: 0xF4,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "SBC",
    opcode: 0xF5,
    addressing_mode: AddressingMode::ZPX,
    cycles: 4,
    execute: OPCODE::SBC,
},Instruction{
    instr_name: "INC",
    opcode: 0xF6,
    addressing_mode: AddressingMode::ZPX,
    cycles: 6,
    execute: OPCODE::INC,
},Instruction{
    instr_name: "ISB",
    opcode: 0xF7,
    addressing_mode: AddressingMode::ZPX,
    cycles: 6,
    execute: OPCODE::ISB,
},Instruction{
    instr_name: "SED",
    opcode: 0xF8,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::SED,
},Instruction{
    instr_name: "SBC",
    opcode: 0xF9,
    addressing_mode: AddressingMode::ABY,
    cycles: 4,
    execute: OPCODE::SBC,
},Instruction{
    instr_name: "NOP",
    opcode: 0xFA,
    addressing_mode: AddressingMode::IMP,
    cycles: 2,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "ISB",
    opcode: 0xFB,
    addressing_mode: AddressingMode::ABY,
    cycles: 7,
    execute: OPCODE::ISB,
},Instruction{
    instr_name: "NOP",
    opcode: 0xFC,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::NOP,
},Instruction{
    instr_name: "SBC",
    opcode: 0xFD,
    addressing_mode: AddressingMode::ABX,
    cycles: 4,
    execute: OPCODE::SBC,
},Instruction{
    instr_name: "INC",
    opcode: 0xFE,
    addressing_mode: AddressingMode::ABX,
    cycles: 7,
    execute: OPCODE::INC,
},Instruction{
    instr_name: "ISB",
    opcode: 0xFF,
    addressing_mode: AddressingMode::ABX,
    cycles: 7,
    execute: OPCODE::ISB,
}];


/// The CPU of the NES
/// Based on the 6502 CPU for reference (https://www.nesdev.org/wiki/CPU)
pub struct CPU{
    //Registers
    //Accumulator Register (8 bits) - A
    pub accumulator: u8,
    //Index Register X (8 bits) - X
    pub x: u8,
    //Index Register Y (8 bits) - Y
    pub y: u8,
    //Stack Pointer (8 bits) - S
    pub stack_pointer: u8,
    //Program Counter (16 bits) - PC
    pub program_counter: u16,
    //Status register
    pub status: u8,

    //Private variables used for the CPU
    //Current instruction (8 bits)
    current_instruction: u8,
    //Current address(this is the address of the data that is being operated on)
    current_address: u16,
    //Current cycles (this is the number of cycles that the current instruction has left to run)
    current_cycles: u8,
    //current fetched data (this is the data that is being operated on)
    current_fetched_data: u8,
    //branch address (this is the address that the program counter will be set to when a branch occurs)
    branch_address: u16,

    //needs additional cycle
    additional_cycle: bool,
    //current instruction set
    curr_instr_set: Instruction,

    //total number of cycles that the CPU has run
    pub total_cycles: u64,

    //Bus that the CPU is connected to
    //The CPU takes ownership of the bus
    pub bus: Option<Bus>,
}

//Implement the CPU
impl CPU{
    pub(crate) fn new() ->Self{
        CPU{
            accumulator: 0,
            x: 0,
            y: 0,
            stack_pointer: 0xFD,
            program_counter: 0x8000,
            status: 0x00,
            current_instruction: 0,
            current_address: 0,
            current_cycles: 0,
            current_fetched_data: 0,
            branch_address: 0,
            additional_cycle: false,
            curr_instr_set: Instruction{
                instr_name: "Unknown",
                opcode: 0x00,
                addressing_mode: AddressingMode::IMP,
                cycles: 0,
                execute: OPCODE::Unknown,
            },
            total_cycles: 0,
            bus: None,
        }
    }
    //Based on https://www.nesdev.org/wiki/CPU_power_up
    pub fn reset(&mut self){
        //Get the address to set the program counter to
        //The address is stored at 0xFFFC and 0xFFFD
        self.program_counter = self.read_memory(None,0xFFFC) as u16 | ((self.read_memory(None,0xFFFC+1) as u16) << 8);

        //Set the program counter to the address stored at 0xFFFC
        //Set the stack pointer to 0xFD
        //Set the status register to 0x24
        //Set the accumulator, X and Y registers to 0
        //Reset the current instruction, current address, current cycles, current fetched data and branch address
        self.accumulator = 0;
        self.x=0;
        self.y=0;
        self.stack_pointer=0xFD;
        self.status=0x24;
        self.current_instruction=0;
        self.current_address=0;
        self.current_cycles=0;
        self.current_fetched_data=0;
        self.branch_address=0;
        //Reset takes 8 cycles
        self.current_cycles=0;
    }
    //In case base is needed
    pub fn connect_bus(&mut self, bus:Bus)->Result<(), String>{
        //check if the bus is already connected
        match self.bus{
            //If the bus is already connected, return an error
            Some(_) => Err(String::from("Bus already connected")),
            //If the bus is not connected, connect it
            None => {
                self.bus = Some(bus);
                Ok(())
            }
        }
    }

}


//get and set the flags
impl CPU{
    //get the flag value based on the flag passed
    fn get_flag(&self, flag: FLAGS) -> bool{
        match flag{
            FLAGS::C => (self.status & 0x01)     !=0,
            FLAGS::Z => (self.status & 0x02) >> 1!=0,
            FLAGS::I => (self.status & 0x04) >> 2!=0,
            FLAGS::D => (self.status & 0x08) >> 3!=0,
            FLAGS::B => (self.status & 0x10) >> 4!=0,
            FLAGS::U => (self.status & 0x20) >> 5!=0,
            FLAGS::V => (self.status & 0x40) >> 6!=0,
            FLAGS::N => (self.status & 0x80) >> 7!=0,
        }
    }
    //set the flag value(0 if false, 1 if true) to the status register
    fn set_flag(&mut self, flag: FLAGS, val: bool){
        let value :u8 = if val {1} else {0};
        match flag{
            FLAGS::C => self.status = (self.status & !0x01) | (value     ),
            FLAGS::Z => self.status = (self.status & !0x02) | (value << 1),
            FLAGS::I => self.status = (self.status & !0x04) | (value << 2),
            FLAGS::D => self.status = (self.status & !0x08) | (value << 3),
            FLAGS::B => self.status = (self.status & !0x10) | (value << 4),
            FLAGS::U => self.status = (self.status & !0x20) | (value << 5),
            FLAGS::V => self.status = (self.status & !0x40) | (value << 6),
            FLAGS::N => self.status = (self.status & !0x80) | (value << 7),
        }
    }
}


//write and read from the bus
impl CPU{
    //Function in case the bus is not connected
    pub fn read_memory(&mut self,ppu:Option<&mut tudelft_nes_ppu::Ppu>, address: u16) ->u8{
        //check if the bus is connected
        match self.bus{
            Some(ref mut bus) =>
            //If the bus is connected, read from the bus
            match bus.read_cpu(ppu,address){
                Ok(data) => data,
                Err(e) => panic!("Error reading from bus: {}", e),
            },
            None => panic!("Bus not connected"),
        }
    }
    //write to the bus
    pub fn write_memory(&mut self,ppu:Option<&mut tudelft_nes_ppu::Ppu>,address: u16, data: u8){
        //check if the bus is connected
        match self.bus{
            Some(ref mut bus) =>
                {    //if the address is 0x4014, send the OAM DMA data to the PPU
                    if address == 0x4014 {
                        //if the address is 0x4014 (PPU OAM DMA), write to the PPU
                        //get the page of the dma address
                        let dma_address = (data as u16) << 8;
                        //vector to store the 256 bytes of data
                        let mut dam_data:[u8;256]=[0;256];
                        //read the data from the bus
                        for i in 0..256{
                            dam_data[i as usize]=bus.read_cpu(None,dma_address+i).unwrap();
                        }
                        //write the data to the PPU
                        ppu.unwrap().write_oam_dma(dam_data);
                    } else {
                        match bus.write_cpu(ppu, address, data) {
                            Ok(_) => (),
                            Err(e) => panic!("Error writing to bus: {}", e),
                        }
                    }
                },
            None => panic!("Bus not connected"),
        }
    }
}
impl CPU{
    //interrupts (NMI, IRQ)
    #[allow(dead_code)]
    fn irq(&mut self){


        //check if the interrupt is disabled
        if !self.get_flag(FLAGS::I){
            //push the program counter to the stack
            self.write_memory(None,0x0100 + self.stack_pointer as u16, ((self.program_counter >> 8) & 0x00FF) as u8);
            self.stack_pointer -= 1;
            self.write_memory(None,0x0100 + self.stack_pointer as u16, (self.program_counter & 0x00FF) as u8);
            self.stack_pointer -= 1;
            //push the status register to the stack
            self.set_flag(FLAGS::B, false);
            self.set_flag(FLAGS::U, true);
            self.set_flag(FLAGS::I, true);
            self.write_memory(None,0x0100 + self.stack_pointer as u16, self.status);
            self.stack_pointer -= 1;
            //set the program counter to the IRQ vector
            self.program_counter = ((self.read_memory(None,0xFFFF) as u16) << 8) | (self.read_memory(None,0xFFFE) as u16);
            //set the cycles
            self.current_cycles = 7;
        }
    }
    fn nmi(&mut self){

        //push the program counter to the stack
        self.write_memory(None,0x0100 + self.stack_pointer as u16, (self.program_counter >> 8) as u8);
        self.stack_pointer =self.stack_pointer.wrapping_sub(1);
        self.write_memory(None,0x0100 + self.stack_pointer as u16, (self.program_counter & 0x00FF) as u8);
        self.stack_pointer =self.stack_pointer.wrapping_sub(1);
        //push the status register to the stack
        self.set_flag(FLAGS::B, false);
        self.set_flag(FLAGS::U, true);
        self.set_flag(FLAGS::I, true);
        self.write_memory(None,0x0100 + self.stack_pointer as u16, self.status);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        //set the program counter to the NMI vector
        self.program_counter = ((self.read_memory(None,0xFFFB) as u16) << 8) | (self.read_memory(None,0xFFFA) as u16);
        //set the cycles
        self.current_cycles = 8;
    }
    ///Execution model:
    //1. Fetch the instruction
    //2. Decode the instruction (get the addressing mode and the operation)
    //3. Execute the instruction
    //4. Increment the program counter
    //5. Increment the cycles
    //6. Fetch the next instruction and repeat

    ///Addressing modes and operations:
    //There are different addressing modes and operations for each instruction (see the instruction set)
    //Because there are different addressing modes for the same operation,its better to have a function for each operation and a function for each addressing mode
    //The addressing mode function will update the current address, so the operation function can use it
    //Finally there are some operations that for some addressing modes need additional cycles, so the operation function will update the current cycles
    fn execute(&mut self,ppu: &mut Ppu){
        //Get the current instruction
        self.curr_instr_set=LOOKUP[self.current_instruction as usize];
        //Set the current cycles to the instruction cycles
        self.current_cycles = self.curr_instr_set.cycles;
        //Set the additional cycles to 0
        self.additional_cycle= false;
        //print the current instruction opcode
        //Call the addressing mode function to get the current address of the data
        self.addressing_mode(ppu);
        //Call the operation function to execute the instruction
        self.operator(ppu).unwrap();
    }
    //Addressing modes
    fn addressing_mode(&mut self,ppu:&mut Ppu){
        match self.curr_instr_set.addressing_mode {
            AddressingMode::IMP => self.implied(),
            AddressingMode::IMM => self.immediate(),
            AddressingMode::ZP0 => self.zero_page(ppu),
            AddressingMode::ZPX => self.zero_page_x(ppu),
            AddressingMode::ZPY => self.zero_page_y(ppu),
            AddressingMode::REL => self.relative(ppu),
            AddressingMode::ABS => self.absolute(ppu),
            AddressingMode::ABX => self.absolute_x(ppu),
            AddressingMode::ABY => self.absolute_y(ppu),
            AddressingMode::IND => self.indirect(ppu),
            AddressingMode::IZX => self.indirect_x(ppu),
            AddressingMode::IZY => self.indirect_y(ppu),
        }
    }
    //Operator function
    fn operator(&mut self,ppu:&mut Ppu)->Result<(),Box<dyn Error>> {
        match self.curr_instr_set.execute {
            OPCODE::ADC => self.adc(ppu),
            OPCODE::AND => self.and(ppu),
            OPCODE::ASL => self.asl(ppu),
            OPCODE::BCC => self.bcc(ppu),
            OPCODE::BCS => self.bcs(),
            OPCODE::BEQ => self.beq(),
            OPCODE::BIT => self.bit(ppu),
            OPCODE::BMI => self.bmi(),
            OPCODE::BNE => self.bne(),
            OPCODE::BPL => self.bpl(),
            OPCODE::BRK => self.brk(ppu),
            OPCODE::BVC => self.bvc(),
            OPCODE::BVS => self.bvs(),
            OPCODE::CLC => self.clc(),
            OPCODE::CLD => self.cld(),
            OPCODE::CLI => self.cli(),
            OPCODE::CLV => self.clv(),
            OPCODE::CMP => self.cmp(ppu),
            OPCODE::CPX => self.cpx(ppu),
            OPCODE::CPY => self.cpy(ppu),
            OPCODE::DEC => self.dec(ppu),
            OPCODE::DEX => self.dex(),
            OPCODE::DEY => self.dey(),
            OPCODE::EOR => self.eor(ppu),
            OPCODE::INC => self.inc(ppu),
            OPCODE::INX => self.inx(),
            OPCODE::INY => self.iny(),
            OPCODE::JMP => self.jmp(),
            OPCODE::JSR => self.jsr(ppu),
            OPCODE::LDA => self.lda(ppu),
            OPCODE::LDX => self.ldx(ppu),
            OPCODE::LDY => self.ldy(ppu),
            OPCODE::LSR => self.lsr(ppu),
            OPCODE::NOP => self.nop(),
            OPCODE::ORA => self.ora(ppu),
            OPCODE::PHA => self.pha(ppu),
            OPCODE::PHP => self.php(ppu),
            OPCODE::PLA => self.pla(ppu),
            OPCODE::PLP => self.plp(ppu),
            OPCODE::ROL => self.rol(ppu),
            OPCODE::ROR => self.ror(ppu),
            OPCODE::RTI => self.rti(ppu),
            OPCODE::RTS => self.rts(),
            OPCODE::SBC => self.sbc(ppu),
            OPCODE::SEC => self.sec(),
            OPCODE::SED => self.sed(),
            OPCODE::SEI => self.sei(),
            OPCODE::STA => self.sta(ppu),
            OPCODE::STX => self.stx(ppu),
            OPCODE::STY => self.sty(ppu),
            OPCODE::TAX => self.tax(),
            OPCODE::TAY => self.tay(),
            OPCODE::TSX => self.tsx(),
            OPCODE::TXA => self.txa(),
            OPCODE::TXS => self.txs(),
            OPCODE::TYA => self.tya(),
            //Unofficial opcodes
            OPCODE::SKB=> self.skb(ppu),
            OPCODE::IGN=> self.ign(ppu),
            OPCODE::ISB=> self.isb(ppu),
            OPCODE::DCP=> self.dcp(ppu),
            OPCODE::AXS=> self.axs(ppu),
            OPCODE::LAS=> self.las(ppu),
            OPCODE::LAX=> self.lax(ppu),
            OPCODE::AHX=> self.ahx(ppu),
            OPCODE::SAX=> self.sax(ppu),
            OPCODE::XAA=> self.xaa(ppu),
            OPCODE::SXA=> self.sxa(ppu),
            OPCODE::RRA=> self.rra(ppu),
            OPCODE::TAS=> self.tas(ppu),
            OPCODE::SYA=> self.sya(ppu),
            OPCODE::ARR=> self.arr(ppu),
            OPCODE::SRE=> self.sre(ppu),
            OPCODE::ALR=> self.alr(ppu),
            OPCODE::RLA=> self.rla(ppu),
            OPCODE::ANC=> self.anc(ppu),
            OPCODE::SLO=> self.slo(ppu),
            OPCODE::DOP=> self.dop(ppu),
            OPCODE::Unknown => self.other(ppu),
        }

        Ok(())
    }


}

/// See docs of `Cpu` for explanations of each function
impl Cpu for CPU{
    fn tick(&mut self,ppu: &mut Ppu) -> Result<(), Box<dyn Error>> {
        //Operations take cycles to complete, and this is important for the cpu
        //In real 6502 hardware the cycles are represented by a clock signal and every cycles corresponds to and operation
        //In this emulator, we will just use a counter to keep track of the cycles and execute the operation when the counter reaches 0
        if self.current_cycles == 0 {
            self.total_cycles += 1;
            //Fetch the next instruction
            self.current_instruction = self.read_memory(Some(ppu),self.program_counter);
                //print the current instruction opcode
                self.program_counter = self.program_counter.wrapping_add(1);

                self.set_flag(FLAGS::U, true);
                //execute the instruction
                self.execute(ppu);
                self.set_flag(FLAGS::U, true);
            } else {
                 //Decrement the cycles
                self.current_cycles -= 1;
            }
        Ok(())

    }

    //read memory from the ppu
    fn ppu_read_chr_rom(&self, offset: u16) -> u8 {
        match self.bus {
            Some(ref bus) =>
                bus.read_ppu(offset).unwrap_or(0),
            None => 0,
        }
    }

    fn non_maskable_interrupt(&mut self) {
        self.nmi();
    }

    fn update_controller(&mut self, controller: u8) {
        //update the controller with the current instance of the controller
        if let Some(ref mut bus) = self.bus {bus.controller[0] = controller;}
    }

}

/// Implementing this trait allows automated tests to be run on your cpu.
/// The crate `tudelft-nes-test` contains all kinds of small and large scale
/// tests to find bugs in your cpu.

impl TestableCpu for CPU{
    fn get_cpu(rom: &[u8]) -> Result<Self, Box<dyn Error>> {


        //if ROM is empty return error
        if rom.is_empty() {
            return Err("ROM is empty".into());
        }


        println!("CPU got called for NESTED TESTS");

        //print a statement to the console
        println!("Creating CPU");
        //Get a cartridge from the ROM
        let mut cartridge =Cartridge::new();
        //Load the ROM into the cartridge
        match cartridge.load_rom(rom){
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

        //first program counter value



        /*
        let mut i=1;
        while i<=10{

        cpu.current_instruction = cpu.read_memory(cpu.program_counter);
       //print the current instruction opcode
       cpu.program_counter += 1;
       cpu.set_flag(FLAGS::U, true);
      cpu.execute();
      cpu.set_flag(FLAGS::U, true);

            i=i+1;
        }
        */


        //Return the CPU
        Ok(cpu)
    }

    fn set_program_counter(&mut self, value: u16) {
        self.program_counter = value;
    }
    //This is for the test to get the current instruction
    #[allow(clippy::cast_ref_to_mut)]
    fn memory_read(&self, address: u16) -> u8 {
       //Convert from imutable self to mutable self
        //use unsafe to convert from immutable to mutable

        //It is sound to do this, it is a bad idea but it is the easiers way to do it
        let self_mut = unsafe { &mut *(self as *const CPU as *mut CPU) };

        //Read the memory at the address
        //if reading from address $6000 to $7FFF, return the value of the RA
        self_mut.read_memory(None, address)
    }
}

/// Implementing the struct for addressing modes
impl CPU{

    //Implied addressing mode
    //In an implied instruction, the data and/or destination is mandatory for the instruction.
    // For example, the CLC instruction is implied, it is going to clear the processor's Carry flag.
    //Address Mode : Implied
    fn implied(&mut self){
        self.current_fetched_data= self.accumulator;
    }
    //These instructions have their data defined as the next byte after the opcode.

    //Address Mode ::Immediate
    //The data is the next byte after the opcode.
    fn immediate(&mut self){
        //set the program counter to the next byte
        self.current_address = self.program_counter;
        //increment the program counter
        self.program_counter= self.program_counter.wrapping_add(1);
    }
    //Zero-Page is an addressing mode that is only capable of addressing the first 256 bytes of the CPU's memory map.
    //Address Mode : Zero Page
    fn zero_page(&mut self,ppu: &mut Ppu){
        //Read the address of the data from the current program counter
        self.current_address= self.read_memory(Some(ppu),self.program_counter) as u16;
        //Increment the program counter
        self.program_counter= self.program_counter.wrapping_add(1);
        //Only the lowest 8 bits are used(first 256 bytes)
        self.current_address &= 0x00FF;

    }
    //Zero page with X offset
    //Address Mode : Zero Page X
    fn zero_page_x(&mut self,ppu: &mut Ppu){
        //Read the address of the data from the current program counter with an offset of the X register
        self.current_address = (self.read_memory(Some(ppu),self.program_counter) as u16 + self.x as u16) & 0x00FF;
        //Increment the program counter
        self.program_counter =self.program_counter.wrapping_add(1);
        //Only the lowest 8 bits are used(first 256 bytes)
    }
    //Zero page with Y offset
    fn zero_page_y(&mut self,ppu: &mut Ppu){
        //Read the address of the data from the current program counter with an offset of the Y register
        self.current_address = self.read_memory(Some(ppu),self.program_counter) as u16 + self.y as u16;
        //Increment the program counter
        self.program_counter = self.program_counter.wrapping_add(1);
        //Only the lowest 8 bits are used(first 256 bytes)
        self.current_address &= 0x00FF;
    }
    //Relative addressing on the 6502 is only used for branch operations. The byte after the opcode is the branch offset.
    // If the branch is taken, the new address will the the current PC plus the offset.
    // The offset is a signed byte, so it can jump a maximum of 127 bytes forward, or 128 bytes backward.
    fn relative(&mut self,ppu: &mut Ppu){
        self.branch_address= self.read_memory(Some(ppu),self.program_counter) as u16;
        self.program_counter=self.program_counter.wrapping_add(1);
        //If the branch address is bigger than 127, it is a negative number
        if (self.branch_address & (1<<7) as u16)!= 0{
            //Set the branch address to the negative number
            self.branch_address |= 0xFF00;
        }

    }
    //Absolute addressing specifies the memory location explicitly in the two bytes following the opcode.
    // So JMP $4032 will set the PC to $4032. The hex for this is 4C 32 40. The 6502 is a little endian machine, so any 16 bit (2 byte) value is stored with the LSB first.
    // All instructions that use absolute addressing are 3 bytes.
    fn absolute(&mut self,ppu: &mut Ppu){
        //Read the address of the data from the current program counter
        self.current_address = self.read_memory(Some(ppu),self.program_counter) as u16;
        //Increment the program counter
        self.program_counter = self.program_counter.wrapping_add(1);
        //Read the address of the data from the current program counter
        self.current_address |= (self.read_memory(Some(ppu),self.program_counter) as u16) << 8;
        //Increment the program counter
        self.program_counter = self.program_counter.wrapping_add(1);
    }
    //Absolute with X offset
    fn absolute_x(&mut self,ppu: &mut Ppu){
        //Read the address of the data from the current program counter
        self.current_address = self.read_memory(Some(ppu),self.program_counter) as u16;
        //Increment the program counter
        self.program_counter = self.program_counter.wrapping_add(1);
        //Read the address of the data from the current program counter
        self.current_address |= (self.read_memory(Some(ppu),self.program_counter) as u16) << 8;
        //Increment the program counter
        self.program_counter = self.program_counter.wrapping_add(1);
        //Add the X register to the address
        self.current_address = self.current_address.wrapping_add(self.x as u16);

        //It is possible that the address jumps to the next page
        //This would require an extra cycle
        if (self.current_address<self.x as u16) || (self.current_address & 0xFF00) != (self.current_address - self.x as u16) & 0xFF00{
            self.additional_cycle = true;
        }

    }
    //Absolute with Y offset
    fn absolute_y(&mut self,ppu: &mut Ppu){
        //Read the address of the data from the current program counter
        self.current_address = self.read_memory(Some(ppu),self.program_counter) as u16;
        //Increment the program counter
        self.program_counter = self.program_counter.wrapping_add(1);
        //Read the address of the data from the current program counter
        self.current_address |= (self.read_memory(Some(ppu),self.program_counter) as u16) << 8;
        //Increment the program counter
        self.program_counter= self.program_counter.wrapping_add(1);
        //Add the Y register to the address
        self.current_address = self.current_address.wrapping_add(self.y as u16);

        //It is possible that the address jumps to the next page
        //This would require an extra cycle
        if (self.current_address <self.y as u16) || (self.current_address & 0xFF00) != (self.current_address - self.y as u16) & 0xFF00{
            self.additional_cycle = true;
        }

    }
    //The JMP instruction is the only instruction that uses this addressing mode.
    // //It is a 3 byte instruction - the 2nd and 3rd bytes are an absolute address.
    // The set the PC to the address stored at that address. So maybe this would be clearer.
    fn indirect(&mut self,ppu: &mut Ppu){

        //Read the address of the data from the current program counter
        let mut address = self.read_memory(Some(ppu),self.program_counter) as u16;
        //Increment the program counter
        self.program_counter =self.program_counter.wrapping_add(1);
        //Read the address of the data from the current program counter
        address |= (self.read_memory(Some(ppu),self.program_counter) as u16) << 8;
        //Increment the program counter
        self.program_counter= self.program_counter.wrapping_add(1);

        //If the address is on the last byte of the page, it will wrap around to the first byte of the page

        //--------------------------Bug in the 6502--------------------------
        if address & 0x00FF == 0x00FF{
            //Read the address of the data from the current program counter
            self.current_address = (self.read_memory(Some(ppu),address & 0xFF00) as u16) << 8;
            //Read the address of the data from the current program counter
            self.current_address |= self.read_memory(Some(ppu),address) as u16;
        }
        else{
            //Read the address of the data from the current program counter
            self.current_address = (self.read_memory(Some(ppu),address + 1) as u16) << 8;
            //Read the address of the data from the current program counter
            self.current_address |= self.read_memory(Some(ppu),address) as u16;
        }
    }
    //This is a 2 byte instruction. The 2nd byte is an address, and the 3rd byte is an offset from that address.
    fn indirect_x(&mut self,ppu: &mut Ppu) {
         //Read the address of the data from the current program counter
        let address = self.read_memory(Some(ppu),self.program_counter) as u16;
         //Increment the program counter
        self.program_counter = self.program_counter.wrapping_add(1);

        //Read the the low byte of the address from the address
        self.current_address = self.read_memory(Some(ppu),(address + self.x as u16) & 0x00FF) as u16;
        //Read the the high byte of the address from the address
        self.current_address |= (self.read_memory(Some(ppu),(address + self.x as u16 + 1) & 0x00FF) as u16) << 8;




    }
    //This is a 2 byte instruction. The 2nd byte is an address, and the 3rd byte is an offset from that address.
    fn indirect_y(&mut self,ppu: &mut Ppu) {
        let address = self.read_memory(Some(ppu),self.program_counter) as u16;
        //Increment the program counter
        self.program_counter = self.program_counter.wrapping_add(1);

        //Read the the low byte of the address from the address
        self.current_address = self.read_memory(Some(ppu),address & 0x00FF) as u16;
        //Read the the high byte of the address from the address
        self.current_address |= (self.read_memory(Some(ppu),(address + 1) & 0x00FF) as u16) << 8;

        //Add the Y register to the address
        self.current_address = self.current_address.wrapping_add(self.y as u16);
        //It is possible that the address jumps to the next page
        //This would require an extra cycle
        if  self.current_address<self.y as u16 ||  (self.current_address & 0xFF00) != (self.current_address - self.y as u16) & 0xFF00{
            self.additional_cycle = true;
        }

    }



}


/// Implementing the struct for instructions
/// The instructions are implemented as functions
/// Based on the instruction set of the 6502 processor http://www.6502.org/tutorials/6502opcodes.html#ASL

/// The functions are named after the instruction
/// In the function explanation A is the accumulator, M is the memory(the data given by the current address), X is the X register and Y is the Y register
impl CPU{
     //A helper function to get the current data
     fn get_current_data(&mut self,ppu: &mut Ppu){
         if self.curr_instr_set.addressing_mode!=AddressingMode::IMP{
             self.current_fetched_data=self.read_memory(Some(ppu),self.current_address);

         }
     }


    ///How the function works:
    //self.current_address is the address of the data which you get from the addressing mode ,you use it to get the current data for the operation
    //you save this data in self.current_fetched_data just in case you need it later

    //self.accumulator is the accumulator register
    ////check this https://www.masswerk.at/6502/6502_instruction_set.html for more info on the operations

    //There is one very important thing,sometimes you will see that some operation have a +1 cycles if the page is crossed
    //If its says that check the self.additional_cycles variable,if its true then add 1 cycle to the current cycles
    //if its not required do not do anything



    //Add memory to accumulator with cary
    //Function: A + M + C -> A, C
    //Affects Flags: N, Z, C, V
    //Addressing Mode: Immediate, Zero Page, Zero Page Indexed, Absolute, Absolute Indexed
    //+ add 1 cycle if page boundary is crossed
    fn adc(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
         self.get_current_data(ppu);
        //add the data to the accumulator

        //It is important to cast the accumulator to u16 to prevent overflow. In nature the accumulator is 8 bits wide so it can only hold values from 0 to 255
        //Carry is added to the sum
        let temp = self.accumulator as u16 + self.current_fetched_data as u16 + self.get_flag(FLAGS::C) as u16;
        //set the carry flag if the sum is bigger than 255 (8 bits)
        self.set_flag(FLAGS::C, temp & 0xFF00 != 0);
        //set the zero flag if the sum is 0
        self.set_flag(FLAGS::Z, (temp & 0x00FF) == 0);
        //set the negative flag if the sum is negative
        self.set_flag(FLAGS::N, (temp & 0x0080)!=0);
        //detect overflow
        //Here is the problem
        //if we work with unsigned numbers then to detect overflow we just need to check if the sum is bigger than 255
        //but what happens with signed numbers?
        //for signed number the value are between -128 and 127

        //The overflow detection is based on this http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html article
        self.set_flag(FLAGS::V, (self.accumulator ^ self.current_fetched_data) & 0x80 == 0 && (self.accumulator ^ temp as u8) & 0x80 != 0);

        //set the data to the accumulator
        self.accumulator = temp as u8;
        //there is a potential page boundary crossing
        if self.additional_cycle {
            self.current_cycles += 1;
        }
    }

    //Instruction: Bitwise AND between memory and accumulator
    //Function: A = A & M
    //Affects Flags: N Z
    //Addressing Mode: Immediate, Zero Page, Zero Page X, Absolute, Absolute X, Absolute Y, Indexed X, Indexed Y
    //+ add 1 cycle if page boundary crossed (this is implemented by the bool return value)
    fn and(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
         self.get_current_data(ppu);
        //and the data with the accumulator
        self.accumulator &= self.current_fetched_data;
        //set the zero flag in case the accumulator is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag in case the accumulator is negative
        //to check for negative values we check the 8th bit of the accumulator
        //THE 8th BIT IS THE SIGN BIT
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
        //there is a potential page boundary crossing which adds 1 cycle
        if self.additional_cycle {
            self.current_cycles += 1;
        }
    }

    //Shift left one bit
    //Function: A = C <- (A << 1) <- 0
    //Affects Flags: N Z C
    //Addressing Mode: Accumulator, Zero Page, Zero Page X, Absolute, Absolute X

    fn asl(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //shift left one bit we need 16 bits so we can easily check the carry bit
        //set the carry flag if the 8th bit is 1
        self.set_flag(FLAGS::C, (self.current_fetched_data >>7) & 1 > 0);
        let result = self.current_fetched_data.wrapping_shl(1);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, result==0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, result & 0x80 != 0);
         //if the addressing mode is accumulator then we need to set the accumulator to the result
        if self.curr_instr_set.addressing_mode == AddressingMode::IMP{
            self.accumulator = result as u8;
        }else{
            self.write_memory(Some(ppu),self.current_address, result as u8);
        }

    }

    //Branch on carry clear
    //Function: if(C == 0) PC = address
    //Affects Flags: none
    //Addressing Mode: Relative,
    fn bcc(&mut self,_ppu: &mut Ppu){

        //check if the carry flag is clear
        if !self.get_flag(FLAGS::C) {
            //add the cycles
            self.current_cycles += 1;

            //get the new address
            self.current_address = self.program_counter.wrapping_add(self.branch_address);
            //check if the page is crossed and add a cycle if it is
            if self.current_address & 0xFF00 != self.program_counter & 0xFF00 {
                self.current_cycles += 1;
            }
            //set the program counter to the new address
            self.program_counter = self.current_address;
        }
    }
    //Branch on carry set
    //Function: branch on C = 1
    fn bcs(&mut self){

        //if the carry flag is set
        if self.get_flag(FLAGS::C) {
            //add 1 cycle
            self.current_cycles += 1;
            //get the new address
            self.current_address = self.program_counter.wrapping_add(self.branch_address);
            //check if the page is crossed and add 1 cycle if it is
            if self.current_address & 0xFF00 != self.program_counter & 0xFF00 {
                self.current_cycles += 1;
            }
            //set the new program counter
            self.program_counter = self.current_address;
        }

    }

    //Branch on result zero
    //Function:branch on Z = 1
    fn beq(&mut self){

           //if the zero flag is set
            if self.get_flag(FLAGS::Z){
            //add 1 cycle
            self.current_cycles += 1;
                //get the new address
            self.current_address = self.program_counter.wrapping_add(self.branch_address);
            //check if the page boundary is crossed and add 1 cycle if it is
            if  self.current_address & 0xFF00 != self.program_counter & 0xFF00 {
                self.current_cycles += 1;
            }
            //set the new program counter
            self.program_counter = self.current_address;
        }
    }
    //Bit test
    //Test Bits in Memory with Accumulator
    //bits 7 and 6 of operand are transferred to bit 7 and 6 of SR (N,V);
    //the zero-flag is set to the result of operand AND accumulator.
    //Function:A AND M, M7 -> N, M6 -> V
    //Flags: N V Z
    fn bit(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, (self.accumulator & self.current_fetched_data) == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.current_fetched_data & (1 << 7) > 0);
        //set the overflow flag if the result is negative
        self.set_flag(FLAGS::V, self.current_fetched_data & (1 << 6) > 0);

    }
    //Branch on result minus
    //Function: branch on N = 1
    //Flags: none
    fn bmi(&mut self){
        //if the negative flag is set
        if self.get_flag(FLAGS::N){
            //add 1 cycle
            self.current_cycles += 1;
            //get the new address
            self.current_address = self.program_counter.wrapping_add(self.branch_address);
            //check if the page boundary is crossed and add 1 cycle if so
            if self.current_address & 0xFF00 != self.program_counter & 0xFF00 {
                self.current_cycles += 1;
            }
            //set the new program counter
            self.program_counter = self.current_address;
        }

    }
    //Branch on result not zero
    //Function: branch on Z = 0
    //Flags: none
    fn bne(&mut self){
        //if the zero flag is not set
        if !self.get_flag(FLAGS::Z) {
            //add 1 cycle
            self.current_cycles += 1;
            //calculate the new address
            self.current_address = self.program_counter.wrapping_add(self.branch_address);
            //check if the page boundary is crossed and add 1 cycle if so
            if self.current_address & 0xFF00 != self.program_counter & 0xFF00{
                self.current_cycles += 1;
            }
            //set the new program counter
            self.program_counter = self.current_address;
        }
    }

    //Branch on result plus
    //Function: branch on N = 0
    //Flags: none
    fn bpl(&mut self){
        //if the negative flag is not set
        if !self.get_flag(FLAGS::N){
            //add 1 cycle
            self.current_cycles += 1;
            //calculate the new address
            self.current_address = self.program_counter.wrapping_add(self.branch_address);
            //check if the page boundary is crossed
            if self.current_address & 0xFF00 != self.program_counter & 0xFF00 {
                self.current_cycles += 1;
            }
            //set the new program counter
            self.program_counter = self.current_address;
        }
    }
    //Force break
    //BRK initiates a software interrupt similar to a hardware
    // interrupt (IRQ). The return address pushed to the stack is
    // PC+2, providing an extra byte of spacing for a break mark
    // (identifying a reason for the break.)
    // The status register will be pushed to the stack with the break
    // flag set to 1. However, when retrieved during RTI or by a PLP
    // instruction, the break flag will be ignored.
    // The interrupt disable flag is not set automatically.

    //Function: push PC+2, push SR, set I
    fn brk(&mut self,ppu: &mut Ppu){

          //increment the program counter
          self.program_counter =self.program_counter.wrapping_add(1);

        //push the program counter to the stack
          self.write_memory(Some(ppu),0x0100 + self.stack_pointer as u16, (self.program_counter >> 8) as u8);
          self.stack_pointer =self.stack_pointer.wrapping_sub(1);
          self.write_memory(Some(ppu),0x0100 + self.stack_pointer as u16, self.program_counter as u8);
          self.stack_pointer =self.stack_pointer.wrapping_sub(1);
          //push the status register t
         self.write_memory(Some(ppu),0x0100 + self.stack_pointer as u16, self.status | 1<<4 | 1<<5);
          self.stack_pointer =self.stack_pointer.wrapping_sub(1);

          //set the interrupt disable flag
            self.set_flag(FLAGS::I, true);


          //set the program counter to the interrupt vector
         self.program_counter = (self.read_memory(Some(ppu),0xFFFE) as u16) | ((self.read_memory(Some(ppu),0xFFFF) as u16) << 8);

    }
    //Branch on overflow clear
    //Function: branch V==0
    fn bvc(&mut self){
        //if the overflow flag is not set
        if !self.get_flag(FLAGS::V){
            //add 1 cycle
            self.current_cycles += 1;
            //get the new address
            self.current_address = self.program_counter.wrapping_add(self.branch_address);
            //check if there is a page boundary crossing and add 1 cycle if there is
            if self.current_address & 0xFF00 != self.program_counter & 0xFF00 {
                self.current_cycles += 1;
            }
            //set the program counter to the new address
            self.program_counter = self.current_address;
        }
    }


    //Branch on overflow set
    //Function: branch on V = 1
    //Flags:None
    fn bvs(&mut self){
        if self.get_flag(FLAGS::V) {
            //add 1 cycle if the branch occurs to the same page
            self.current_cycles += 1;
            //get the address to jump to
            self.current_address = self.program_counter.wrapping_add(self.branch_address);
            //add 1 cycle if the branch occurs to a different page
            if self.current_address & 0xFF00 != self.program_counter & 0xFF00 {
                self.current_cycles += 1;
            }
            //set the program counter to the new address
            self.program_counter = self.current_address;
        }
    }

    //Clear carry flag
    //Function: C = 0
    //Flags: C
    fn clc(&mut self){
        self.set_flag(FLAGS::C, false);
    }
    //Clear decimal mode
    //Function: D = 0
    //Flags: D
    fn cld(&mut self){
        self.set_flag(FLAGS::D, false);
    }
    //Clear interrupt disable bit
    //Function: I = 0
    //Flags: I
    fn cli(&mut self){
        self.set_flag(FLAGS::I, false);
    }
    //Clear overflow flag
    //Function: V = 0
    //Flags: V
    fn clv(&mut self){
        self.set_flag(FLAGS::V, false);
    }
    //Compare memory and accumulator
    //Function: A - M
    //Flags :N Z C
    fn cmp(&mut self,ppu: &mut Ppu)
    {
        //fetch data from the current address
        self.get_current_data(ppu);
        //compare the accumulator with the fetched data
        //let result = (self.accumulator as u16 - self.current_fetched_data as u16).wrapping_neg();
        let temp= self.accumulator.wrapping_sub(self.current_fetched_data);

        //set the carry flag if the accumulator is greater than the fetched data
        self.set_flag(FLAGS::C, self.accumulator>=self.current_fetched_data);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, temp == 0);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, temp&0x80!=0);
        //there is a potential page boundary crossing which adds 1 cycle
        if self.additional_cycle {
            self.current_cycles += 1;
        }


    }

    //Compare memory and index X
    //Function : X - M
    //Flags : N,Z,C
    fn cpx(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //compare the index X with the fetched data
        let temp=self.x.wrapping_sub(self.current_fetched_data);
        //set the carry flag if the index X is greater than the fetched data
        self.set_flag(FLAGS::C, self.x >= self.current_fetched_data);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, temp==0);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, temp & 0x80 != 0);
    }

    //Compare memory and index Y
    //Function: Y - M
    //Flags: C,Z,N
    fn cpy(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);

        let temp= self.y.wrapping_sub(self.current_fetched_data);

        //compare the index Y with the fetched data
        //set the carry flag if the index Y is greater than the fetched data
        self.set_flag(FLAGS::C, self.y >= self.current_fetched_data);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, temp==0);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, temp&0x80!=0);
        }
    //Decrement memory by one
    //Function: M = M - 1
    //Flags: N, Z
    fn dec(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //decrement the fetched data by one
        let result = self.current_fetched_data.wrapping_sub(1);
        //write the result to the memory
        self.write_memory(Some(ppu),self.current_address, result);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, result == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, result & 0x80 != 0);
        //there is a potential page boundary crossing which adds 1 cycle

    }

    //Decrement index X by one
    //Function: X = X - 1
    //Flags: N,Z
    fn dex(&mut self){
        //decrement the index X
        self.x = self.x.wrapping_sub(1);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.x == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.x & 0x80 != 0);
    }

    //Decrement index Y by one
    //Function: Y = Y - 1
    //Flags: Z, N
    fn dey(&mut self){
        //decrement the index Y
        self.y=self.y.wrapping_sub(1);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.y == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.y & 0x80 != 0);
    }

    //Exclusive-OR memory with accumulator
    //Function: A = A ^ M
    //Flags: N, Z
    fn eor(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //perform the operation
        self.accumulator ^= self.current_fetched_data;
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);

        //there is a potential page boundary crossing which adds 1 cycle
        if self.additional_cycle {
            self.current_cycles += 1;
        }

    }


    //Increment memory by one
    //Function: M = M + 1
    //Affects Flags: Z, N
    fn inc(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);

        //increment the fetched data
        let result=(self.current_fetched_data).wrapping_add(1);

        self.write_memory(Some(ppu),self.current_address, result as u8);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, result== 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, result & 0x80 != 0);
    }

    //Increment index X by one
    //Function: X = X + 1
    //Flags: N,Z
    fn inx(&mut self){
        //increment the index X
        self.x=self.x.wrapping_add(1);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.x == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.x & 0x80 != 0);
    }

    //Increment index Y by one
    //Function: Y = Y + 1
    //Flags: N, Z
    fn iny(&mut self){
        //increment the index Y
        self.y= self.y.wrapping_add(1);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.y == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.y & 0x80 != 0);
    }

    //Jump to new location
    //Function:(PC+1) -> PCL , (PC+2) -> PCH
    fn jmp(&mut self){
        //Implied from the addressing mode
        self.program_counter = self.current_address;
    }

    //Jump to new location saving return address
    //Function: push (PC+2), (PC+1) -> PCL, (PC+2) -> PCH
    //Flags: None
    fn jsr(&mut self,ppu: &mut Ppu){
        //push the program counter to the stack
        self.program_counter = self.program_counter.wrapping_sub(1);
        self.write_memory(Some(ppu),0x0100 + self.stack_pointer as u16, ((self.program_counter >> 8) & 0x00FF) as u8);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.write_memory(Some(ppu),0x0100 + self.stack_pointer as u16, (self.program_counter & 0x00FF) as u8);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        //set the program counter to the current address
        self.program_counter = self.current_address;
    }

    //Load accumulator with memory
    //Function: A = M
    //Flags: N, Z
    fn lda(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
         self.get_current_data(ppu);

       // println!("LDA: {:X}",self.current_fetched_data);

        //set the accumulator to the fetched data
        self.accumulator = self.current_fetched_data;
        //set the zero flag if the result is zero
        //println!("LDA: {:X}",self.accumulator);

        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
        //there is a potential page boundary crossing which adds 1 cycle
        if self.additional_cycle {
            self.current_cycles += 1;
        }
    }

    //Load index X with memory
    //Function: X=M
    //Flags: N,Z
    fn ldx(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //set the index X to the fetched data
        self.x = self.current_fetched_data;
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.x == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.x & 0x80 != 0);
        //there is a potential page boundary crossing which adds 1 cycle
        if self.additional_cycle {
            self.current_cycles += 1;
        }
    }

    //Load index Y with memory
    //Function:M -> Y
    //Flags: N Z
    fn ldy(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //set the index Y to the fetched data
        self.y = self.current_fetched_data;
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.y == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.y & 0x80 != 0);
        //there is a potential page boundary crossing which adds 1 cycle
        if self.additional_cycle {
            self.current_cycles += 1;
        }
    }

    //Shift one bit right (memory or accumulator)
    //Function: 0 ->M -> C
    //Flags: N,Z
    fn lsr(&mut self,ppu: &mut Ppu){
        //fetch data from the current address

        self.get_current_data(ppu);
        //set the carry flag if the 0th bit is set
        self.set_flag(FLAGS::C, self.current_fetched_data & 1 > 0);
        //shift the data right by 1 bit
        let result = self.current_fetched_data.wrapping_shr(1);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, result == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, result & 0x80 != 0);


        //if the current addressing mode is implied, then the accumulator is being shifted
        if self.curr_instr_set.addressing_mode==AddressingMode::IMP {
            self.accumulator = result;
        }
        //otherwise, write the result to the current address
        else{
            self.write_memory(Some(ppu),
                self.current_address,
                result
            );
        }
    }

    //No operation
    //Function: No operation
    //Flags: None
    fn nop(&mut self){
        //no operation
        //there is a potential page boundary crossing which adds 1 cycle
        if self.current_instruction == 0xFC && self.additional_cycle{
            self.current_cycles += 1;
        }
    }

    //Logical inclusive OR
   //Function: A OR M -> A
    //Flags: N, Z
    fn ora(&mut self,ppu: &mut Ppu){
        //fetch data from the current address

        self.get_current_data(ppu);
        //perform the logical OR operation
        self.accumulator |= self.current_fetched_data;
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
        //there is a potential page boundary crossing which adds 1 cycle
        if self.additional_cycle {
            self.current_cycles += 1;
        }
    }

    //Push accumulator on stack
    //Function: push A
    //Flags: none
    fn pha(&mut self,ppu: &mut Ppu){
        //push the accumulator to the stack
        self.write_memory(Some(ppu),0x0100+self.stack_pointer as u16, self.accumulator);
        //decrement the stack pointer
        self.stack_pointer=self.stack_pointer.wrapping_sub(1);
    }

    //Push processor status on stack
    //The status register will be pushed with the break flag and bit 5 set to 1.
    //Function : push SR
    //Flags: none
    fn php(&mut self,ppu: &mut Ppu){
        //push the status register to the stack
        self.write_memory(Some(ppu),0x0100 + self.stack_pointer as u16, self.status | (1<<5) | (1<<4));
        //decrement the stack pointer
        self.stack_pointer=self.stack_pointer.wrapping_sub(1);
    }

    //Pull accumulator from stack
    //Function: pull A
    //Flags: N Z
    fn pla(&mut self,ppu: &mut Ppu){
        //pop the accumulator from the stack
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        //set the accumulator to the popped value from the stack
        self.accumulator = self.read_memory(Some(ppu),0x0100 + self.stack_pointer as u16);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the 8th bit is set
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
    }

    //Pull processor status from stack
    //The pull processor status from stack instruction pulls the processor status from the stack and sets the processor status flags accordingly.
    //Function: pull SR
    //Flags: U
    fn plp(&mut self,ppu: &mut Ppu){
        //pop the status register from the stack
        self.stack_pointer=self.stack_pointer.wrapping_add(1);
        //set the status register to the popped value from the stack
        self.status = self.read_memory(Some(ppu),0x0100 + self.stack_pointer as u16);
        //set the unused flag to true
        self.set_flag(FLAGS::U, true);
        //set the break flag to false
        self.set_flag(FLAGS::B, false);
        }

    //Rotate one bit left (memory or accumulator)
    //Function : CC<- A<<1 <-C
    //Flags : C, Z, N
    fn rol(&mut self,ppu: &mut Ppu){
         //fetch data from the current address
        self.get_current_data(ppu);

        let carry = self.get_flag(FLAGS::C);
        //shift the fetched data left by 1
        let result = (self.current_fetched_data<<1) | carry as u8;
        //set the carry flag if the 7th bit is set
        self.set_flag(FLAGS::C, (self.current_fetched_data>>7) & 1 > 0);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, result == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, result & 0x80 != 0);

        //if the addressing mode is implied, then write the result to the accumulator
        if self.curr_instr_set.addressing_mode == AddressingMode::IMP {
            self.accumulator = result;
        }else {
            //otherwise write the result to the fetched address
            self.write_memory(Some(ppu),self.current_address, result);
        }
    }



    //Rotate one bit right (memory or accumulator)
    //Function: C->A >> 1 -> C
    //Flags: C, Z, N
    fn ror(&mut self,ppu: &mut Ppu) {
        //fetch data from the current address

        self.get_current_data(ppu);
        //shift the fetched data right by 1 bit
        //the carry flag is shifted into the 7th bit
        let carry = if self.get_flag(FLAGS::C) {1} else {0};
        //shift the fetched data right by 1
        let mut result=self.current_fetched_data.rotate_right(1);
        //check if the carry flag is set so that the 7th bit can be set due to the rotation
        if carry==1{
            //if the carry flag is set, set the 7th bit to 1
            result |= 0x80;
        }else{
            //otherwise, set the 7th bit to 0
            result &= 0x7F;
        }

        //set the carry flag if the 0th bit is set
        self.set_flag(FLAGS::C, self.current_fetched_data & 0x01 >0);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, result == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, result & 0x80 != 0);

        //if the addressing mode is implied, write the result to the accumulator
         if self.curr_instr_set.addressing_mode == AddressingMode::IMP {
            self.accumulator = result;
    }else{
         //otherwise write the result to the current address
        self.write_memory(Some(ppu),self.current_address, result);
    }


    }



    //Return from interrupt
    //The status register is pulled with the break flag
    // and bit 5 ignored. Then PC is pulled from the stack.
    //Function: pull SR, pull PC
    fn rti(&mut self,_ppu: &mut Ppu){
        //pull the status register from the stack
        self.stack_pointer=self.stack_pointer.wrapping_add(1);
        //set the status register to the popped value from the stack
        self.status = self.read_memory(None,0x0100 + self.stack_pointer as u16);
        //Break flag and bit 5 is ignored
        self.set_flag(FLAGS::B, false);
        //set the unused flag to true
        self.set_flag(FLAGS::U, false);

        //increment the stack pointer
        self.stack_pointer=self.stack_pointer.wrapping_add(1);
        //set the program counter to the popped value from the stack (low byte)
        self.program_counter = self.read_memory(None,0x0100 + self.stack_pointer as u16) as u16;
        self.stack_pointer =self.stack_pointer.wrapping_add(1);
         //set the program counter to the popped value from the stack (high byte)
        self.program_counter |= (self.read_memory(None,0x0100 + self.stack_pointer as u16) as u16) << 8 ;
    }


    //Return from subroutine
    //Function: pull PC,PC+1 ->PC
    //Flags: none
    fn rts(&mut self){
        //pop the program counter from the stack

        //Move the stack pointer up by 1
        self.stack_pointer= self.stack_pointer.wrapping_add(1);
        //set the status register to the value at the current stack pointer
        self.program_counter = self.read_memory(None,0x0100 + self.stack_pointer as u16) as u16;
        //Move the stack pointer up by 1
        self.stack_pointer= self.stack_pointer.wrapping_add(1);
        //Read the high byte of the program counter from the stack
        self.program_counter |= (self.read_memory(None,0x0100 + self.stack_pointer as u16) as u16) << 8;
        //Increment the program counter by 1
        self.program_counter=self.program_counter.wrapping_add(1);

    }

    //Subtract memory from accumulator with borrow
    //Function: A - M - C -> A
    //Flags: N, Z, C, V
    fn sbc(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);

        //add the carry flag to the fetched data
        let value = (self.current_fetched_data as u16) ^ 0x00FF;
        //compare the accumulator with the fetched data
        let result = (self.accumulator as u16 +self.get_flag(FLAGS::C) as u16).wrapping_add(value);
        //set the carry flag if the accumulator is greater than the fetched data
        self.set_flag(FLAGS::C, result & 0xFF00 != 0);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, result & 0x00FF == 0x0000);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, result & 0x80 != 0);
        //set the overflow flag if the result is negative
        self.set_flag(FLAGS::V, (self.accumulator ^ self.current_fetched_data) & (self.accumulator ^ result as u8) & 0x80 != 0);
        //set the accumulator to the result
        self.accumulator = result as u8;
        //there is a potential page boundary crossing which adds 1 cycle
        if self.additional_cycle {
            self.current_cycles += 1;
        }
    }


    //Set carry flag
    //Function:1->C
    //Flags: C
    fn sec(&mut self){
        self.set_flag(FLAGS::C, true);
    }


    //Set decimal mode
    //Function:1->D
    //Flag: D
    fn sed(&mut self){
        self.set_flag(FLAGS::D, true);
    }


    //Set interrupt disable status
    //Function: 1->I
    //Flags: I
    fn sei(&mut self){
        self.set_flag(FLAGS::I, true);
    }

    //Store accumulator in memory
    //Function: A -> M
    //Flags: None
    fn sta(&mut self,ppu: &mut Ppu){
        //write the accumulator to the current address
        self.write_memory(Some(ppu),self.current_address, self.accumulator);
    }

    //Store index X in memory
    //Function: X-> M
    //Flags: None
    fn stx(&mut self,ppu: &mut Ppu){
        //write the register X to the current address
        self.write_memory(Some(ppu),self.current_address, self.x);
    }

    //Store index Y in memory
    //Function: Y -> M
    //Flags: None
    fn sty(&mut self,ppu: &mut Ppu){
        //write the register Y to the current address
        self.write_memory(Some(ppu),self.current_address, self.y);
    }

    //Transfer accumulator to index X
    //Function: A-> X
    //Flags: N Z
    fn tax(&mut self){
        //transfer the accumulator to the index X
        self.x= self.accumulator;
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.x == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.x & 0x80 != 0);


    }

    //Transfer accumulator to index Y
    //Function: A -> Y
    //Flags: N, Z
    fn tay(&mut self){
        //set the index Y to the accumulator
        self.y = self.accumulator;
        //set the zero flag if the index Y is zero
        self.set_flag(FLAGS::Z, self.y == 0x00);
        //set the negative flag if the index Y is negative
        self.set_flag(FLAGS::N, self.y & 0x80 != 0);


    }


    //Transfer stack pointer to index X
    //Function: SP -> X
    //Flags: N Z
    fn tsx(&mut self){
        //set the index X to the stack pointer
        self.x= self.stack_pointer;
        //set the zero flag if the index X is zero
        self.set_flag(FLAGS::Z, self.x == 0x00);
        //set the negative flag if the index X is negative
        self.set_flag(FLAGS::N, self.x & 0x80 != 0);

    }

    //Transfer index X to accumulator
    //Function: X -> A
    //Flags: N, Z
    fn txa(&mut self){
        //transfer the value of the index X to the accumulator
        self.accumulator = self.x;
        //set the zero flag if the accumulator is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the accumulator is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);

    }


    //Transfer index X to stack pointer
    //Function : X -> SP
    //Flags affected : None

    pub fn txs(&mut self){
        //set the stack pointer to the value of the index X
        self.stack_pointer = self.x;
    }

    //Transfer index Y to accumulator
    //Function: Y -> A
    //Flags: N, Z
    fn tya(&mut self){
        //set the accumulator to the value of the index Y
        self.accumulator = self.y;
        //set the zero flag if the accumulator is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the accumulator is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);

    }








    ///Unofficial instructions
    //It should be pointed out that some of



    //SKB - Skip next byte
    //Function: PC + 2
    //Flags: None
    fn skb(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
    }

    //IGN - Ignore next byte
    //Function: PC + 2
    //Flags: None
    fn ign(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
    }

    //ISC/ISB - Increment memory and subtract from accumulator
    //Function: M + 1 -> M, M - A -> A
    //Flags: N, Z, C, V
    fn isb(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //add 1 to the fetched data
        let result = self.current_fetched_data.wrapping_add(1);
        //write the result to the current address
        self.write_memory(Some(ppu),self.current_address, result);
         //
        let a = self.accumulator;
        let (x1,o1) = a.overflowing_sub(result);
        let (x2,o2) = x1.overflowing_sub(1-self.get_flag(FLAGS::C) as u8);
        self.accumulator = x2;
        //set
        self.set_flag(FLAGS::C, !o1 && !o2);
        self.set_flag(FLAGS::Z, self.accumulator == 0);
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
        self.set_flag(FLAGS::V, (a ^ result) & (a ^ x2) & 0x80 != 0);
    }


    //DCP - Decrement memory and compare with accumulator
    fn dcp(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //subtract 1 from the fetched data
        let result = self.current_fetched_data.wrapping_sub(1);
        //write the result to the current address
        self.write_memory(Some(ppu),self.current_address, result);
        //compare the accumulator with the result
        let temp = self.accumulator.wrapping_sub(result);
        //set the carry flag if the result is positive
        self.set_flag(FLAGS::C,  self.accumulator>=result);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z,temp==0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, temp & 0x80 != 0);

    }




    //Based on https://www.nesdev.org/undocumented_opcodes.txt
    //AND byte with accumulator. If result is negative then carry is set.
    //Function: A & M -> A
    //Flags: N, Z, C
    #[allow(dead_code)]
    fn aac(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        if self.curr_instr_set.addressing_mode != AddressingMode::IMP {
            self.current_fetched_data = self.read_memory(Some(ppu),self.current_address);
        }
        //AND the accumulator with the fetched data
        self.accumulator &= self.current_fetched_data;
        //set the carry flag if the result is negative
        self.set_flag(FLAGS::C, self.accumulator & 0x80 != 0);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
    }

    //AND X register with accumulator and store result in memory. Status
    //Function : A & X -> M
    //Flags: N, Z
    #[allow(dead_code)]
    fn aax(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        if self.curr_instr_set.addressing_mode != AddressingMode::IMP {
            self.current_fetched_data = self.read_memory(Some(ppu),self.current_address);
        }
        //AND the accumulator with the index X
        let result = self.accumulator & self.x;
        //write the result to the current address
        self.write_memory(Some(ppu),self.current_address, result);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, result == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, result & 0x80 != 0);
    }

    //AND byte with accumulator, then rotate one bit right in accumulator and check bit 5 and 6
    //If both bits are 1 :set C,clear V
    //If both bits are 0 :clear C,clear V
    //If bit 5 is 1 and bit 6 is 0 :set C,set V
    //If only bit 6 is 1:set C and V
     //Function: A & M -> A, ROR A
    //Flags: N, Z, C, V
    fn arr(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //AND the accumulator with the fetched data
        self.accumulator &= self.current_fetched_data;
        //set the overflow flag if bit 6 and 5 are 1
        self.set_flag(FLAGS::V, (self.accumulator ^ (self.accumulator >> 1)) & 0x40 == 0x40);
        //rotate the accumulator one bit right
        let msb=self.accumulator >> 7;
        self.accumulator = self.accumulator >> 1 | (self.get_flag(FLAGS::C) as u8) << 7;
        //set the carry flag if the most significant bit is 1
        self.set_flag(FLAGS::C, msb & 0x01 == 1);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
    }

    //AND byte with accumulator, then shift right one bit in accumulator.
    //Function: A & M -> A, LSR A
    //Flags: N, Z, C
    #[allow(dead_code)]
    fn asr(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        if self.curr_instr_set.addressing_mode != AddressingMode::IMP {
            self.current_fetched_data = self.read_memory(Some(ppu),self.current_address);
        }
        //AND the accumulator with the fetched data
        self.accumulator &= self.current_fetched_data;
        //shift the accumulator one bit to the right
        //last LSB
        let lsb = self.accumulator & 0x01;
        self.accumulator >>= 1;

        //set the carry flag if the bit 0 of the accumulator is 1
        self.set_flag(FLAGS::C, lsb != 0);
        //set the zero flag if the accumulator is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the accumulator is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
    }

    //AND byte with accumulator, then transfer accumulator to X register.
    //Flags: N,Z
    #[allow(dead_code)]
    fn atx(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        if self.curr_instr_set.addressing_mode != AddressingMode::IMP {
            self.current_fetched_data = self.read_memory(Some(ppu),self.current_address);
        }
        //AND the accumulator with the fetched data
        self.accumulator &= self.current_fetched_data;
        //set the X register to the value of the accumulator
        self.x = self.accumulator;
        //set the zero flag if the accumulator is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the accumulator is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
    }

    //AND X register with accumulator then AND result with 7 and store in memory.
    //Flags:unused
    #[allow(dead_code)]
    fn axa(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        if self.curr_instr_set.addressing_mode != AddressingMode::IMP {
            self.current_fetched_data = self.read_memory(Some(ppu),self.current_address);
        }
        //AND the accumulator with the index X
        let result = self.accumulator & self.x;
        //AND the result with 7
        let result = result & 0x07;
        //write the result to the current address
        self.write_memory(Some(ppu),self.current_address, result);
    }


    //AND X register with accumulator and store result in X regis-ter, then
   //subtract byte from X register (without borrow).
    //Function: A & X -> X, X - M
    //Flags: N, Z, C
    fn axs(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
       self.get_current_data(ppu);
        //AND the accumulator with the index X
        let result=((self.accumulator & self.x) as u32).wrapping_sub(self.current_fetched_data as u32);
        //set the carry flag if the result is positive
        self.set_flag(FLAGS::C,( (result>>8) & 0x01 ) ^ 0x01 == 0x01);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, result as u8 == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, result as u8 & 0x80 != 0);
        //set the X register to the result
        self.x = result as u8;

   }

    //No operation (double NOP). The argument has no significance.
    fn dop(&mut self,ppu: &mut Ppu){
        //do nothing
        self.get_current_data(ppu);
    }


    //Stop program counter (processor lock up).
    #[allow(dead_code)]
    fn kil(&mut self){
        //do nothing
    }


    //AND memory with stack pointer, transfer result to accumulator, X register and stack pointer.
    //Function: A & S & M -> A, X, S
    //Flags: N, Z
    #[allow(dead_code)]
    fn lar(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        if self.curr_instr_set.addressing_mode != AddressingMode::IMP {
            self.current_fetched_data = self.read_memory(Some(ppu),self.current_address);
        }
        //AND the stack pointer with the fetched data
        let result = self.stack_pointer & self.current_fetched_data;
        //set the accumulator to the result
        self.accumulator = result;
        //set the X register to the result
        self.x = result;
        //set the stack pointer to the result
        self.stack_pointer = result;
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, result == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, result & 0x80 != 0);
    }

    //LAS
    //AND memory with stack pointer, transfer result to accumulator and X register, then AND result with 7 and store in stack pointer.
    //Function: A & S & M -> A, X, S & 7
    //Flags: N, Z
    fn las(&mut self,ppu: &mut Ppu){
        //execute the lda instruction
        self.lda(ppu);
        //execute the tax instruction
        self.tsx();
    }


    //AHX/SHA/AXA
    //AND X register with accumulator and store result in X register, then AND result with 7 and store in memory.
    //Function: A & X -> X, X & 7 -> M
    //Flags: unused
    fn ahx(&mut self,ppu: &mut Ppu){
        // and the accumulator with the index X and store the result in the X register
        let result=self.accumulator & self.x & self.current_fetched_data.wrapping_add(self.y).wrapping_shr(8).wrapping_add(1);
        //write the result to the current address
        self.write_memory(Some(ppu),self.current_address, result);
    }


    //SAX : AND X register with accumulator and store result in memory.

    fn sax(&mut self,ppu: &mut Ppu){
        //AND the accumulator with the index X
        self.get_current_data(ppu);
        //and the accumulator with the register X
        let result = self.accumulator & self.x;
        //write the result to the current address
        self.write_memory(Some(ppu),self.current_address, result);
    }

    //XAA :AND X register with accumulator and store result in accumulator.
    //Function: A & X -> A
    //Flags: N, Z

    //Based on the code from https://docs.rs/tetanes/latest/tetanes/
    fn xaa(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        self.accumulator |= 0xEE;
        //AND the accumulator with the index X
        self.accumulator &= self.x;
        //and the accumulator with the fetched data
        self.accumulator &= self.current_fetched_data;
        //set the zero flag if the accumulator is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the accumulator is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
    }



    //Load accumulator and X register with memory.
    //Function: M -> A, X
    //Flags: N, Z
    fn lax(&mut self,ppu: &mut Ppu){
        //execute the lda instruction
        self.lda(ppu);
        //execute the tax instruction
        self.tax();
    }

    //Rotate one bit left in memory, then AND accumulator with memory.
    //Function: M << 1 -> M, A & M -> A
    //Flags: N, Z, C
    fn rla(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //get the carry flag
        let carry = self.get_flag(FLAGS::C) as u8;
        //set the carry flag if the most significant bit is set
        self.set_flag(FLAGS::C, self.current_fetched_data & 0x80 != 0);
        //rotate the fetched data left by one bit
        let result = self.current_fetched_data << 1 | carry;
        //write the result to the current address
        self.write_memory(Some(ppu),self.current_address, result);
        //AND the accumulator with the result
        self.accumulator &= result;
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
    }
   //Rotate one bit right in memory, then add memory to accumulator (with carry).
   //Function: M >> 1 -> M, A + M + C -> A
   //Flags: N, Z, C, V
    fn rra(&mut self,ppu: &mut Ppu) {
       //fetch data from the current address
       self.get_current_data(ppu);
       //rotate the fetched data right
       let mut result = self.current_fetched_data.rotate_right(1);

       //check the  carry flag
       //wrapping add the accumulator with the result and the carry flag
       if self.get_flag(FLAGS::C) {
       result |= 0x80;
       } else {
         result &= 0x7F;
       }
       //set the carry flag if the least significant bit is set
         self.set_flag(FLAGS::C, self.current_fetched_data & 0x01 != 0);
         //write the result to the current address
            self.write_memory(Some(ppu),self.current_address, result);

       let a=self.accumulator;
        let (x1, overflow1) = result.overflowing_add(a);
        let (x2, overflow2) = x1.overflowing_add(self.get_flag(FLAGS::C) as u8);

        self.accumulator = x2;

       //set the carry flag
        self.set_flag(FLAGS::C, overflow1 | overflow2);
        //set the overflow flag
       self.set_flag(FLAGS::V, (a ^ result) & 0x80 == 0 && (a ^ x2) & 0x80 != 0);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);

   }

    //TAS : Shortcut for STA  AND TXS
    //Function: A & X -> M, A & X -> S
    //Flags: unused
    fn tas(&mut self,ppu: &mut Ppu){
        //store the accumulator in the current address
        self.write_memory(Some(ppu),self.current_address, self.accumulator);
        //assign the stack pointer to the accumulator and the index X
        self.stack_pointer = self.x;
    }

    //Shift left one bit in memory, then OR accumulator with memory.
    //Function: M << 1 -> M, A | M -> A
    //Flags: N,Z,C
    fn slo(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //set the carry flag if the 7 bit is set
        self.set_flag(FLAGS::C, self.current_fetched_data & 0x80 != 0);
        //shift the fetched data left
        let result = self.current_fetched_data.wrapping_shl(1);
        //write the result to the current address
        self.write_memory(Some(ppu),self.current_address, result);
        //OR the accumulator with the fetched data
        self.accumulator |= result;
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
    }


    //ALR/ASR : AND accumulator with memory, then shift right one bit in accumulator.
    //Function: A & M -> A, A >> 1 -> A
    //Flags: N,Z,C
    fn alr(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //AND the accumulator with the fetched data
        self.accumulator &= self.current_fetched_data;
        //shift the accumulator right
        let result = self.accumulator >> 1;
        //set the carry flag if the 0 bit is set
        self.set_flag(FLAGS::C, self.accumulator & 0x01 != 0);
        //set the accumulator to the result
        self.accumulator = result;
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
    }






    //Shift right one bit in memory, then EOR accumulator with memory.
    //Function: M >> 1 -> M, A ^ M -> A
    //Flags: N, Z, C
    fn sre(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //shift the fetched data right
        //write the result to the current address
        //set the carry flag if the 0 bit is set
        self.set_flag(FLAGS::C, self.current_fetched_data & 0x01 != 0);
        //shift the fetched data right
        let result = self.current_fetched_data.wrapping_shr(1);
        self.write_memory(Some(ppu),self.current_address, result);
        //XOR the accumulator with the fetched data
        self.accumulator ^= result;
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
    }

     //AND X register with the high byte of the target address of the argument + 1. Store the result in memory.
     //Function: X & (A + 1) -> M
     //Flags: N, Z
    fn sxa(&mut self,ppu: &mut Ppu){
         //get the high byte of the current address
         let high_byte= (self.current_address >> 8) as u8;
         //get the low byte of the current address
         let low_byte= (self.current_address & 0x00FF) as u8;
         //and X register with the high byte of the target address of the argument + 1
         let result = self.x & high_byte.wrapping_add(1);
         self.current_address=(self.x as u16 & high_byte.wrapping_add(1) as u16)<<8 | low_byte as u16;
         //write the result to the new address
         self.write_memory(Some(ppu),self.current_address, result);

    }
    //AND Y register with the high byte of the target address of the argument + 1. Store the result in memory.
    //Function: Y & (A + 1) -> M
    //Flags: None
    fn sya(&mut self,ppu: &mut Ppu){
        //get the high byte of the current address
        let high_byte= (self.current_address >> 8) as u8;
        //get the low byte of the current address
        let low_byte= (self.current_address & 0xFF) as u8;
        //AND the index Y with the high byte of the current address + 1
        let result = self.y & high_byte.wrapping_add(1);
        //set the current address to the result
        self.current_address=((self.y as u16) & high_byte.wrapping_add(1) as u16)<<8 | low_byte as u16;
        //write the result to the current address
        self.write_memory(Some(ppu),self.current_address, result);
    }
     //No operation (triple NOP). The argument has no significance
     //Function: No operation/
     //Flags: None
     //clippy says that this function is not used, but it is used in the opcode table
     //clippy says that this function is not used, but it is used in the opcode table

     #[allow(dead_code)]
    fn top(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
     }
     //AND X register with accumulator and store result in stack pointer, then AND stack pointer with the high byte of the target address of the
     //argument + 1. Store result in memory.
     //Function: X & A -> SP, SP & (A + 1) -> M
        //Flags: None
     #[allow(dead_code)]
    fn xas(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
         if self.curr_instr_set.addressing_mode != AddressingMode::IMP {
             self.current_fetched_data = self.read_memory(Some(ppu),self.current_address);
         }
        //AND the X register with the accumulator
        let result = self.x & self.accumulator;
        //store the result in the stack pointer
        self.stack_pointer = result;
        //AND the stack pointer with the high byte of the target address of the argument + 1
        let result = self.stack_pointer & ((self.current_address >> 8) as u8 + 1);
        //write the result to the current address
        self.write_memory(Some(ppu),self.current_address, result);

    }


    // ANC/AAC : AND accumulator with memory, then set carry flag to bit 7 of accumulator.
    fn anc(&mut self,ppu: &mut Ppu){
        //fetch data from the current address
        self.get_current_data(ppu);
        //AND the accumulator with the fetched data
        self.accumulator &= self.current_fetched_data;
        //set the carry flag if the result is positive
        self.set_flag(FLAGS::C, self.accumulator & 0x80 != 0);
        //set the zero flag if the result is zero
        self.set_flag(FLAGS::Z, self.accumulator == 0x00);
        //set the negative flag if the result is negative
        self.set_flag(FLAGS::N, self.accumulator & 0x80 != 0);
    }







    //Others function to emulate the NES
    //At the moment is just a placeholder for the future
    fn other(&mut self,ppu: &mut Ppu){
        self.get_current_data(ppu);
    }


}