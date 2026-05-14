const STACK_SIZE: usize = 16;

struct Stack {
    data: [u16; STACK_SIZE],
    sp: usize,
}

impl Stack {
    fn new() -> Self {
        Self {
            data: [0; STACK_SIZE],
            sp: 0,
        }
    }

    fn push(&mut self, value: u16) {
        if self.sp >= STACK_SIZE {
            panic!("Stack Overflow");
        }

        self.data[self.sp] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        if self.sp == 0 {
            panic!("Stack Underflow");
        }

        self.sp -= 1;
        self.data[self.sp]
    }
}
