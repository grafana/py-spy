use std::collections::HashMap;
use anyhow::Error;
use opentelemetry_proto::tonic::profiles::v1::ProfilesDictionary;
use opentelemetry_proto::tonic::profiles::v1::Function;
use crate::stack_trace::StackTrace;

pub struct OTLP {
     pd : ProfilesDictionary,
    strings: HashMap<String, i32>
}

impl OTLP {
    pub fn new() -> Self {
        let mut res = Self{
            pd: ProfilesDictionary::default(),
            strings: HashMap::default(),
        };
        res.pd.string_table.push("".to_string());
        res.strings.insert("".to_string(), 0);
        res
    }
    
    pub fn record(&mut self, trace: &StackTrace) -> Result<(), Error> {
        for x in &trace.frames {
            let f = Function{
                name_strindex: self.str_clone(&x.name),
                system_name_strindex: 0,
                filename_strindex: self.str_clone(&x.filename),
                start_line: 0,
            };
        }
        Ok(())
    }
    
    fn str_clone(&mut self, s: &str) -> i32 {
        match self.strings.get(s) {
            None => {
                let ids
            }
            Some(idx) => {idx}
        }
    }
}