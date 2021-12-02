# hasty
A development tool which "reloads" code instantly upon changes.

Hasty works regardless of language/technology, as long as your project can be built from the command line.
Reloading in hasty is customizable, allowing it to support almost any use-case.

## Installation (Linux, Requires cargo)
```
> git clone https://github.com/t0a5ted/hasty.git
> cd hasty
> cargo build --release
```
To test (continuing from above steps):
```
> ./target/release/hasty dummy_dir/ -c "echo reload!"
// Edit or create a file under the "dummy_dir" directory.
// This should print "reload!" to stdout.
```

For ease of use, you might want to make an alias.
In Bash, add the following line to your `.bashrc` file

```
alias hasty="./path/to/hasty"
```
You can now call hasty from anywhere using `hasty`.

## Usage
`hasty` takes 2 arguments, FILE and COMMAND (-c).
If any of the 2 arguments are not supplied, hasty will use the `.hastyrc` file in the cwd.

You could do something like:
```
hasty dummy_dir/ -c "echo reload!"
```
`.hastyrc` files use the JSON syntax, and are useful in large-scale collaborative projects.
An equivilant `.hastyrc` file would look like this:
```
{
	"dir": "dummy_dir/",
	"command": "echo reload!"
}
```

Run `hasty -h` for more details.

## Features
- [x] File watching and command execution
- [x] Config File
- [ ] Proper Error Handling instead of calling `.unwrap()` on everything

> Stdin is not supported by rust's command execution, and any attempt by the child process to read from stdin will break.
> See [here](https://doc.rust-lang.org/std/process/struct.Command.html#method.output) for more info.

