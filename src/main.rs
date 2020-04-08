mod diagnostic; use diagnostic::{parse, *};
use {std::{env, process::{Command, Stdio}, path::Path}, framework::{core::Ok, process::Status}};

pub struct Tee<R, W> {reader: R, writer: W}
impl<R: std::io::Read, W: std::io::Write> Tee<R, W> { pub fn new(reader: R, writer: W) -> Self { Self{reader: reader, writer: writer} } }
impl<R: std::io::Read, W: std::io::Write> std::io::Read for Tee<R, W> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.reader.read(buf)?;
        self.writer.write_all(&buf[..n])?;
        Ok(n)
    }
}

fn main() -> Status {
    use clap::Clap;
    #[derive(Clap)] struct Arguments { #[clap(long = "manifest-path")] manifest_path: String }
    let arguments: Arguments = Arguments::parse();
    let mut child = Command::new("cargo").args(&["build","--message-format=json-diagnostic-rendered-ansi"]) // json-diagnostic-short
                                .args(std::env::args().skip(1)).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    'parse: for msg in parse(std::io::BufReader::new(child.stdout.as_mut().unwrap())) { match msg? {
    // 'parse: for msg in parse(&mut Tee::new(child.stdout.as_mut().unwrap(), std::io::stdout())) { match msg? {
        Message::CompilerMessage(CompilerMessage{message: Diagnostic{message, spans, rendered, ..}, ..}) => {
            if message == "aborting due to previous error" { continue; }
            print!("{}",rendered.ok()?);
            for span in spans {
                let path = Path::new(&arguments.manifest_path).with_file_name(&span.file_name);
                if path.exists() {
                    let span = format!("{}:{}:{}", path.to_str().unwrap(), span.line_start, span.column_start);
                    print!("\n{}", span);
                    Command::new(env::var("EDITOR")?).arg(span).stdout(Stdio::null()).stderr(Stdio::null()).spawn()?;
                    break 'parse;
                }
            }
        },
        Message::CompilerArtifact(Artifact{target: Target{name, ..}, fresh, ..}) => if !fresh { println!("{}", name); },
        _=>{},
    }}
    child.wait()?.into()
}
