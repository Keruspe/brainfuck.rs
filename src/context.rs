pub struct Context {
    buf:      Vec<char>,
    index:    usize,
    nest_lvl: u64,
}

impl Context {
    pub fn new() -> Context {
        Context {
            buf:      Vec::new(),
            index:    0,
            nest_lvl: 0,
        }
    }
}
