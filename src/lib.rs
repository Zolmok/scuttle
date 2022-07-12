use std::fmt::{self, Formatter, Display};
use std::process::{Command, ExitStatus, Output};

pub struct Args(pub Vec<String>);
pub struct App {
    pub command: String,
    pub args: Vec<String>
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(" "))
    }
}

/// Run an app and display its output
///
/// This is useful when you just want to run an app and don't care about parsing its output
///
/// # Arguments
///
/// * `app` - An app of type `App`
///
/// return an `ExitStatus` struct
pub fn run_status<'a>(app: &App) -> Result<ExitStatus, std::io::Error> {
    let child_result = Command::new(app.command.clone())
        .args(app.args.clone())
        .spawn();
    let mut child = match child_result {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    match child.try_wait() {
        Ok(Some(status)) => Ok(status),
        Ok(None) => match child.wait() {
            Ok(result) => Ok(result),
            Err(error) => return Err(error),
        },
        Err(error) => Err(error),
    }
}

/// Run an app and return its output
///
/// This is usefull when you need to parse the return of an app
///
/// # Arguments
///
/// * `app` - An app of type `App`
///
/// return an `Output` struct
pub fn run_output<'a>(app: &App) -> Result<Output, std::io::Error> {
    Command::new(app.command.clone())
        .args(app.args.clone())
        .output()
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
