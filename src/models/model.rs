pub struct Program {
    pub name: String,
    pub vendor: String
}


pub enum ProgramType {
    Game(Program),
    Work(Program),
    Other(Program)
}