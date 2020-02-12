/*#![allow(incomplete_features)]#![feature(const_generics,maybe_uninit_extra,maybe_uninit_ref,non_ascii_idents)]*/
mod diagnostic; use diagnostic::{parse, *};
use std::{env, process::{Command, Stdio}};

fn main() -> framework::core::Result {
    let mut child = Command::new("cargo").args(&["build","--message-format","JSON"]).stdout(Stdio::piped())/*.stderr(Stdio::null())*/.spawn()?;
    let mut stdout = std::io::BufReader::new(child.stdout.as_mut().unwrap());
    let mut editor = None;
    for msg in parse(&mut stdout) {
        if let Message::CompilerMessage(CompilerMessage{message: msg, ..}) = msg? {
            if msg.spans.is_empty() { continue; }
            impl ToString for Span { fn to_string(&self) -> String { format!("{}:{}:{}", self.file_name, self.line_start, self.column_start) } }
            if editor.is_none() { editor = Some(Command::new(env::var("EDITOR")?).args(&[msg.spans[0].to_string()]).stderr(Stdio::null()).spawn()?); }
            //println!("{:?}", msg.spans.iter().map(Span::to_string));
            print!("\n{}", msg.message);
        }
    }
    child.wait()?;
    Ok(())
}
