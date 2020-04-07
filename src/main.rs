mod diagnostic; use diagnostic::{parse, *};
use {std::{env, process::{Command, Stdio}, path::Path}, framework::process::Status};

fn main() -> Status {
    use clap::Clap;
    #[derive(Clap)] struct Arguments { #[clap(long = "manifest-path")] manifest_path: String }
    let arguments: Arguments = Arguments::parse();
    let mut child = Command::new("cargo").args(&["run","--message-format=json-diagnostic-rendered-ansi,json-diagnostic-short"])
                                .args(std::env::args().skip(1)).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    'parse: for msg in parse(std::io::BufReader::new(child.stdout.as_mut().unwrap())) {
        if let Message::CompilerMessage(CompilerMessage{message: msg, ..}) = msg? {
            if msg.message == "aborting due to previous error" { continue; }
            print!("{}", msg);
            if msg.spans.is_empty() { continue; }
            impl ToString for Span { fn to_string(&self) -> String { format!("{}:{}:{}", self.file_name, self.line_start, self.column_start) } }
            for span in msg.spans {
                //print!("\n{}", span.file_name);
                let path = Path::new(&arguments.manifest_path).with_file_name(&span.file_name);
                if path.exists() {
                    Command::new(env::var("EDITOR")?).arg(path).stderr(Stdio::null()).spawn()?;
                    break 'parse;
                }
            }
        }
    }
    child.wait()?.into()
}
