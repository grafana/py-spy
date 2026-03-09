use anyhow::Error;

use crate::config::Config;
use crate::python_spy::PythonSpy;
use crate::stack_trace::StackTrace;

use remoteprocess::Pid;

pub fn print_traces(pid: Pid, config: &Config, parent: Option<Pid>) -> Result<(), Error> {
    let mut process = PythonSpy::new(pid, config)?;
    if config.dump_json {
        let traces = process.get_stack_traces()?;
        println!("{}", serde_json::to_string_pretty(&traces)?);
        return Ok(());
    }

    println!(
        "Process {}: {}",
        process.pid,
        process.process.cmdline()?.join(" ")
    );

    println!("Python v{} ({})", &process.version, process.process.exe()?);

    if let Some(parentpid) = parent {
        let parentprocess = remoteprocess::Process::new(parentpid)?;
        println!(
            "Parent Process {}: {}",
            parentpid,
            parentprocess.cmdline()?.join(" ")
        );
    }
    println!();
    let traces = process.get_stack_traces()?;
    for trace in traces.iter().rev() {
        print_trace(trace, true);
        if config.subprocesses {
            for (childpid, parentpid) in process
                .process
                .child_processes()
                .expect("failed to get subprocesses")
            {
                println!("\n{}", "-".repeat(80));
                // child_processes() returns the whole process tree, since we're recursing here
                // though we could end up printing grandchild processes multiple times. Limit down
                // to just once
                if parentpid == pid {
                    print_traces(childpid, config, Some(parentpid))?;
                }
            }
        }
    }
    Ok(())
}

pub fn print_trace(trace: &StackTrace, include_activity: bool) {
    let thread_id = trace.format_threadid();

    let status = if include_activity {
        format!(" ({})", trace.status_str())
    } else if trace.owns_gil {
        " (gil)".to_owned()
    } else {
        "".to_owned()
    };

    match trace.thread_name.as_ref() {
        Some(name) => {
            println!("Thread {}{}: \"{}\"", thread_id, status, name);
        }
        None => {
            println!("Thread {}{}", thread_id, status);
        }
    };

    for frame in &trace.frames {
        let filename = match &frame.short_filename {
            Some(f) => f,
            None => &frame.filename,
        };
        if frame.line != 0 {
            println!("    {} ({}:{})", &frame.name, &filename, frame.line);
        } else {
            println!("    {} ({})", &frame.name, &filename);
        }

        if let Some(locals) = &frame.locals {
            let mut shown_args = false;
            let mut shown_locals = false;
            for local in locals {
                if local.arg && !shown_args {
                    println!("        Arguments:");
                    shown_args = true;
                } else if !local.arg && !shown_locals {
                    println!("        Locals:");
                    shown_locals = true;
                }

                let repr = local.repr.as_deref().unwrap_or("?");
                println!("            {}: {}", local.name, repr);
            }
        }
    }
}
