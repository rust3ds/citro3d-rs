# citro3d-rs

⚠️ WIP ⚠️

Rust bindings and safe wrapper to the [citro3d](https://github.com/devkitPro/citro3d)
and [citro2d](https://github.com/devkitPro/citro2d) library, to write homebrew graphical programs for the Nintendo 3DS.

## Crates

* [`citro3d-sys`](./citro3d-sys) - C bindings to `libcitro3d`
  ([docs](https://rust3ds.github.io/citro3d-rs/crates/citro3d_sys))
* [`citro3d`](./citro3d) - safe Rust wrappers for `citro3d-sys` (WIP)
  ([docs](https://rust3ds.github.io/citro3d-rs/crates/citro3d))
* [`citro3d-macros`](./citro3d-macros/) – helper proc-macros for `citro3d`

## License

* `citro3d-sys` and `citro2d-sys` is licensed under Zlib
* `citro3d` and `citro3d-macros` are dual-licensed under MIT or Apache-2.0
