libosu
======

[![Crates.io](https://img.shields.io/crates/d/libosu.svg)][1]
[![Documentation](https://docs.rs/libosu/badge.svg)][2]
[![dependency status](https://deps.rs/repo/github/mzhang28/libosu/status.svg)][3]

General-purpose osu! library

Installation
------------

This package is hosted on [crates.io][1]. In order to include this library into
your project, simply add this line into your `Cargo.toml`:

```rust
libosu = "*"
```

The following features are available through adding features in `Cargo.toml`,
and are not included by default since they may bring in extra dependencies:

- `apiv1`: Bindings for the osu! API v1.
- `apiv2`: Bindings for the osu! API v2.
- `replay-data`: Action data for osu! replay files (requires LZMA).

Getting Started
---------------

Check out the [API Documentation][2] for details on how to use the various
functions, or check out some of the examples (pending).

Projects using libosu
---------------------

- [editor](https://github.com/mzhang28/editor): wip osu editor
- [mapping-tools](https://github.com/mzhang28/mapping-tools): wip port of mapping tools to rust

If you have a project using libosu, open an issue with a brief description and
I'll add it to the list!

Contact
-------

Primary maintainer: Michael Zhang

See [contributors page][4] for full list of authors.

License: MIT

[1]: https://crates.io/crates/libosu
[2]: https://docs.rs/libosu
[3]: https://deps.rs/repo/github/mzhang28/libosu
[4]: https://github.com/mzhang28/libosu/graphs/contributors
