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

    let mut instructions: Vec<Inst> = Vec::new();
    for symbol in bf.chars() {
        let kind = match symbol {
            '>' => IncPtr,
            '<' => DecPtr,
            '+' => IncByte,
            '-' => DecByte,
            '[' => unimplemented!(),
            ']' => unimplemented!(),
            _ => unimplemented!(),
        };
        instructions.push(Inst { kind });
    }

    computer.execute(&instructions);
    dbg!(&computer);
}
