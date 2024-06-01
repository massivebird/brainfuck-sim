use InstructionKind::*;

// https://github.com/pretzelhammer/rust-blog/blob/master/posts/too-many-brainfuck-compilers.md#what-is-brainfuck

#[derive(Debug)]
struct Computer {
    ptr: usize,
    memory: Vec<i32>,
}

impl Computer {
    fn new(num_bytes: usize) -> Self {
        Self {
            ptr: 0,
            memory: vec![0; num_bytes],
        }
    }
}

impl Computer {
    fn get_ptr_data(&self) -> i32 {
        unsafe { *self.memory.get_unchecked(self.ptr) }
    }

    fn execute(&mut self, instructions: &[Inst]) {
        for inst in instructions {
            match inst.kind {
                IncPtr => self.ptr += 1,
                DecPtr => self.ptr -= 1,
                IncByte => *self.memory.get_mut(self.ptr).unwrap() += 1,
                DecByte => *self.memory.get_mut(self.ptr).unwrap() -= 1,
                WriteByte => todo!(),
                ReadByte => todo!(),
                LoopStart { end_idx } => {
                    if self.get_ptr_data() == 0 {
                        self.ptr = end_idx;
                    }
                }
                LoopEnd { start_idx } => {
                    if self.get_ptr_data() != 0 {
                        self.ptr = start_idx;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Inst {
    kind: InstructionKind,
}

impl Inst {
    fn from_str(str: &str) -> Vec<Self> {
        // static analysis (?)
        // checks if all square brackets are properly closed
        let mut stack: Vec<InstructionKind> = Vec::new();
        for char in str.chars() {
            match char {
                '[' => stack.push(InstructionKind::LoopStart { end_idx: 0 }),
                ']' => {
                    assert!(
                        stack.pop().is_some(),
                        "ERROR: loop delimiter: unmatched ']'"
                    );
                }
                _ => continue,
            }
        }
        assert!(
            stack.is_empty(),
            "ERROR: loop delimiter: one or more unclosed '['"
        );

        let mut instructions: Vec<Self> = Vec::new();
        for symbol in str.chars() {
            let kind = match symbol {
                '>' => IncPtr,
                '<' => DecPtr,
                '+' => IncByte,
                '-' => DecByte,
                '[' => unimplemented!(),
                ']' => unimplemented!(),
                _ => continue, // all other characters are interpreted as comments
            };
            instructions.push(Self { kind });
        }
        instructions
    }
}

#[derive(Debug)]
enum InstructionKind {
    IncPtr,
    DecPtr,
    IncByte,
    DecByte,
    WriteByte,
    ReadByte,
    // end_idx = index of instruction after matching LoopEnd
    LoopStart { end_idx: usize },
    // start_idx = index of instruction after matching LoopStart
    LoopEnd { start_idx: usize },
}

fn main() {
    let mut computer = Computer::new(5);

    let bf = "++";

    let instructions = Inst::from_str(bf);

    computer.execute(&instructions);

    dbg!(&computer);
}
