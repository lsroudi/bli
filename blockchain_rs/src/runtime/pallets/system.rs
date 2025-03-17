pub trait Config {
    type BlockNumber: Copy + Default;
}

pub struct System<T: Config> {
    block_number: T::BlockNumber,
}

impl<T: Config> System<T> {
    pub fn new() -> Self {
        System {
            block_number: T::BlockNumber::default(),
        }
    }

    pub fn on_initialize(&mut self) {
        self.block_number = self.block_number + T::BlockNumber::from("1");
    }

    pub fn on_finalize(&mut self) {
        println!("block {} is finalized", self.block_number);
    }
}
