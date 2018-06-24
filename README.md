# Rustler Bad Tag

Reproducing a core dump due with rustler.

Note: This has been resolved ([see](https://github.com/hansihe/rustler/issues/144)).

## Run Example

```
$ mix deps.get
$ mix run -e RustlerCoreDump.test
Compiling NIF crate :rustler_core_dump (native/rustler_core_dump)...
    Finished release [optimized] target(s) in 0.0 secs
#Reference<0.1720102286.1584136196.121188>
[1]    23077 segmentation fault (core dumped)
```
