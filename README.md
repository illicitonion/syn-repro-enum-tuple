To reproduce:

Stable:
```
$ cargo --version
cargo 1.30.0 (36d96825d 2018-10-24)
$ cargo run
   Compiling syn-repro-enum-tuple v0.1.0 (/home/dwh/src/github.com/illictonion/syn-repro-enum-tuple)
error: unexpected token
 --> src/main.rs:8:14
  |
8 |     Zero = (0, 1).0,
  |              ^

error: aborting due to previous error

error: Could not compile `syn-repro-enum-tuple`.

To learn more, run the command again with --verbose.
```

Nightly:
```
$ cargo +nightly --version
cargo 1.32.0-nightly (1fa308820 2018-10-31)
$ cargo +nightly run
   Compiling syn-repro-enum-tuple v0.1.0 (/home/dwh/src/github.com/illictonion/syn-repro-enum-tuple)
error: unexpected token
 --> src/main.rs:8:14
  |
8 |     Zero = (0, 1).0,
  |              ^

error: aborting due to previous error

error: Could not compile `syn-repro-enum-tuple`.

To learn more, run the command again with --verbose.
```

If you remove the `UsingSyn` derive, the code works (even if it's awful code which no one should ever write).
