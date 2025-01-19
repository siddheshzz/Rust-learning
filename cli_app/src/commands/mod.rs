use clap::{ArgMatches, Command};
mod hello;
mod server;
pub fn configure(command: Command) -> Command {
    command
    .subcommand(hello::configure())
    .subcommand(server::configure())
    .arg_required_else_help(true)
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some((cmd, _matches)) = matches.subcommand() {
        match cmd {
            hello::COMMAND_NAME => hello::handle(matches)?,
            server::COMMAND_NAME => server::handle(matches)?,
            &_ => {}
        }
    }

    Ok(())
}
