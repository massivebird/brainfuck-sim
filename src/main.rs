use InstructionKind::*;

// https://github.com/pretzelhammer/rust-blog/blob/master/posts/too-many-brainfuck-compilers.md#what-is-brainfuck

#[derive(Debug)]
struct Computer {
    data_ptr: usize,
    inst_ptr: usize,
    memory: Vec<u8>,
}

impl Computer {
    fn new(num_bytes: usize) -> Self {
        Self {
            data_ptr: 0,
            inst_ptr: 0,
            memory: vec![0; num_bytes],
        }
    }
}

impl Computer {
    fn read_memory(&self) -> u8 {
        unsafe { *self.memory.get_unchecked(self.data_ptr) }
    }

    fn execute(&mut self, instructions: &[Inst]) {
        while let Some(inst) = instructions.get(self.inst_ptr) {
            match inst.kind {
                IncPtr => self.data_ptr += 1,
                DecPtr => self.data_ptr -= 1,
                IncByte => {
                    *self.memory.get_mut(self.data_ptr).unwrap() =
                        self.read_memory().wrapping_add(1);
                }
                DecByte => {
                    *self.memory.get_mut(self.data_ptr).unwrap() =
                        self.read_memory().wrapping_sub(1);
                }
                WriteByte => todo!(),
                PrintByte => {
                    println!("{}", char::from_u32(u32::from(self.read_memory())).unwrap());
                }
                LoopStart { end_idx } => {
                    if self.read_memory() == 0 {
                        self.inst_ptr = end_idx;
                        continue;
                    }
                }
                LoopEnd { start_idx } => {
                    if self.read_memory() != 0 {
                        self.inst_ptr = start_idx;
                        continue;
                    }
                }
            }

            // prime next instruction
            self.inst_ptr += 1;
        }
    }
}

#[derive(Debug)]
struct Inst {
    idx: usize,
    kind: InstructionKind,
}

impl Inst {
    fn from_str(str: &str) -> Vec<Self> {
        let mut instructions: Vec<Self> = Vec::new();

        // we only care about the brainfuck
        let str = str
            .chars()
            .filter(|&c| "<>+-.,[]".contains(c))
            .collect::<String>();

        // static analysis (?)
        // checks if all square brackets are properly closed
        let mut stack: Vec<usize> = Vec::new();
        for (idx, char) in str.char_indices() {
            match char {
                '[' => stack.push(idx),
                ']' => {
                    // close the latest opening bracket
                    let Some(start_idx) = stack.pop() else {
                        panic!("ERROR: loop delimiter: unmatched ']'");
                    };

                    instructions.push(Self {
                        idx: start_idx,
                        kind: InstructionKind::LoopStart { end_idx: idx + 1 },
                    });

                    instructions.push(Self {
                        idx,
                        kind: InstructionKind::LoopEnd {
                            start_idx: start_idx + 1,
                        },
                    });
                }
                _ => continue,
            }
        }

        assert!(
            stack.is_empty(),
            "ERROR: loop delimiter: one or more unclosed '['"
        );

        for (idx, char) in str.char_indices() {
            let kind = match char {
                '>' => IncPtr,
                '<' => DecPtr,
                '+' => IncByte,
                '-' => DecByte,
                '.' => PrintByte,
                _ => continue, // brackets already handled
            };
            instructions.push(Self { idx, kind });
        }

        instructions.sort_by(|a, b| a.idx.cmp(&b.idx));
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
    PrintByte,
    // end_idx = index of instruction after matching LoopEnd
    LoopStart { end_idx: usize },
    // start_idx = index of instruction after matching LoopStart
    LoopEnd { start_idx: usize },
}

fn main() {
    let mut computer = Computer::new(5);

    let bf = "
++++ ++++         initialize counter (cell #0) to 8
[                 loop adds 8x6 = 48 to cell #2
> +++ +++     
< -          
]        
> .               print character 48 '0'
< +++ +++ +++     initialize counter (cell #0) to 9
[                 loop prints characters 48 thru 57
> + .                                   '1' thru '9'
< -
]
";

    let instructions = Inst::from_str(bf);

    computer.execute(&instructions);

    dbg!(&computer);
}
