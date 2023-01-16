use serde::Deserialize;
use thiserror::Error;

mod jc;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Command has no output")]
    NoOutput,
}

pub enum CmdOutput {
    Uptime,
}


impl CmdOutput {
    pub fn get_flag(&self) -> &str {
        match self {
            Self::Uptime => "--uptime"
        }
    }
}

pub trait JcWrapper<T> where T: for<'a> Deserialize<'a> {
    fn parse(&mut self, output_type: CmdOutput) -> Result<T, Box<dyn std::error::Error>>;
}
