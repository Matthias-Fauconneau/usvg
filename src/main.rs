mod diagnostic;

// Maps cargo errors/warning to file:line
use std::process::{Command, Stdio};
fn main() -> Result<(),std::io::Error> {
	let mut child = Command::new("cargo").args(std::env::args().skip(1))
																	.arg("--message-format=json-diagnostic-rendered-ansi")
                                                                    .stdout(Stdio::piped()).spawn()?;
	use diagnostic::{parse, Message, CompilerMessage, Diagnostic};
    for msg in parse(std::io::BufReader::new(child.stdout.take().unwrap())) { match msg? {
        Message::CompilerMessage(CompilerMessage{message: Diagnostic{message, spans, rendered: Some(rendered), ..}, ..}) => {
            let _ = child.kill(); // Kill on first warning/error to save power/heat
            if message == "aborting due to previous error" { continue; }
            eprint!("{}", rendered);
            for span in spans {
                if std::path::Path::new(&span.file_name).exists() { println!("{}:{}:{}", span.file_name, span.line_start, span.column_start); }
            }
			std::process::exit(-1);
        },
        _=>{},
    }}
    std::process::exit(child.wait()?.code().unwrap_or(-1));
}
