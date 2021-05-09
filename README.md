# GVLC

The Gentle Vim Lisp Compiler is a compiler for the Vim Lisp programming language.

Vim Lisp is a high level language targetting Vim Script, in order to make plugins creation and configuration more pleasant to do.

## Missing commands

If you miss a special syntax for a command, fill free to created an issue to request it.

## Installation

```bash
$ git clone https://github.com/wafelack/gvlc
$ cd gvlc/
$ cargo test
$ cargo build --release
$ cp target/release/gvlc /wherever/you/want
```

## License

GVLC is licensed under the GNU General Public License version 3.0 and later.
