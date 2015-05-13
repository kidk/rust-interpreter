const CODE_MAX_SIZE: usize = 8192 * 2 * 2;
const CODE_MAX_DATA: usize = 8192 * 2;

const OP_VAL_INCR: u8 = 1;
const OP_VAL_DECR: u8 = 2;
const OP_PNT_INCR: u8 = 3;
const OP_PNT_DECR: u8 = 4;
const OP_LOOP_START: u8 = 5;
const OP_LOOP_END: u8 = 6;
const OP_OUT: u8 = 7;
const OP_IN: u8 = 8;
const OP_END: u8 = 9;

#[derive(Copy, Clone)]
pub struct Operation {
    op: u8,
    ex: usize
}

pub struct Program {
    pc: usize,
    ptr: usize,
    data: [u32; CODE_MAX_DATA],
    code: [Operation; CODE_MAX_SIZE]
}

impl Program {
    pub fn new() -> Program
    {
        Program {
            pc: 0,
            ptr: 0,
            data: [0u32; CODE_MAX_DATA],
            code: [Operation { op: 0u8, ex: 0usize}; CODE_MAX_SIZE]
        }
    }

    pub fn set(&mut self, location: usize, operation: Operation)
    {
        self.code[location] = operation;
    }

    pub fn get(&mut self, location: usize) -> Operation
    {
        self.code[location]
    }

    fn reset(&mut self)
    {
        self.pc = 0;
        self.ptr = 0;
        self.data = [0u32; CODE_MAX_DATA];
    }

    pub fn run(&mut self)
    {
        self.reset();
        while self.code[self.pc].op != OP_END {
            match self.code[self.pc].op {
                OP_VAL_INCR => self.data[self.ptr] += 1,
                OP_VAL_DECR => if self.data[self.ptr] > 0 { self.data[self.ptr] -= 1 },
                OP_PNT_INCR => self.ptr += 1,
                OP_PNT_DECR => if self.ptr > 0 { self.ptr -= 1 },
                OP_LOOP_START => {
                    if self.data[self.ptr] == 0 {
                        self.pc = self.code[self.pc].ex;
                    }
                },
                OP_LOOP_END => {
                    if self.data[self.ptr] > 0 {
                        self.pc = self.code[self.pc].ex;
                    }
                },
                OP_OUT => {
                    print!("{}", self.data[self.ptr] as u8 as char);
                }
                _ => {}
            }

            self.pc += 1;
        }
    }

    pub fn debug(&mut self)
    {
        self.reset();
        while self.code[self.pc].op != OP_END {
            match self.code[self.pc].op {
                OP_VAL_INCR => print!("{}: \t+\n", self.pc),
                OP_VAL_DECR => print!("{}: \t-\n", self.pc),
                OP_PNT_INCR => print!("{}: \t>\n", self.pc),
                OP_PNT_DECR => print!("{}: \t<\n", self.pc),
                OP_LOOP_START => {
                    print!("{}: \t[ \t{} \t{}\n", self.pc, self.data[self.ptr], self.code[self.pc].ex);
                },
                OP_LOOP_END => {
                    print!("{}: \t] \t{} \t{}\n", self.pc, self.data[self.ptr], self.code[self.pc].ex);
                },
                OP_OUT => {
                    print!("{}: \t.\n", self.pc)
                }
                _ => {}
            }

            self.pc += 1;
        }
    }
}

pub struct BrainfuckParser;
impl BrainfuckParser {
    pub fn parse(codestring: &str, program: &mut Program)
    {
        // Current program pointer
        let mut pc = 0usize;

        // Stack(Vector) used for loops
        let mut stack: Vec<usize> = Vec::new();

        for c in codestring.chars() {
            match c {
                '+' => program.set(pc, Operation { op: OP_VAL_INCR, ex: 1 }),
                '-' => program.set(pc, Operation { op: OP_VAL_DECR, ex: 1 }),
                '>' => program.set(pc, Operation { op: OP_PNT_INCR, ex: 1 }),
                '<' => program.set(pc, Operation { op: OP_PNT_DECR, ex: 1 }),
                '[' => {
                    program.set(pc, Operation { op: OP_LOOP_START, ex: 0 });
                    stack.push(pc);
                    },
                ']' => {
                    let pc_jmp = match stack.pop() {
                        None => break, // empty
                        Some(x) => x,
                    };
                    program.set(pc, Operation { op: OP_LOOP_END, ex: pc_jmp });
                    let mut op = program.get(pc_jmp);
                    op.ex = pc;
                    program.set(pc_jmp, op);
                    },
                '.' => program.set(pc, Operation { op: OP_OUT, ex: 0 }),
                ',' => program.set(pc, Operation { op: OP_IN, ex: 0 }),
                _ => {
                    if pc > 0 {
                        pc = pc - 1
                    }
                },
            };

            pc = pc + 1;
        }
        program.set(pc, Operation { op: OP_END, ex: 0});
    }


}
