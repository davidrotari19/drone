//type of mappers for cartridge
pub enum MAPPERS{
    NROM,
    MMC1,
    MMC2,
    MMC3,
    //this apparently is called MMC3 although is the 4th mapper
    MMC4,
}
//impliment defalu trait for MAPPERS
impl Default for MAPPERS{
    fn default() -> Self{
        MAPPERS::NROM
    }
}