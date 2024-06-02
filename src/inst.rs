use Kind::{DecByte, DecPtr, IncByte, IncPtr, LoopEnd, LoopStart, PrintByte, WriteByte};

#[derive(Debug)]
pub enum Kind {
    IncPtr,
    DecPtr,
    IncByte,
    DecByte,
    WriteByte,
    PrintByte,
    // end_idx: index of instruction after matching LoopEnd
    LoopStart { loop_end_idx: usize },
    // start_idx: index of instruction after matching LoopStart
    LoopEnd { loop_start_idx: usize },
}

#[derive(Debug)]
pub struct Inst {
    idx: usize,
    pub kind: Kind,
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
        // Parse square brackets, panic if they are unbalanced.
        // We'll use a stack of opening bracket indices to do this!
        let mut stack: Vec<usize> = Vec::new();
        for (idx, char) in parsed.char_indices() {
            match char {
                '[' => stack.push(idx),
                ']' => {
                    // close the latest opening bracket
                    let Some(opening_bracket_idx) = stack.pop() else {
                        panic!("ERROR: loop delimiter: unmatched ']'");
                    };

                    instructions.push(Self {
                        idx: opening_bracket_idx,
                        kind: LoopStart { loop_end_idx: idx + 1 },
                    });

                    instructions.push(Self {
                        idx,
                        kind: LoopEnd {
                            loop_start_idx: opening_bracket_idx + 1,
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
