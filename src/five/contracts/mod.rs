#[derive(Copy, Clone)]
pub struct Move {
    pub number: u32,
    pub source: u32,
    pub destination: u32,
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Move {{ number: {}, source: {}, destination: {} }}",
            self.number, self.source, self.destination
        )
    }
}
