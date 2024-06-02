// https://github.com/pretzelhammer/rust-blog/blob/master/posts/too-many-brainfuck-compilers.md#what-is-brainfuck

use crate::computer::Computer;
use crate::inst::Inst;

mod computer;
mod inst;

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
