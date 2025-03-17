pub trait Config {
    type Moment: Copy + Default;
}

#[derive(Debug)]
pub struct Timestamp<T: Config> {
    now: T::Moment,
}


impl<T:Config> Timestamp<T> {
    pub fn new() -> Self {
        Timestamp {
            now: T::Moment::default(),
        }
    }

    pub fn set(&mut self, now: T::Moment) {
        self.now = now;
    }

    pub fn now(&self) -> T::Moment {
        self.now
    }
}
