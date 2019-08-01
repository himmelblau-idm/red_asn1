
#[derive(Debug, PartialEq)]
pub struct BitSringValue {
    bytes: Vec<u8>,
    padding_length: u8
}

impl BitSringValue {

    pub fn new(bytes: Vec<u8>, padding_length: u8) -> Self {
        let mut bsv = Self {
            bytes,
            padding_length
        };

        bsv.pad_with_0();
        return bsv;
    }

    pub fn get_bytes(&self) -> &Vec<u8> {
        return &self.bytes;
    }

    pub fn get_padding_length(&self) -> u8 {
        return self.padding_length;
    }

    fn pad_with_0(&mut self) {
        match self.bytes.pop() {
            Some(last_item) => {
                self.bytes.push(Self::set_0_padding(last_item, self.padding_length));
            },
            None => {}
        }
    }

    fn set_0_padding(mut item: u8, padding_length: u8) -> u8 {
        item >>= padding_length;
        item <<= padding_length;
        return item;
    }

}
