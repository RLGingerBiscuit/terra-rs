use bit_reverse::ParallelReverse;

#[derive(thiserror::Error, Debug)]
pub enum BoolByteError {
    #[error("Invalid index: {0}.")]
    InvalidIndex(u8),
}

#[derive(Default, Debug, Clone)]
pub struct BoolByte {
    value: u8,
}

impl From<u8> for BoolByte {
    fn from(value: u8) -> Self {
        Self { value }
    }
}

impl From<&u8> for BoolByte {
    fn from(value: &u8) -> Self {
        Self { value: *value }
    }
}

impl From<BoolByte> for u8 {
    fn from(bb: BoolByte) -> Self {
        bb.value
    }
}

impl From<&BoolByte> for u8 {
    fn from(bb: &BoolByte) -> Self {
        bb.value
    }
}

impl BoolByte {
    fn check_index(&self, index: u8) -> anyhow::Result<(), BoolByteError> {
        if index >= 8 {
            Err(BoolByteError::InvalidIndex(index))
        } else {
            Ok(())
        }
    }

    pub fn clear_all(&mut self) {
        self.value = u8::MIN
    }

    pub fn set_all(&mut self) {
        self.value = u8::MAX
    }

    pub fn get(&self, index: u8) -> anyhow::Result<bool, BoolByteError> {
        self.check_index(index)?;
        Ok(self.value & (1 << index) != 0)
    }

    pub fn on(&mut self, index: u8) -> anyhow::Result<(), BoolByteError> {
        self.check_index(index)?;
        self.value |= 1 << index;
        Ok(())
    }

    pub fn off(&mut self, index: u8) -> anyhow::Result<(), BoolByteError> {
        self.check_index(index)?;
        self.value &= (1 << index).swap_bits();
        Ok(())
    }

    pub fn set(&mut self, index: u8, value: bool) -> anyhow::Result<(), BoolByteError> {
        if value {
            self.on(index)
        } else {
            self.off(index)
        }
    }

    pub fn toggle(&mut self, index: u8) -> anyhow::Result<(), BoolByteError> {
        if self.get(index)? {
            self.off(index)
        } else {
            self.on(index)
        }
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value
    }
}
