use crate::inst::{
    Inst,
    InstructionKind::{DecByte, DecPtr, IncByte, IncPtr, LoopEnd, LoopStart, PrintByte, WriteByte},
};

#[derive(Debug)]
pub struct Computer {
    data_ptr: usize,
    inst_ptr: usize,
    memory: Vec<u8>,
}

impl Computer {
    pub fn new(num_bytes: usize) -> Self {
        Self {
            data_ptr: 0,
            inst_ptr: 0,
            memory: vec![0; num_bytes],
        }
    }

    fn read_memory(&self) -> u8 {
        *self.memory.get(self.data_ptr).unwrap()
    }

    pub fn execute(&mut self, instructions: &[Inst]) {
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
                    print!("{}", char::from_u32(u32::from(self.read_memory())).unwrap());
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
