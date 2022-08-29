pub struct Remember<T: Copy> {
    value: T,
    pub stale: bool,
    calculate: Box<dyn Fn() -> T>
}



impl<T: Copy> Remember<T> {
    pub fn new(calculate: Box<dyn Fn() -> T>, start_value: Option<T>) -> Remember<T> {
        match start_value {
            Some(value) => Remember {
                value,
                stale: false,
                calculate
            },
            None => Remember {
                value: calculate(),
                stale: false,
                calculate
            }
        }
    }

    pub fn get_static(&self) -> T {
        if self.stale {
            (self.calculate)()
        } else {
            self.value
        }
    }

    pub fn get(&mut self) -> T {
        if self.stale {
            self.value = (self.calculate)();
            self.stale = false;
        }

        self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        self.stale = false;
    }
}