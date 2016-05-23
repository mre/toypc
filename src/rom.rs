use std::ops::Index;

pub struct Rom {
    instructions: Vec<String>,
}

impl Rom {
    pub fn new(instructions: Vec<String>) -> Rom {
        Rom { instructions: instructions }
    }

    pub fn get(&mut self, index: usize) -> String {
        self.instructions[index].clone()
    }
}

// Make access to the ROM data more convenient
// This does not work. I guess because the size of instructions[index]
// is not known at compile time...
// Maybe I get this to work later.
// impl Index<usize> for Rom {
//    type Output = str;
//
//    fn index<'a>(&'a self, index: usize) -> &'a str {
//        &self.instructions[index][..]
//    }
// }
//
