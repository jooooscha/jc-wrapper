use crate::{JcWrapper, CmdOutput, Error};
use std::process::{Stdio, Command};
use serde::Deserialize;

impl<T: for<'a> Deserialize<'a>> JcWrapper<T> for Command {

    fn parse(&mut self, output_type: CmdOutput) -> Result<T, Box<dyn std::error::Error>> {

        let output = self.stdout(Stdio::piped())
            .spawn()?;
        
        if let Some(text) = output.stdout {
            let jc = Command::new("jc")
                .arg(output_type.get_flag())
                .stdin(Stdio::from(text))
                .output()?;

            Ok(serde_json::from_slice(&jc.stdout)?)
        } else {
            Err(Error::NoOutput.into())
        }

    }

}
