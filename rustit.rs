/**
 * A Rust reimplementation of basic Git. Educational?
 */

use std::env;
use std::process::Command;
use std::process::ExitStatus;

// TODO - Add return codes for scripty usage
fn main() {
  // Grab the name of the subcommand, and its arguments.
  let args: Vec<String> = env::args().collect();
  let command: String = args[1].to_string();
  let command_args: Vec<String> = args[2..].to_vec();

  // TODO: handle -h, --help, help to show usage....
  let rc = launch_command(format!("rustit-{}", command), command_args);

  rc.unwrap();
}

fn launch_command(command: String, args: Vec<String>) -> Result<ExitStatus, std::io::Error> {
  // Invoke the binary in $PATH with the name in it - rustit-$name - plus the args.
  // TODO - collect rc?????
  let rc = Command::new(command)
    .env("PATH", "/home/emily/src/rustgit")
    .args(args)
    .status();   // blocking, don't capture the error.

  return rc;
}
