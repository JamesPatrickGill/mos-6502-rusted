pub struct BUS {
    pub ram: [u8; 65536],
}

impl BUS {
    pub fn new() -> BUS {
        BUS { ram: [0; 65536] }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[(addr as usize)] = data;
    }

    pub fn read(&mut self, addr: u16, _read_only: bool) -> u8 {
        self.ram[(addr as usize)]
    }
}
