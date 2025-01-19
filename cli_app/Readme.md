
# CLI APP

In this project we will get to know how to read environment variables, configure files in to single unified application configuration.

Learn how to add CLI commads to applications to perfoem simple tasks like running migrations easier.

#### Dependencies
```
[dependencies]
clap = "4"
anyhow = "1"
```

To create a new subcommand with clap you have to do two things:

add the subcommand to the clap configuration
handle the different commands according to command-line parameters


#### Commands

```
cargo build
```

```
./target/debug/cli_app
```

```
Usage: cli_app [COMMAND]

Commands:
  hello  Hello World!
  serve  Start HTTP server
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```