pub struct Grid {
    ticks: u32,
}

impl Default for Grid {
    fn default() -> Self {
        Self { ticks: 10 }
    }
}
