# citro3d-sys

Rust bindings to [`citro3d`](https://github.com/devkitPro/citro3d).
Bindings are generated at build time using the locally-installed devkitPro.

[Documentation](https://rust3ds.github.io/citro3d-rs/crates/citro3d_sys) is generated from the
`main` branch, and should generally be up to date with the latest devkitPro.
This will be more useful than [docs.rs](https://docs.rs/crates/citro3d), since
the bindings are generated at build time and `docs.rs`' build environment does not
have a copy of devkitPro to generate bindings from.
