# Task


This is a simple CLI task manager made to help people focus on their priorities.

```
❯ task help
Usage: task [COMMAND]

Commands:
  new         Creates a new task
  update      Updates an already existing task, i.e opens it in vim
  delete      Deletes an existing task
  list        Lists all exising tasks
  toggle      Toggles the completion of a task
  widget      Updates the widget (Needs widget-loc)
  info        Puts the Title and the description in a 'less'
  widget-loc  Sets the location of the widget
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information

```

## Installation

Cargo provides a nifty little feature to install from github, you can run

```cargo install --git https://github.com/rithulkamesh/task```

## Contributing

Please refer to [Contributing.md](./CONTRIBUTING.md).
All contributions will be taken under GNU GPL v3 only.


