pub enum Priority {
    Min,
    VLow,
    Low,
    Normal,
    High,
    VHigh,
    Max
}

impl Priority {
    pub fn as_usize(&self) -> usize {
        match self {
            Priority::Min    => 0,
            Priority::VLow   => 1,
            Priority::Low    => 2,
            Priority::Normal => 3,
            Priority::High   => 4,
            Priority::VHigh  => 5,
                      _      => 6,
        }
    }
}
