use crate::numbers::LMCInt;

enum ExecutionStatus {
    Success,
    Failure,
    RequestInput,
    Output(String)
}
pub struct LittleManComputer {
    pub memory: [LMCInt; 100],
    pub pc: LMCInt,
    pub acc: LMCInt,
}

impl LittleManComputer {
    pub fn new(instructions : [LMCInt; 100]) -> Self {
        Self { memory: instructions, pc: LMCInt::new(0), acc : LMCInt::new(0)}
    }
    pub fn step(&mut self) {
        let currentinstruction = self.memory[self.pc.num as usize];
    }
}