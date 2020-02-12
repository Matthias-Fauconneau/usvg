mod diagnostic; use diagnostic::{parse, *};
use {std::{env, process::{Command, Stdio}, path::Path}, framework::process::Status};

fn main() -> Status {
    let mut child = Command::new("cargo").args(&["build","--message-format","JSON"]).args(std::env::args().skip(1)).stdout(Stdio::piped()).stderr(Stdio::null()).spawn()?;
    let mut stdout = std::io::BufReader::new(child.stdout.as_mut().unwrap());
    'parse: for msg in parse(&mut stdout) {
        if let Message::CompilerMessage(CompilerMessage{message: msg, ..}) = msg? {
            if msg.spans.is_empty() { continue; }
            print!("\n{}", msg.message);
            impl ToString for Span { fn to_string(&self) -> String { format!("{}:{}:{}", self.file_name, self.line_start, self.column_start) } }
            for span in msg.spans {
                //print!("\n{}", span.file_name);
                if Path::new(&span.file_name).exists() {
                    Command::new(env::var("EDITOR")?).arg(span.to_string()).stderr(Stdio::null()).spawn()?;
                    break 'parse;
                }
            }
        }
    }
    Ok(child.wait_with_output()?.status).into()
}
