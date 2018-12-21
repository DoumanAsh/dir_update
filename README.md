# dir_update

[![Build status](https://ci.appveyor.com/api/projects/status/fd2vix6ilxkj509v/branch/master?svg=true)](https://ci.appveyor.com/project/DoumanAsh/dir-update/branch/master)
[![Build Status](https://travis-ci.org/DoumanAsh/dir_update.svg?branch=master)](https://travis-ci.org/DoumanAsh/dir_update)
[![Crates.io](https://img.shields.io/crates/v/dir_update.svg)](https://crates.io/crates/dir_update)

Simple utility to copy files into directory, only if they are updated

## Usage

```bash
dir_update 0.1.0
Copy files into directory, if they are updated

USAGE:
    dir_update.exe [FLAGS] <FROM> <TO>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Enables verbose output to print each action.

ARGS:
    <FROM>    Directory from which to copy files.
    <TO>      Directory into which to copy files.
```
