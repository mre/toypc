use std::ops::{AddAssign, MulAssign, DivAssign};
use rom::Rom;

#[derive(Debug)]
pub enum Instruction {
    Hlf{ rid: RegisterId }, // sets register r to half its current value, then continues with the next instruction.
    Inc, // increments register r, adding 1 to it, then continues with the next instruction.
    Jie, // jumps if register r is even
    Jio, // jumps if register r is 1
    Jmp, // jump: continue with the instruction offset away relative to itself
    Tpl, // sets register r to triple its current value, then continues with the next instruction.
}

#[derive(Debug)]
pub struct Register {
    val: u64,
}

impl Register {
    fn new() -> Register {
        Register { val: 0 }
    }
}

impl AddAssign<u64> for Register {
    fn add_assign(&mut self, _rhs: u64) {
        self.val += _rhs;
    }
}

impl MulAssign<u64> for Register {
    fn mul_assign(&mut self, _rhs: u64) {
        self.val *= _rhs;
    }
}

impl DivAssign<u64> for Register {
    fn div_assign(&mut self, _rhs: u64) {
        self.val /= _rhs;
    }
}

#[derive(Debug)]
pub enum RegisterId {
    A,
    B,
}

pub struct Cpu {
    a: Register,
    b: Register,
    pc: usize,
    rom: Rom,
}

impl Cpu {
    pub fn new(rom: Rom) -> Cpu {
        Cpu {
            a: Register::new(),
            b: Register::new(),
            pc: 0,
            rom: rom,
        }
    }

    /// Execute the next instruction
    pub fn step(&mut self) {
        let data = self.rom.get(self.pc);
        let instruction = self.read(data);
        self.exec(instruction);
    }
    
    /// Parse raw data into instruction
    fn read(&mut self, data: String) -> Instruction {
        let tokens: Vec<&str> = data.split(' ').collect();
        let (opcode, args) = tokens.split_at(1);
        
        let instruction = match opcode[0] {
           "hlf"  => self.read_hlf(args),
          // "inc" => Opcode::Inc,
           //"jie" => Opcode::Jie,
           //"jio" => Opcode::Jio,
           //"jmp" => Opcode::Jmp,
           //"tpl" => Opcode::Tpl,
            _ => panic!("unimplemented or illegal instruction: {}", data),
        };
        
        match instruction {
            Ok(instruction) => instruction,
            Err(reason) => panic!("Cannot parse opcode: {}. Reason: {}", data, reason),
        }
    }
    
    /// Parse hlf instruction
    fn read_hlf(&mut self, args: &[&str]) -> Result<Instruction, String>{
        if args.len() != 1 {
            return Err("Invalid number of arguments".to_string());
        }
        match args[0] {
            "a," => Ok(Instruction::Hlf{rid: RegisterId::A}),
            "b," => Ok(Instruction::Hlf{rid: RegisterId::B}),
            _ => Err("Invalid register name".to_string())
        }
    }
    
    /// Run instruction on CPU
    fn exec(&mut self, instruction: Instruction) {
        println!("{:?}", instruction);
    }

    fn get_register(&mut self, r: RegisterId) -> &mut Register {
        match r {
            RegisterId::A => &mut self.a,
            RegisterId::B => &mut self.b,
        }
    }

    /// hlf r sets register r to half its current value, then continues with the next instruction.
    fn hlf(&mut self, rid: RegisterId) {
        {
            let register = self.get_register(rid);
            *register /= 2;
        }
        self.pc += 1;
    }

    // tpl r sets register r to triple its current value, then continues with the next instruction.
    fn tpl(&mut self, rid: RegisterId) {
        let register = self.get_register(rid);
        *register *= 3;
    }

    // inc r increments register r, adding 1 to it, then continues with the next instruction.
    fn inc(&mut self, rid: RegisterId) {
        let register = self.get_register(rid);
        *register += 1;
    }

    // jmp offset is a jump; it continues with the instruction offset away relative to itself.
    // jie r, offset is like jmp, but only jumps if register r is even ("jump if even").

    // jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).
    // fn jio(&mut self, rid: Register) {
    //  let register = self.get_register(rid);

    // }
}

#[cfg(test)]
mod tests {
    use super::RegisterId;
    use super::Cpu;
    use rom::Rom;

    #[test]
    fn test_hlf() {
        let tests = [(10, 0), (7, 0), (0, 4)];
        let results = [(5, 0), (3, 0), (0, 2)];
        for (&(a, b), &(ar, br)) in tests.iter().zip(results.iter()) {
            let mut cpu = Cpu::new(Rom::new(vec![]));
            cpu.a += a;
            cpu.b += b;
            cpu.hlf(RegisterId::A);
            cpu.hlf(RegisterId::B);
            assert_eq!(ar, cpu.a.val);
            assert_eq!(br, cpu.b.val);
        }
    }

    #[test]
    fn test_tpl() {
        let tests = [(10, 0), (0, 0)];
        let results = [(30, 0), (0, 0)];
        for (&(a, b), &(ar, br)) in tests.iter().zip(results.iter()) {
            let mut cpu = Cpu::new(Rom::new(vec![]));
            cpu.a += a;
            cpu.b += b;
            cpu.tpl(RegisterId::A);
            cpu.tpl(RegisterId::B);
            assert_eq!(ar, cpu.a.val);
            assert_eq!(br, cpu.b.val);
        }
    }

    #[test]
    fn test_inc() {
        let tests = [(1, 0), (1, 1)];
        let results = [(2, 1), (2, 2)];
        for (&(a, b), &(ar, br)) in tests.iter().zip(results.iter()) {
            let mut cpu = Cpu::new(Rom::new(vec![]));
            cpu.a += a;
            cpu.b += b;
            cpu.inc(RegisterId::A);
            cpu.inc(RegisterId::B);
            assert_eq!(ar, cpu.a.val);
            assert_eq!(br, cpu.b.val);
        }
    }
}
