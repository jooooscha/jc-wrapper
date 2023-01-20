use crate::{JcWrapper, CmdOutput, Error};
use std::process::{Stdio, Command};
use serde::Deserialize;

impl<T: for<'a> Deserialize<'a>> JcWrapper<T> for Command {

    /// Called on a Command.
    fn parse(&mut self, output_type: CmdOutput) -> Result<T, Error> {

        let output = self.stdout(Stdio::piped())
            .spawn().map_err(|_| Error::CouldNotSpawnCommand)?;
        
        if let Some(text) = output.stdout {
            let jc = Command::new("jc")
                .arg(output_type.get_flag())
                .stdin(Stdio::from(text))
                .output();

            let jc = match jc {
                Ok(output) => output,
                Err(err) => { return Err(Error::CouldNotSpawnJc(err)); },
            };


            let parsed_output = serde_json::from_slice(&jc.stdout).map_err(|_| Error::CouldNotParse)?;

            Ok(parsed_output)
        } else {
            Err(Error::NoOutput.into())
        }
    }

}
