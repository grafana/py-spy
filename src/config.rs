use remoteprocess::Pid;

/// Options on how to collect samples from a python process
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    /// Whether or not we should stop the python process when taking samples.
    /// Setting this to false will reduce the performance impact on the target
    /// python process, but can lead to incorrect results like partial stack
    /// traces being returned or a higher sampling error rate
    pub blocking: LockingStrategy,

    /// Whether or not to profile native extensions. Note: this option can not be
    /// used with the nonblocking option, as we have to pause the process to collect
    /// the native stack traces
    pub native: bool,

    // The following config options only apply when using py-spy as an application
    #[doc(hidden)]
    pub command: String,
    #[doc(hidden)]
    pub pid: Option<Pid>,
    #[doc(hidden)]
    pub python_program: Option<Vec<String>>,
    #[doc(hidden)]
    pub sampling_rate: u64,
    #[doc(hidden)]
    pub filename: Option<String>,
    #[doc(hidden)]
    pub format: Option<FileFormat>,
    #[doc(hidden)]
    pub show_line_numbers: bool,
    #[doc(hidden)]
    pub duration: RecordDuration,
    #[doc(hidden)]
    pub include_idle: bool,
    #[doc(hidden)]
    pub include_thread_ids: bool,
    #[doc(hidden)]
    pub subprocesses: bool,
    #[doc(hidden)]
    pub gil_only: bool,
    #[doc(hidden)]
    pub hide_progress: bool,
    #[doc(hidden)]
    pub capture_output: bool,
    #[doc(hidden)]
    pub dump_json: bool,
    #[doc(hidden)]
    pub dump_locals: u64,
    #[doc(hidden)]
    pub full_filenames: bool,
    #[doc(hidden)]
    pub lineno: LineNo,
    #[doc(hidden)]
    pub refresh_seconds: f64,
    #[doc(hidden)]
    pub core_filename: Option<String>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FileFormat {
    flamegraph,
    raw,
    speedscope,
    chrometrace,
}

impl std::str::FromStr for FileFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "flamegraph" => Ok(FileFormat::flamegraph),
            "raw" => Ok(FileFormat::raw),
            "speedscope" => Ok(FileFormat::speedscope),
            "chrometrace" => Ok(FileFormat::chrometrace),
            _ => Err(format!("Invalid fileformat: {s}")),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LockingStrategy {
    NonBlocking,
    #[allow(dead_code)]
    AlreadyLocked,
    Lock,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RecordDuration {
    Unlimited,
    Seconds(u64),
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum LineNo {
    NoLine,
    First,
    LastInstruction,
}

impl Default for Config {
    /// Initializes a new Config object with default parameters
    #[allow(dead_code)]
    fn default() -> Config {
        Config {
            pid: None,
            python_program: None,
            filename: None,
            format: None,
            command: String::from("top"),
            blocking: LockingStrategy::Lock,
            show_line_numbers: false,
            sampling_rate: 100,
            duration: RecordDuration::Unlimited,
            native: false,
            gil_only: false,
            include_idle: false,
            include_thread_ids: false,
            hide_progress: false,
            capture_output: true,
            dump_json: false,
            dump_locals: 0,
            subprocesses: false,
            full_filenames: false,
            lineno: LineNo::LastInstruction,
            refresh_seconds: 1.0,
            core_filename: None,
        }
    }
}
