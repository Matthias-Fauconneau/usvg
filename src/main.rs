mod diagnostic; use diagnostic::{parse, *};
use {std::{process::{Command, Stdio, ExitStatus}}, framework::{core::Ok, process::Status}};

// Maps cargo errors/warning to file:line
fn main() -> Status {
    let mut child = Command::new("cargo").args(&["build","--message-format=json-diagnostic-rendered-ansi"/*short*/]).args(std::env::args().skip(1)).stdout(Stdio::piped()).spawn()?;
    for msg in parse(std::io::BufReader::new(child.stdout.take().unwrap())) { match msg? {
        Message::CompilerMessage(CompilerMessage{message: Diagnostic{message, spans, rendered, ..}, ..}) => {
            let _ = child.kill(); // Kill on first warning/error to save power/heat
            if message == "aborting due to previous error" { continue; }
            eprint!("{}",rendered.ok()?);
            for span in spans {
                println!("{}:{}:{}", span.file_name, span.line_start, span.column_start);
            }
            use std::os::unix::process::ExitStatusExt; return Status::from(ExitStatus::from_raw(-1));
        },
        _=>{},
    }}
    child.wait()?.into() // Forward cargo status (success on warnings)
}
