// https://github.com/pretzelhammer/rust-blog/blob/master/posts/too-many-brainfuck-compilers.md#what-is-brainfuck

#[derive(Default)]
struct Computer {
    ptr: usize,
    memory: Vec<i32>,
}

impl Computer {
    fn execute(&mut self, instructions: &[Instruction]) {

    }
}

struct Instruction {
    kind: InstructionKind,
}

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
    let computer = Computer::default();

    let bf = "++";
}
