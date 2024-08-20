pub struct BoardRep {
    pub vec: Vec<u8>,
}

impl BoardRep {
    pub fn new(vec: &[u8]) -> Self {
        Self { vec: vec.to_vec() }
    }
}
