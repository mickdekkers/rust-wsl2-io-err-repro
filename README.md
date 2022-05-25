# rust-wsl2-io-err-repro

This is a reproduction repo for an I/O error that occurs when piping output of a `x86_64-pc-windows-gnu` Rust program to a file in WSL 2

## Error reproduction

Clone this repo and cd into it.

From [WSL 2](https://docs.microsoft.com/en-us/windows/wsl/install) on Windows, run:

```sh
$ cargo build && ./target/x86_64-pc-windows-gnu/debug/rust_wsl2_io_err_repro.exe > stdout.log 2> stderr.log
```

You'll notice that the `stdout.log` is cut off and that the `stderr.log` contains this error:

```log
fatal runtime error: I/O error: operation failed to complete synchronously
```

## Version info

```
rustup 1.24.3 (ce5817a94 2021-05-31)
cargo 1.63.0-nightly (a4c1cd0eb 2022-05-18)
rustc 1.63.0-nightly (9257f5aad 2022-05-21)
```

WSL version 2 on Windows 10 21H2 (OS Build 19044.1706)

## Observations

Interestingly, the error still occurs when you pipe to just `stderr.log`:

```sh
$ cargo build && ./target/x86_64-pc-windows-gnu/debug/rust_wsl2_io_err_repro.exe 2> stderr.log
```

And also when you pipe to just `stdout.log`:

```sh
$ cargo build && ./target/x86_64-pc-windows-gnu/debug/rust_wsl2_io_err_repro.exe > stdout.log
```

But not when you just run the command without piping to a file:

```sh
$ cargo build && ./target/x86_64-pc-windows-gnu/debug/rust_wsl2_io_err_repro.exe
```

It also does not occur when you change the target to `x86_64-unknown-linux-gnu` and use this command:

```sh
$ cargo build && ./target/x86_64-unknown-linux-gnu/debug/rust_wsl2_io_err_repro > stdout.log 2> stderr.log
```

Try changing the amount of characters printed per line. It looks like the error occurs more often the longer the lines are.

I'm not sure what's causing the error, but the code that prints it and aborts the process is found here https://github.com/rust-lang/rust/blob/9fed13030c2a2ebd79bfb1fd8be4f768cbe8c9d9/library/std/src/sys/windows/handle.rs#L255-L258

## Background

I normally use VS Code + WSL (2) for development on Windows. In this case I was working on a CLI app for Windows, so I compiled it using the `x86_64-pc-windows-gnu` target. I had the build/run command set to pipe the output to an `out.log` file for convenience from the beginning but started noticing the error intermittently and more regularly as I kept working on the project. This repo provides a minimal reproduction of the error.
