


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MMC4 {
    pub num_prg_banks: u8,
    pub num_chr_banks: u8,
    //program ram also called static ram
    pub prog_ram: Vec<u8>,

   //program bank mode
    prg_mode: u8,
    //character bank mode
    chr_mode: u8,

   //register for bank switching
    chr_bank: [u32; 8],
    prg_bank: [u32; 4],
    //additonal registers
    reg: [u32; 8],

    //register for mirroring
    mirroring: u8,


    //Registers for IRQ
    irq_counter: u16,
    irq_reload: u16,
    irq_enable: bool,
    ir_active: bool,
    irq_update: bool,

    //regiters
    reg_select: u8,

}

impl MMC4{
    #[allow(dead_code)]
    fn new(num_prg_banks:u8,num_chr_banks:u8)->Self{
        Self{
            num_prg_banks,
            num_chr_banks,
            prog_ram:vec![255;8*1024],
            prg_mode:0,
            chr_mode:0,
            chr_bank:[0;8],
            prg_bank:[0;4],
            reg:[0;8],
            mirroring:0,
            irq_counter:0,
            irq_reload:0,
            irq_enable:false,
            ir_active:false,
            irq_update:false,
            reg_select:0,
        }
    }
}