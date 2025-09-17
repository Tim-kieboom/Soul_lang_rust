use std::sync::{Arc, Mutex};
use crate::steps::step_interfaces::i_sementic::soul_fault::SoulFault;
use crate::steps::step_interfaces::i_parser::parser_response::ParserResponse;
use crate::steps::step_interfaces::i_sementic::sementic_response::SementicResponse;
use crate::{run_options::run_options::RunOptions, utils::{logger::Logger, time_logs::TimeLogs}};

pub fn generate_code(
    run_options: &Arc<RunOptions>, 
    logger: &Arc<Logger>, 
    time_logs: &Arc<Mutex<TimeLogs>>,
) -> Vec<SoulFault> {
    
    
    todo!()
}

pub fn sementic_analyse(
    parser: ParserResponse, 
    run_options: &Arc<RunOptions>, 
    logger: &Arc<Logger>, 
    time_logs: &Arc<Mutex<TimeLogs>>,
) -> SementicResponse {
    
    todo!()
}


