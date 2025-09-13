# typed_ecs

[![no_std](https://img.shields.io/badge/no__std-supported-brightgreen)](https://docs.rust-embedded.org/book/intro/no-std.html)
[![Alloc-free](https://img.shields.io/badge/alloc--free-supported-brightgreen)](https://doc.rust-lang.org/alloc/)
[![License](https://img.shields.io/badge/license-MIT-orange)](https://github.com/heydocode/typed_ecs)

A `no_std` and `no-alloc` tiny ECS written in Rust, which provides strong compile-time guarantees.

This project is in active development but is ready to be used, just note that its API will break often until v0.1.x releases.

## About

`typed_ecs` is a tiny, zero-cost framework that:

- lets you wire plugins together at **compile time** using tuple-based plugin lists
- enforces plugin / shared-data compatibility via Rust trait bounds (no runtime checking)
- is written for `no_std` and `no_alloc` environments (small, deterministic runtime)

## Key features

- `no_std` compatible, and `no-alloc`: all kinds of platforms supported
- Compile-time plugin composition
- Strong compile-time guarantees for plugin/SharedData compatibility
- Zero runtime-registration or reflection — everything resolved at compile time
- Light, optimized runtime loop, with direct plugin calls
- Ergonomic design: no tricky or cryptic code needed to use `typed_ecs`

## Examples

Explore the examples to see `typed_ecs` in action:

```shell
git clone https://github.com/heydocode/typed_ecs.git
cd typed_ecs
cargo run --example hello_world
```

Check the `examples/` directory for more comprehensive examples, including:

- `hello_world.rs`: Plugin definition and message on startup

## Contributing

We welcome contributions! Whether you're interested in:

- Reporting bugs
- Writing code
- Improving documentation
- Enhancing CI/CD pipelines
- Adding tests
- Creating a dedicated website

Please check our [Contribution Guidelines](CONTRIBUTING.md) first.

For larger contributions or significant changes (like creating a website), we recommend:

1. Opening an issue using the [Question template](.github/ISSUE_TEMPLATE/question.md)
2. Discussing your approach with maintainers
3. Getting alignment on design and implementation details

This ensures your efforts align with project goals and standards. For smaller fixes like documentation tweaks or test additions, feel free to submit a PR directly.
