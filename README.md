# zigcli

A build dependency for running the `zig` build tool to compile a native
library.

```toml
# Cargo.toml
[build-dependencies]
zigcli = "0.1.0"
```

The Zig executable is assumed to be `zig` unless the `ZIG`
environmental variable is set.

## Implementation status

The following commands of the `zig` build tool are available
at the time of writing:

- [x] `zig build`
- [ ] `zig fetch`
- [ ] `zig init`
- [ ] `zig build-exe`
- [ ] `zig build-lib`
- [ ] `zig build-obj`
- [ ] `zig test`
- [ ] `zig run`
- [ ] `zig ast-check`
- [ ] `zig fmt`
- [ ] `zig reduce`
- [ ] `zig translate-c`
- [ ] `zig ar`
- [ ] `zig cc`
- [ ] `zig c++`
- [ ] `zig dlltool`
- [ ] `zig lib`
- [ ] `zig ranlib`
- [ ] `zig objcopy`
- [ ] `zig env`
- [ ] `zig version`

### Caveats

- The `zig` build system may introduce breaking changes at any moment.
  This utility tries to keep up to date with the `master` branch.
- Cross compilation with `cross` is broken at the moment.

# License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in zigcli by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
