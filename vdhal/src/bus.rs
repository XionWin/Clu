pub trait Bus
{
    fn select(&mut self);
    // fn write_bit(&self, address: u8, bit_num: u8, value: bool);
    // fn read_bit(&self, address: u8, bit_num: u8) -> bool;

    // fn write_bits(&self, address: u8, bit_start: u8, len: u8, value: u8);
    // fn read_bits(&self, address: u8, bit_num: u8, len: u8,) -> u8;

    fn write_byte(&mut self, address: u8, value: u8);
    fn read_byte(&mut self, address: u8) -> u8;

    // fn write(&self, value: *const u8);
    // fn read(&self, address: u8, buffer: *mut u8);
    fn deselect(&mut self);
}