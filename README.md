# typed_ecs

[![no_std](https://img.shields.io/badge/no__std-supported-brightgreen)](https://docs.rust-embedded.org/book/intro/no-std.html)
[![Alloc-free](https://img.shields.io/badge/no__alloc-supported-brightgreen)](https://doc.rust-lang.org/alloc/)
[![License](https://img.shields.io/badge/license-MIT-orange)](https://github.com/heydocode/typed_ecs)

A `no_std` and `no-alloc` tiny ECS written in Rust, which provides strong compile-time guarantees.

This project is in active development but technically is ready to be used, just note that its API will break often until v0.1.x releases.

Also, its ergonomicity and flexibility, despite the compile-time guarantees, will improve over time.

## About

`typed_ecs` is a tiny, zero-cost framework that:

- lets you wire plugins together at **compile time**, without runtime overhead
- enforces plugin collection / plugin / shared-data compatibility via Rust trait bounds (no runtime checking)
- is written for `no_std` and `no_alloc` environments, but still optimized for very powerful machines (profiling built-in, multithreading and async coming! (TODO!))
- contains no runtime (only a scheduler): every plugin gets optimized by the compiler to the point that the produced assembly is comparable to a hand-written loop (see the godbolt section below)
- has no dependencies! (Profiling dependencies are not taken into account, because these are enabled on feature, and are not advised in releases)

## Key features

- `no_std` compatible, and `no-alloc`: all kinds of platforms supported!
- Compile-time plugin composition (in plugin collections)
- Strong compile-time guarantees for plugin/SharedData compatibility
- Zero runtime-registration or reflection — everything resolved at compile time
- Light, optimized scheduler loop, with **direct** plugin calls (no indirection, no v-tables, and other overhead)
- Ergonomic design: no tricky or cryptic code needed to use `typed_ecs` (only one essential and transparent macro)
- Flexible: you can publish third-party plugins that other users could add in their applications in a plug & play manner (in the future, this process will be more documented)

## Examples

Explore the examples to see `typed_ecs` in action:

```sh
git clone https://github.com/heydocode/typed_ecs.git
cd typed_ecs
cargo run --example hello_world
```

Check the `examples/` directory for more comprehensive examples, including:

- `hello_world.rs`: Plugin definition and message on startup
- `plugin_collection.rs`: Explanation of how to use plugin collections
- `profile.rs`: Usage of the crate's built-in profiling

## Compile time optimizations demonstration

You may yourself test how `typed_ecs` examples produce neat, highly optimized
assembly, by putting the [`godbolt_analysis.rs`](godbolt_analysis.rs) file contents into [`Godbolt`](https://godbolt.org/).

Moreover, this file contains guidance for analyzing other examples and how to export the crate into one file, 
then how append the example code in it, using [`cargo-expand`](https://crates.io/crates/cargo-expand)

Alternatively, you can go [to this link](https://godbolt.org/z/1KeTjExh3) (Godbolt) in order to view these optimizations without pasting anything. Though, note that this link is updated by hand, and may be outdated!

In the future, a special util will be developed, in order to automate the process of creating a mono, godbolt ready file, for assembly analysis purposes.

## Profiling with [`tracing`](https://github.com/tokio-rs/tracing)

### Example

There's a `examples/profile.rs` example for that! Check it out with:
```sh
git clone https://github.com/heydocode/typed_ecs.git
cd typed_ecs
# (forest tracing backend - zero setup needed compared to Tracy)
cargo run --example profile --features=profile-forest
```

### Why?

Tracing crate allows to easily profile Rust applications, a process that helps finding and resolving performance bottlenecks. Profilers play a critical role in optimizing software, and producing performant applications / games.

### Note on profiled "dead" systems execution time

When you'll profile your app that uses `typed_ecs`, you'll notice something: some "dead" (= empty) systems seem to execute, and seem to not be fully optimized away, but that's the case! When you see a system running for 200-1000ns, depending on the profiler and your hardware, that's simply the begin recording function overhead. In fact, even when the system is completely optimized away, the recording function (in the case of systems: inside the `on_system_start` hook) takes time to create a span, then send it before dropping it, and finally execute the `Drop` implementation of the span, which purpose is to stop the timer. This process takes time and therefore it seems that systems that should have been optimized away are still there (which is, again, not the case). And if you wonder why if it's optimized away, you see the current schedule, system name, and even its plugin displayed, that's the work of the proc-macro `generate_collection`!

Obviously, this kind of overhead is completely vanishing when the `profile` feature is disabled.

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

After the benches, you should see an html report at `target/criterion/report/index.html`.

If you don't see it, try cleaning the building artefacts and re-benching:

```sh
cargo clean
cargo bench
```

Repeat this a few times until you see the HTML report. (It may be just a personal issue that it doesn't get generated at the first time, but I prefer leaving it there in case it's not).

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
