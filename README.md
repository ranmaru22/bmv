# bmv - Batch Move

![Build](https://github.com/ranmaru22/bmv/workflows/Build/badge.svg)
![Release](https://github.com/ranmaru22/bmv/workflows/Release/badge.svg)
![crates.io](https://img.shields.io/crates/v/bmv.svg)

**Rename large numbers of files easily.**

**bmv** is a Regex-based move/rename tool that can handle many files and directories
at once.

## Usage
bmv commands are formend in a *from-to* pattern, similar to `sed` but in a simpler
way. For example, `sed -i 's/foo/bar/' <files>` would be `bmv 'foo' 'bar' <files>`
in bmv. The replacement is then done across *all* matches.

```sh
bmv '(.*)\.foo' '$1.bar' ./**/*
```

You can use parentheses to capture any regular expression match in the first
argument and then refer to it using `$n`, where `n` is a number from 1 to to the
total amount of groups you have, in the second.

You can also use named capture groups, if you prefer, using the `?P<name>`
syntax. The example above would then look like this:

```sh
bmv '(?P<filename>.*)\.foo' '$filename.bar' ./**/*
```

If you want to use a literal dollar sign in the second argument, use `$$`.

### Look-arounds don't work.
Yes, this is because Rust's [regex crate](https://docs.rs/regex/latest/regex/)
does not support them and bmv uses it for its matching. Like the crate's docs
say, "[i]n exchange, all searches execute in linear time with respect to the
size of the regular expression and search text."

### It does not match hidden files when using globs!
bmv by itself does not handle resolving the glob, it just receives a list of
files it's supposed to operate on from standard input. So in order to include
hidden files, you need to make sure the shell you're using includes them into
your glob. In `zsh` this can easily be done by using the `(D)` modifier.

```sh
  bmv 'foo' 'bar' ./**/*(D)
```

In `bash` you can set `dotglob`.

## Installation
### Manually from source
You need to have Rust and Cargo installed.

```sh
git clone https://github.com/ranmaru22/bmv.git
cd bmv
cargo install --locked --path=.
```

This will install the `master` branch which has the latest stable release. If
you want the development version, switch to branch `develop`.

```sh
git checkout develop
```

## From the Cargo registry
You need to have Rust and Cargo installed.

```sh
cargo install bmv --locked
```

## From a package manager
*tbd*

## Disclaimer
bmv is provided as-is and with no warranty. I developed it for mostly personal
use, so it likely has some gotchas and bugs. Feel free to hack it (and if you
like, send back a PR, I will happily include any improvements) but use it at
your onw risk.
