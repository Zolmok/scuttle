use std::fmt::{self, Formatter, Display};
use std::process::Command;

struct Args(Vec<String>);
pub struct App {
    pub command: String,
    pub args: Vec<String>
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(" "))
    }
}

pub fn run_app<'a>(app: &App) -> Result<std::process::ExitStatus, std::io::Error> {
    println!("");
    println!("========================");
    println!("$ {} {}", app.command, Args(app.args.to_owned()));
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
        Err(error) => Err(error),
    }
}

pub fn run_apps(apps: &[App]) {
    for app in apps.iter() {
        match run_app(app) {
            Err(error) => panic!("panic{}", error),
            Ok(_status) => continue,
        };
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
