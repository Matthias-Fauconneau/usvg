mod diagnostic; use diagnostic::{parse, *};
use {std::{env, process::{Command, Stdio}, path::Path}, framework::process::Status};

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
    let mut child = Command::new("cargo").args(&["run","--message-format","JSON"]).args(std::env::args().skip(1)).stdout(Stdio::piped())/*.stderr(Stdio::null())*/.spawn()?;
    let mut stdout = std::io::BufReader::new(child.stdout.take().unwrap());
    'parse: for msg in parse(&mut Tee::new(&mut stdout, std::io::stdout())) {
        if let Message::CompilerMessage(CompilerMessage{message: msg, ..}) = msg? {
            print!("\n{}", msg.message);
            if msg.spans.is_empty() { continue; }
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
    loop {
        if let Some(status) = child.try_wait()? { return Ok(status).into() }
        std::io::copy(&mut stdout, &mut std::io::stdout())?;
    }
}
