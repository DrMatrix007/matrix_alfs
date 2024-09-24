use patitions::select_all_patitions;
use version_check_stage_2::check_all_versions_stage_2;

use super::Stage;

pub mod patitions;
pub mod version_check_stage_2;

pub struct Stage2 {}

impl Stage2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Stage for Stage2 {
    fn run(&mut self) {
        check_all_versions_stage_2();
        
        let res = select_all_patitions();
        let res = res.unwrap_or_else(|| panic!("cancelled selecting partitions"));
        println!("{:?}",res);
    }
}
