Weird feature union issues with Cargo, reproduction

# Summary

- `my-core` has features `foo` and `bar`
  - if `foo` is enabled, `my_core::print_stuff` prints `foo`
  - if `bar` is enabled, `my_core::print_stuff` prints `bar`
  - these two are additive - if both are enabled, `foo \n bar` is printed
- `foo-crate` depends on `my-core` with `foo` enabled
- `bar-crate` depends on `my-core` with `bar` enabled
- `cargo run -p foo-crate` outputs just `foo`
- `cargo run -p bar-crate` outputs just `bar`

The issue:
- `cargo run --bin foo-crate` outputs `foo \n bar`
  - so `bar` is enabled for some reason
- likewise, `cargo run --bin bar-crate` outputs `foo \n bar` as well
  - so `foo` is enabled for some reason
