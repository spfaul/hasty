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
> ./target/release/hasty dummy_dir/ "sh -c echo reload!"
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
Run `hasty -h` for more details.

