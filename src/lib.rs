use std::process::Command;

pub struct App {
    pub command: String,
    pub args: Vec<String>
}

pub fn run_app<'a>(app: &App) -> Result<std::process::ExitStatus, std::io::Error> {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
