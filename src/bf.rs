
const CODE_MAX_SIZE: usize = 8192;
const CODE_MAX_DATA: usize = 4096;

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
struct BrainfuckOperation {
    op: u8,
    ex: usize
}

pub struct BrainfuckProgram {
    code: [BrainfuckOperation; CODE_MAX_SIZE]
}

pub struct BrainfuckParser;
impl BrainfuckParser {
    pub fn parse(codestring: &str) -> BrainfuckProgram
    {
        let mut program = BrainfuckProgram {
            code: [BrainfuckOperation { op: 0u8, ex: 0usize}; CODE_MAX_SIZE],
        };

        // Current program pointer
        let mut pc = 0usize;

        // Stack(Vector) used for loops
        let mut stack: Vec<usize> = Vec::new();

        for c in codestring.chars() {
            match c {
                '+' => program.code[pc] = BrainfuckOperation { op: OP_VAL_INCR, ex: 1 },
                '-' => program.code[pc] = BrainfuckOperation { op: OP_VAL_DECR, ex: 1 },
                '>' => program.code[pc] = BrainfuckOperation { op: OP_PNT_INCR, ex: 1 },
                '<' => program.code[pc] = BrainfuckOperation { op: OP_PNT_DECR, ex: 1 },
                '[' => {
                    program.code[pc] = BrainfuckOperation { op: OP_LOOP_START, ex: 0 };
                    stack.push(pc);
                    },
                ']' => {
                    let pc_jmp = match stack.pop() {
                        None => break, // empty
                        Some(x) => x,
                    };
                    program.code[pc] = BrainfuckOperation { op: OP_LOOP_END, ex: pc_jmp };
                    program.code[pc_jmp].ex = pc;
                    },
                '.' => program.code[pc] = BrainfuckOperation { op: OP_OUT, ex: 0 },
                ',' => program.code[pc] = BrainfuckOperation { op: OP_IN, ex: 0 },
                _ => {
                    if pc > 0 {
                        pc = pc - 1
                    }
                },
            };

            pc = pc + 1;
        }
        program.code[pc] = BrainfuckOperation { op: OP_END, ex: 0};

        program
    }


}

pub struct BrainfuckInterpreter {
    pc: usize,
    ptr: usize,
    data: [u32; CODE_MAX_DATA],
}
impl BrainfuckInterpreter
{
    pub fn new() -> BrainfuckInterpreter
    {
        BrainfuckInterpreter {
            pc: 0,
            ptr: 0,
            data: [0u32; CODE_MAX_DATA],
        }
    }

    pub fn run(&mut self, program: BrainfuckProgram) {
        while program.code[self.pc].op != OP_END {
            match program.code[self.pc].op {
                OP_VAL_INCR => self.data[self.ptr] += 1,
                OP_VAL_DECR => self.data[self.ptr] -= 1,
                OP_PNT_INCR => self.ptr += 1,
                OP_PNT_DECR => self.ptr -= 1,
                OP_LOOP_START => {
                    if self.data[self.ptr] == 0 {
                        self.pc = program.code[self.pc].ex;
                    }
                },
                OP_LOOP_END => {
                    if self.data[self.ptr] > 0 {
                        self.pc = program.code[self.pc].ex;
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
}
