/// EOF Data Section
/// 
/// This contains a vector of unstructured, immediate bytes.
pub struct DataSection {
    /// Immediate bytes
    immediates: &[u8],
}

impl DataSection {
    /// Get immediate bytes
    pub fn immediates(&self) -> &[u8] {
        &self.immediates
    }

    /// Get data section size
    pub fn size(&self) -> u16 {
        self.immediates.len() as u16
    }
}
