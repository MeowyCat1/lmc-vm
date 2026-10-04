enum ExecutionStatus {
    Success,
    Failure,
    RequestInput,
    Output(String)
}
pub struct LittleManComputer {
    pub memory: [i16; 100],
    pub pc: usize,
    pub acc: usize
}

impl LittleManComputer {
    pub fn new(instructions : [i16; 100]) -> Result<Self, ()> {
        for i in instructions {
            if i > 999 || i < 0 {
                return Err(())
            }
        }
        Ok(Self { memory: instructions, pc: 0, acc : 0})
    }
    pub fn step(&mut self) {
        let currentinstruction = self.memory[self.pc];
    }
}