use crate::stack_trace::StackTrace;
use anyhow::Error;
use opentelemetry_proto::tonic::profiles::v1::{Function, Line, Location, Mapping};
use opentelemetry_proto::tonic::profiles::v1::ProfilesDictionary;
use std::collections::HashMap;
use std::hash::{Hash};

pub struct OTLP {
    pd: ProfilesDictionary,
    strings: HashMap<String, i32>,//todo think of ways avoiding String clone into this HashMap
    functions: HashMap<FunctionMirror, i32>,
    locations: HashMap<LocationMirror, i32>,
}
const DUMMY_MAPPING_IDX: i32 = 0;
const DUMMY_MAPPING: Mapping = Mapping{
    memory_start: 0,
    memory_limit: 0,
    file_offset: 0,
    filename_strindex: 0,
    attribute_indices: vec![],
    has_functions: false,
    has_filenames: false,
    has_line_numbers: false,
    has_inline_frames: false,
};
impl OTLP {
    pub fn new() -> Self {
        let mut res = Self {
            pd: ProfilesDictionary::default(),
            strings: HashMap::default(),
            functions: HashMap::default(),
            locations: HashMap::default(),
        };
        res.str("".to_string());
        res.pd.mapping_table.push(DUMMY_MAPPING);
        res
    }

    pub fn record(&mut self, trace: &StackTrace) -> Result<(), Error> {
        for x in &trace.frames {
            let f = FunctionMirror {
                name_strindex: self.str(x.name.clone()),//todo just move the whole StackTrace here and dont clone
                filename_strindex: self.str(x.filename.clone()),//todo just move the whole StackTrace here and dont clone
            };
            let l = LocationMirror{
                function_index: self.fun(f),
                line: x.line,
            };
            _ = l;
        }
        Ok(())
    }

    fn str(&mut self, s: String) -> i32 {
        // match self.strings.get(&s) {
        //     None => {
        //         let idx = self.pd.string_table.len() as i32;
        //         self.pd.string_table.push(s.clone());//todo avoid clone here
        //         self.strings.insert(s, idx);
        //         idx
        //     }
        //     Some(idx) => *idx,
        // }
        Self::insert(&mut self.strings, &mut self.pd.string_table, s)
    }
    
    fn fun(&mut self, fm: FunctionMirror) -> i32 {
        Self::insert(&mut self.functions, &mut self.pd.function_table, fm)
    }
    fn loc(&mut self, lm: LocationMirror) -> i32 {
        Self::insert(&mut self.locations, &mut self.pd.location_table, lm)
    }
    fn insert<M, V>(hm: &mut HashMap<M, i32>, table: & mut Vec<V>, m: M) ->i32
    where
        M: PartialEq + Eq + Hash + Clone,
        V: From<M>,
    {
        match hm.get(&m) {
            None => {
                let idx = table.len() as i32;
                table.push(m.clone().into());//todo think how this clone can be avoided for strign table
                hm.insert(m, idx);
                idx
            }
            Some(idx) => *idx,
        }
    }

}

#[derive(PartialEq, Clone, Eq, Hash)]
struct FunctionMirror {
    pub name_strindex: i32,
    pub filename_strindex: i32,
}

impl From<FunctionMirror> for Function {
    fn from(m: FunctionMirror) -> Self {
        Self{
            name_strindex: m.name_strindex,
            system_name_strindex: 0,
            filename_strindex: m.filename_strindex,
            start_line: 0,
        }
    }
}


#[derive(PartialEq, Clone, Eq, Hash)]
struct LocationMirror {
    function_index: i32,
    line: i32,
}

impl From<LocationMirror> for Location {
    fn from(m: LocationMirror) -> Self {
        Self{
            mapping_index: Some(DUMMY_MAPPING_IDX),
            address: 0,
            line: vec![
                Line{
                    function_index: m.function_index,
                    line: m.line as i64,
                    column: 0,
                }
            ],
            is_folded: false,
            attribute_indices: vec![],
        }
    }
}

