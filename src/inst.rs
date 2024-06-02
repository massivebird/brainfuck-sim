use InstructionKind::{DecByte, DecPtr, IncByte, IncPtr, LoopEnd, LoopStart, PrintByte, WriteByte};

#[derive(Debug)]
pub struct Inst {
    idx: usize,
    pub kind: InstructionKind,
}

impl Inst {
    pub fn from_str(str: &str) -> Vec<Self> {
        let mut instructions: Vec<Self> = Vec::new();

        // we only care about the brainfuck
        let parsed = str
            .chars()
            .filter(|&c| "<>+-.,[]".contains(c))
            .collect::<String>();

        // static analysis (?)
        // parse square brackets, panic if they are unbalanced
        let mut stack: Vec<usize> = Vec::new();
        for (idx, char) in parsed.char_indices() {
            match char {
                '[' => stack.push(idx),
                ']' => {
                    // close the latest opening bracket
                    let Some(start_idx) = stack.pop() else {
                        panic!("ERROR: loop delimiter: unmatched ']'");
                    };

                    instructions.push(Self {
                        idx: start_idx,
                        kind: LoopStart { end_idx: idx + 1 },
                    });

                    instructions.push(Self {
                        idx,
                        kind: LoopEnd {
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

        for (idx, char) in parsed.char_indices() {
            let kind = match char {
                '>' => IncPtr,
                '<' => DecPtr,
                '+' => IncByte,
                '-' => DecByte,
                '.' => PrintByte,
                ',' => WriteByte,
                _ => continue, // brackets already handled
            };
            instructions.push(Self { idx, kind });
        }

        // brackets were processed out of order, so we need to reorder all instructions
        instructions.sort_by(|a, b| a.idx.cmp(&b.idx));
        instructions
    }
}

#[derive(Debug)]
pub enum InstructionKind {
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
