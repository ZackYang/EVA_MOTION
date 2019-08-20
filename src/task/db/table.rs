use std::slice::from_raw_parts;

pub enum Scope {
    U8,
    Float
} 

pub struct Table {
    pub u8_range: Vec<u8>,
    pub f32_range: Vec<f32>
}

impl Table {
    pub fn new(u8_size: usize, f32_size: usize) -> Table {
        Table { u8_range: vec![0u8; u8_size], f32_range: vec![0f32; f32_size] }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = self.u8_range.to_vec();
        println!("{:?}", results);
        for float in &self.f32_range {
            results.append(&mut float.to_bits().to_be_bytes().to_vec());
        }
        results
    }

    pub fn get_u8(&self, index: usize) -> u8 {
        self.u8_range[index]
    }

    pub fn get_f32(&self, index: usize) -> f32 {
        self.f32_range[index]
    }

    pub fn from_bytes(&mut self, data: &[u8]) {
        let u8_size = self.u8_range.len();
        for ui in 0..u8_size {
            self.u8_range[ui] = data[ui];
        }

        for fi in 0..(self.f32_range.len()) {
            let mut au8 = [0u8; 4];
            for i in 0..4 {
                let index = u8_size + fi * 4 + i;
                au8[i] = data[index];
            }
            let _u32 = u32::from_be_bytes(au8);
            self.f32_range[fi] = f32::from_bits(_u32);
        }
    }
}

trait __SetAndGet<T, U> {
    fn set(&mut self, index: T, value: U);
}

impl __SetAndGet<usize, u8> for Table {
    fn set(&mut self, index: usize, value: u8) {
        self.u8_range[index] = value;
    }
}

impl __SetAndGet<usize, f32> for Table {
    fn set(&mut self, index: usize, value: f32) {
        self.f32_range[index] = value;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_to_bytes() {
        let table = Table::new(10, 1);
        // println!("{:X?}", table.to_bytes());
        assert_eq!(table.to_bytes().len(), 14usize);
        assert_eq!(table.to_bytes()[10], 0u8);
        assert_eq!(table.to_bytes()[11], 0u8);
        assert_eq!(table.to_bytes()[12], 0u8);
        assert_eq!(table.to_bytes()[13], 0u8);
    }

    #[test]
    fn from_bytes() {
        let mut table = Table::new(1, 1);
        let data = vec![0b0001u8, 0b10111110u8, 0b00100000u8, 0b00000000u8, 0b00000000u8];
        table.from_bytes(&data);
        assert_eq!(table.f32_range[0], -0.15625);
    }

    #[test]
    fn set_and_get() {
        let mut table = Table::new(1, 1);
        table.set(0, 25u8);
        table.set(0, 3.5f32);
        assert_eq!(table.get_u8(0), 25);
        assert_eq!(table.get_f32(0), 3.5f32);
    }
}