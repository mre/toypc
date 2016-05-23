pub mod cpu;
pub mod rom;

use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;

use cpu::Cpu;
use rom::Rom;

fn lines_from_file<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>
{
    let file = File::open(filename).expect("Can't open file");
    let buf = BufReader::new(file);
    buf.lines().map(|line| line.expect("Could not parse line")).collect()
}

fn main() {
    let lines = lines_from_file("roms/big.rom");
    let rom = Rom::new(lines);
    let mut cpu = Cpu::new(rom);

    //loop {
    for i in 1..10 {
        cpu.step()
    }
}
