use bitvec::prelude::*;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    value: i32,
}

impl Instruction {
    fn parse(raw: &str) -> Option<Self> {
        let mut split = raw.split(" ");
        let op = match split.next()? {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            "nop" => Operation::Nop,
            _ => return None,
        };
        let value = split.next()?.parse().ok()?;
        Some(Instruction { op, value })
    }
}

struct Machine {
    acc: i32,
    program: Vec<Instruction>,
    // due to type shenanigans it's more convenient to have the cursor be signed
    // and convert it to usize when indexing.
    // cursor is assumed to always be positive (otherwise conversion fails and program panics)
    cursor: i32,
}

impl Machine {
    fn with_program(instructions: impl Into<Vec<Instruction>>) -> Self {
        Machine {
            acc: 0,
            program: instructions.into(),
            cursor: 0,
        }
    }

    fn cursor(&self) -> usize {
        self.cursor as usize
    }

    fn step(&mut self) {
        let inst = &self.program[self.cursor()];
        use Operation::*;
        match inst.op {
            Acc => self.acc += inst.value,
            // jmp is value - 1 because of the regular increment
            // of the cursor below
            Jmp => self.cursor += inst.value - 1,
            Nop => (),
        }
        self.cursor += 1;
    }

    fn has_terminated(&self) -> bool {
        self.cursor() >= self.program.len()
    }

    // returns to initial state of the program
    fn reset(&mut self) {
        self.cursor = 0;
        self.acc = 0;
    }

    // executes the program, stops and returns true if it starts to loop,
    // false if program terminates normally
    fn detect_loop(&mut self) -> bool {
        let mut loop_detection = vec![false; self.program.len()];
        while !self.has_terminated() && !loop_detection[self.cursor()] {
            loop_detection[self.cursor()] = true;
            self.step();
        }
        !self.has_terminated()
    }

    fn detect_loop_bitvec(&mut self, loop_detection: &mut BitVec) -> bool {
        while !self.has_terminated() && !loop_detection[self.cursor()] {
            loop_detection.set(self.cursor(), true);
            self.step();
        }
        !self.has_terminated()
    }
}

pub fn part1(input: &str) -> i32 {
    let instructions: Vec<_> = input
        .lines()
        .map(|l| Instruction::parse(l).expect("invalid instruction"))
        .collect();
    let mut machine = Machine::with_program(instructions);
    machine.detect_loop(); // assumed we find a loop for this part
    machine.acc
}

// pub fn part1_bitvec(input: &str) -> i32 {
//     let instructions: Vec<_> = input
//         .lines()
//         .map(|l| Instruction::parse(l).expect("invalid instruction"))
//         .collect();
//     let mut machine = Machine::with_program(instructions);
//     machine.detect_loop_bitvec(); // assumed we find a loop for this part
//     machine.acc
// }

pub fn part2(input: &str) -> i32 {
    let instructions: Vec<_> = input
        .lines()
        .map(|l| Instruction::parse(l).expect("invalid instruction"))
        .collect();
    let nops_jmps: Vec<usize> = instructions
        .iter()
        .enumerate()
        .filter_map(|(i, inst)| match inst.op {
            Operation::Jmp | Operation::Nop => Some(i),
            _ => None,
        })
        .collect();
    let mut machine = Machine::with_program(instructions);
    for curr_attempt in nops_jmps.into_iter() {
        let curr_inst = &mut machine.program[curr_attempt];
        let original = curr_inst.op;
        curr_inst.op = match original {
            Operation::Jmp => Operation::Nop,
            Operation::Nop => Operation::Jmp,
            _ => unreachable!(),
        };
        let loops = machine.detect_loop();
        if loops {
            machine.reset();
            machine.program[curr_attempt].op = original;
        } else {
            break;
        }
    }
    machine.acc
}

pub fn part2_bitvec(input: &str) -> i32 {
    let instructions: Vec<_> = input
        .lines()
        .map(|l| Instruction::parse(l).expect("invalid instruction"))
        .collect();
    let nops_jmps: Vec<usize> = instructions
        .iter()
        .enumerate()
        .filter_map(|(i, inst)| match inst.op {
            Operation::Jmp | Operation::Nop => Some(i),
            _ => None,
        })
        .collect();
    let mut loop_detection = bitvec![0; instructions.len()];
    let mut machine = Machine::with_program(instructions);
    for curr_attempt in nops_jmps.into_iter() {
        let curr_inst = &mut machine.program[curr_attempt];
        let original = curr_inst.op;
        curr_inst.op = match original {
            Operation::Jmp => Operation::Nop,
            Operation::Nop => Operation::Jmp,
            _ => unreachable!(),
        };
        let loops = machine.detect_loop_bitvec(&mut loop_detection);
        if loops {
            loop_detection.set_all(false);
            machine.reset();
            machine.program[curr_attempt].op = original;
        } else {
            break;
        }
    }
    machine.acc
}
