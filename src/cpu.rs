use std::ops::{AddAssign, MulAssign, DivAssign};
use std::str::FromStr;
use rom::Rom;

#[derive(Debug)]
pub struct Register {
    val: u64,
}

impl Register {
    fn new() -> Register {
        Register { val: 0 }
    }
    
    fn is_odd(&self) -> bool {
        self.val % 2 == 0
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
        self.exec(data);
    }
    
    /// Parse raw data into instruction
    fn exec(&mut self, data: String) {
        let tokens: Vec<&str> = data.split(' ').collect();
        let (opcode, args) = tokens.split_at(1);
        
        println!("{}", data);
        match opcode[0] {
           "hlf"  => self.read_hlf(args),
           "inc" => self.read_inc(args),
           //"jie" => self.read_jie(args),
           "jio" => self.read_jio(args),
           "jmp" => self.read_jmp(args),
           "tpl" => self.read_tpl(args),
            _ => panic!("unimplemented or illegal instruction: {}", data),
        };
    }
    
    /// Parse hlf instruction
    fn read_hlf(&mut self, args: &[&str]) {
        if args.len() != 1 {
            panic!("Invalid number of arguments");
        }
        match args[0] {
            "a" => self.hlf(RegisterId::A),
            "b" => self.hlf(RegisterId::B),
            _ => panic!("Invalid register name {:?}", args),
        }
    }
    
    /// Parse inc instruction
    fn read_inc(&mut self, args: &[&str]) {
        if args.len() != 1 {
            panic!("Invalid number of arguments");
        }
        match args[0] {
            "a" => self.inc(RegisterId::A),
            "b" => self.inc(RegisterId::B),
            _ => panic!("Invalid register name {:?}", args),
        }
    }
    
    /// Parse inc instruction
    fn read_tpl(&mut self, args: &[&str]) {
        if args.len() != 1 {
            panic!("Invalid number of arguments");
        }
        match args[0] {
            "a" => self.tpl(RegisterId::A),
            "b" => self.tpl(RegisterId::B),
            _ => panic!("Invalid register name {:?}", args),
        }
    }

    /// Parse jmp instruction
    fn read_jmp(&mut self, args: &[&str]) {
        if args.len() != 1 {
            panic!("Invalid number of arguments");
        }
        let (sign, offset_str) = args[0].split_at(1);
        if sign != "+" {
            panic!("Unexpected sign {}", sign);
        }
        let offset = usize::from_str(offset_str).unwrap();
        self.jmp(offset);
    }

    /// Parse jio instruction
    fn read_jio(&mut self, args: &[&str]) {
        if args.len() != 2 {
            panic!("Invalid number of arguments");
        };
        
        {
            let register = match args[0] {
                "a," => &self.a,
                "b," => &self.b,
                _ => panic!("Invalid register name {:?}", args),
            };
            
            if !register.is_odd() {
                self.pc += 1;
                return;
            }
        }
        
        // TODO: Avoid duplicate code
        let (sign, offset_str) = args[1].split_at(1);
        if sign != "+" {
            panic!("Unexpected sign {}", sign);
        }
        let offset = usize::from_str(offset_str).unwrap();
        self.jmp(offset);
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

    /// tpl r sets register r to triple its current value, then continues with the next instruction.
    fn tpl(&mut self, rid: RegisterId) {
        {
            let register = self.get_register(rid);
            *register *= 3;
        }
        self.pc += 1;
    }

    /// inc r increments register r, adding 1 to it, then continues with the next instruction.
    fn inc(&mut self, rid: RegisterId) {
        {
            let register = self.get_register(rid);
            *register += 1;
        }
        self.pc += 1;
    }

    /// jmp offset is a jump; it continues with the instruction offset away relative to itself.
    fn jmp(&mut self, offset: usize) {
        self.pc += offset;
    }

    // jie r, offset is like jmp, but only jumps if register r is even ("jump if even").

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
