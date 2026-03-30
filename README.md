# typed_ecs

[![no_std](https://img.shields.io/badge/no__std-supported-brightgreen)](https://docs.rust-embedded.org/book/intro/no-std.html)
[![Alloc-free](https://img.shields.io/badge/no__alloc-supported-brightgreen)](https://doc.rust-lang.org/alloc/)
[![License](https://img.shields.io/badge/license-MIT-orange)](https://github.com/heydocode/typed_ecs)

A `no_std` and `no-alloc` compatible tiny ECS written in Rust, which provides strong compile-time guarantees.

This project is in active development but technically is ready to be used, just note that its API will break often until v0.1.x releases.

## About

`typed_ecs` is a tiny, zero-cost framework that:

- lets you wire plugins together at **compile time**, without runtime overhead
- is written for `no_std` and `no_alloc` environments, but also for capable machines (profiling, async and parallel execution built-in)
- contains no runtime (only a defined during compile-time execution logic): every plugin gets optimized by the compiler to the point that the produced assembly is comparable to a hand-written loop (see the godbolt section below)
- allows you to do async IO stuff without extra boilerplate

## Key features

- `no_std` compatible, and `no-alloc`: all kinds of platforms supported!
- Compile-time plugin composition (in plugin collections)
- Strong compile-time guarantees
- Zero runtime-registration or reflection — everything resolved at compile time
- Light, optimized scheduler loop, with **direct** plugin calls (no indirection, no v-tables, and other overhead)
- Built-in parallelism: on platforms supporting it, all (!) non-applying systems are runned in parallel
- Async aboard! No need to do cursed stuff to handle IO: it's tightly integrated with typed_ecs!
- Ergonomic design: no tricky or cryptic code needed to use `typed_ecs`: only one transparent macro
- Community-ready: you can publish third-party plugins that other users could add in their applications in a plug & play manner (in the future, this process will be more documented)

## Examples

Explore the examples to see `typed_ecs` in action:

```sh
git clone https://github.com/heydocode/typed_ecs.git
cd typed_ecs
cargo run --example hello_world
```

Check the `examples/` directory for more comprehensive examples, including:

- `hello_world.rs`: Plugin definition and message on startup
- `plugin_collection.rs`: Explanation of how to build a plugin collection
- `profile.rs`: Usage of the crate's built-in profiling

## Profiling with [`tracing`](https://github.com/tokio-rs/tracing)

### Example

There's an `examples/profile.rs` example for that! Check it out with:
```sh
git clone https://github.com/heydocode/typed_ecs.git
cd typed_ecs
# (forest tracing backend - zero setup needed compared to Tracy)
cargo run --example profile --features=profile-forest
```

### Why?

Tracing crate allows to easily profile Rust applications, a process that helps finding and resolving performance bottlenecks. Profilers play a critical role in optimizing software, and producing performant software. 

### Built-in `tracing` backends

`typed_ecs` offers two backends out of the box:

- [`tracing-tracy`](https://docs.rs/tracing-tracy/latest/tracing_tracy/) (`profile-tracy` feature) - Tracy is an advanced open-source profiling tool. It's available for Windows (GitHub releases) and Linux (build yourself or other distributors), and maybe other platforms. When the program exits, it waits until a Tracy instance captures the trace, and only then terminates.
- [`tracing-forest`](https://docs.rs/tracing-forest/latest/tracing_forest/) (`profile-forest` feature) - A terminal trace producer. Works out of the box, with no external application required. It is the preferred option for quick setup & profiling. Lacks Tracy tooling.

Backends where some span data is lost and therefore unusable with `typed_ecs` (create an issue if you want to use some of them):

- [`tracing-chrome`](https://docs.rs/tracing-chrome/latest/tracing_chrome/)
- [`tracing-flame`](https://docs.rs/tracing-flame/latest/tracing_flame/)

## Benchmarks

`typed_ecs` implements [`criterion`](https://docs.rs/criterion/latest/criterion/) benchmarks. You can see them at [`benches`](benches)

You can run these benchmarks with `cargo bench`. If you wish profiling the benches, you may
want to enable a profiler via a dedicated feature (`profile-tracy` for Tracy or `profile-forest` for a TUI profiler).

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

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.