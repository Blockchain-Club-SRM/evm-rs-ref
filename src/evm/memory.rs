use bigint::U256;
pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: Vec::new() }
    }

    pub fn resize(&mut self, new_size: usize) {
        if self.data.len() < new_size {
            self.data.resize(new_size, 0);
        }
    }

    pub fn get_word(&self, addr: usize) -> U256 {
        U256::from_big_endian(&self.data[addr..addr + 32])
    }

    pub fn set_byte(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }

    pub fn set_word(&mut self, addr: usize, value: U256) {
        let mut bytes = vec![0; 32];
        value.to_big_endian(&mut bytes);
        for i in 0..32 {
            self.data[addr + i] = bytes[i];
        }
    }
}
