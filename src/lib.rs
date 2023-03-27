use std::{
    io,
    process::{Command, Output, Stdio},
};

/// Runs a pipeline of commands.
///
/// ```
/// use piper::PipedCommand;
///
/// let output = PipedCommand::run("echo goodbye | tr o b | tr b o").unwrap();
/// assert_eq!("goodoye\n", std::str::from_utf8(&output.stdout).unwrap());
/// ```
pub struct PipedCommand;

impl PipedCommand {
    pub fn run<S>(pipeline: S) -> io::Result<Output>
    where
        S: AsRef<str>,
    {
        // Parse pipeline into separate `Commands`.
        let mut cmds = pipeline
            .as_ref()
            .split('|')
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .map(|cmd| {
                let mut args = cmd.split_whitespace();
                let mut cmd = Command::new(args.next().unwrap());
                for arg in args {
                    cmd.arg(arg);
                }
                cmd
            })
            .collect::<Vec<_>>();

        // `Command` panics on empty string, so I guess this is fine.
        let mut prev = cmds[0].stdout(Stdio::piped()).spawn()?;

        // Chain the commands.
        for cmd in cmds.iter_mut().skip(1) {
            prev = cmd
                .stdin(Stdio::from(prev.stdout.unwrap()))
                .stdout(Stdio::piped())
                .spawn()?;
        }

        prev.wait_with_output()
    }
}
