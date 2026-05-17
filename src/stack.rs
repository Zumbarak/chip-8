const STACK_SIZE: usize = 16;

pub struct Stack {
    data: [u16; STACK_SIZE],
    sp: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: [0; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        if self.sp >= STACK_SIZE {
            panic!("Stack Overflow");
        }

        self.data[self.sp] = value;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        if self.sp == 0 {
            panic!("Stack Underflow");
        }

        self.sp -= 1;
        self.data[self.sp]
    }
}
