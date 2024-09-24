
pub mod stage2;
pub trait Stage {
    fn run(&mut self);
}

pub struct StageRunner {
    stages: Vec<Box<dyn Stage>>,
}

impl StageRunner {
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }
    pub fn add(&mut self, s: impl Stage + 'static) {
        self.stages.push(Box::new(s));
    }
    
    pub fn run_all(&mut self) {
        for stage in &mut self.stages {
            stage.run();
        }
    }
}
