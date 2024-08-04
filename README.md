# `box2c_sys`

This crate contains Rust bindings to **Box2D v3.0** (`box2c`).

We are using [our fork of `box2c`](https://github.com/Carroted/box2c), which makes only one change: increasing the max translation/rotation settings, which are hardcoded into `box2c`.

Lastly, this crate implements certain traits for `box2c` types, such as `Hash`, `PartialEq`, etc. We add these whenever they are needed for our projects which use this crate.

## License

MIT or Apache2 at your option, but note that `box2c` itself is licensed under MIT
