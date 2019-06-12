use std::error::Error;
use std::convert::AsRef;
use std::path::Path;

use crate::error::ConfigReadError;

#[derive(Debug)]
pub struct ReplyCode {
    pub code: u32,
    pub reply: String
}

impl ReplyCode {
    fn new<S: Into<String>>(code: u32, reply: S) -> Self {
        ReplyCode {
            code: code,
            reply: reply.into()
        }
    }
}

pub fn generate_reply_codes<P: AsRef<Path>>(csv_file: P) -> Result<Vec<ReplyCode>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(&csv_file)?;
    let mut reply_codes: Vec<ReplyCode> = Vec::new();

    for result in reader.records() {
        let record = result?;
        let code: u32;
        let reply: &str;

        match record.get(0) {
            Some(code_str) => {
                code = code_str.parse::<u32>()?
            }
            None => {
                let error = ConfigReadError::new(&csv_file, format!("invalid record {:?}", record));
                return Err(Box::new(error));
            }
        };

        match record.get(1) {
            Some(reply_str) => {
                reply = reply_str
            }
            None => {
                let error = ConfigReadError::new(&csv_file, format!("invalid record {:?}", record));
                return Err(Box::new(error));
            }
        };

        reply_codes.push(ReplyCode::new(code, reply));
    }

    Ok(reply_codes)
}

