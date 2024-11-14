pub trait BitOps {
    fn get_bit(&self, pos: u8) -> bool;
    fn set_bit(&mut self, pos: u8, value: bool);
    fn get_bits(&self, start: u8, len: u8) -> u64;
    fn set_bits(&mut self, start: u8, len: u8, value: u64);
}

impl BitOps for u64 {
    fn get_bit(&self, pos: u8) -> bool {
        (*self & (1 << pos)) != 0
    }

    fn set_bit(&mut self, pos: u8, value: bool) {
        if value {
            *self |= 1 << pos;
        } else {
            *self &= !(1 << pos);
        }
    }

    fn get_bits(&self, start: u8, len: u8) -> u64 {
        (*self >> start) & ((1 << len) - 1)
    }

    fn set_bits(&mut self, start: u8, len: u8, value: u64) {
        let mask = ((1 << len) - 1) << start;
        *self = (*self & !mask) | ((value << start) & mask);
    }
}