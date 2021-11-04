use std::fmt::{self, Formatter, Display};
use std::process::Command;

struct Args<'a>(Vec<&'a str>);
pub struct App<'a> {
    pub command: String,
    pub args: Vec<&'a str>
}

impl Display for Args<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(" "))
    }
}

pub fn run_app<'a>(app: &App) -> Result<std::process::ExitStatus, std::io::Error> {
    println!("");
    println!("========================");
    println!("$ {} {}", app.command, Args(app.args.clone()));
    println!("========================");

    let child_result = Command::new(app.command.clone())
        .args(app.args.clone())
        .spawn();
    let mut child = child_result.unwrap();

    match child.try_wait() {
        Ok(Some(status)) => Ok(status),
        Ok(None) => {
            Ok(child.wait().unwrap())
        },
        Err(error) =>  Err(error),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
