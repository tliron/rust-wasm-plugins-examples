Rust Wasm Plugins Example
=========================

A Great Fit
-----------

You've probably heard that [Wasm](https://web.dev/explore/webassembly) (WebAssembly) can be a great
way to support plugins in your application. Plugin authors can write them in any Wasm-compatible
language and you're off to the races with your choice among various excellent and safe Wasm runtimes for
Rust, including ones optimized for embedded environments (e.g. [wasmi](https://github.com/wasmi-labs/wasmi)).

Not So Easy
-----------

Unfortunately, you're going to find out (in early 2025) that examples of this often-mentioned use case
are hard to come by, and that so much of the documentation is irrelevant, confusing, incomplete, or just
out of date, as things have been moving quite quickly in the Wasm world.

If you've read Surma's [Rust to WebAssembly the hard way](https://surma.dev/things/rust-to-webassembly/)
(highly recommended starting point!) then you might feel quite confident in your ability to build
`.wasm` modules, load them into Rust, call functions in them, and expose functions to them. But the hard
way becomes a dead end as you realize something quite critical: Wasm only supports the transfer of just
primitive numeric types, namely integers and floats (and not even unsigned integers). This is an intentional
and understandable design choice to keep Wasm lean and mean and agnostic to any specific implementation.

But this means that if you want to transfer something as basic as a string or a vector then you'll
have to delve deep into the the Wasm memory model. People have come up with various solutions for Rust,
from piggy-backing on [std::ffi::CString](https://doc.rust-lang.org/std/ffi/struct.CString.html) to
exposing custom malloc/free functions to the Wasm module. But not only are these solutions painful,
they would obviously need to be ported to every language we want to support, each with its own string
and array models. There was, and still is, a need for some kind of standard, built on top of Wasm, that
would support higher-level constructs in a portable way.

The Temporary Solutions
-----------------------

It took some time for the community to rally around one. For a while, a promising proposal was
Wasm Interfaces (WAI). This was [pioneered by Wasmer](https://github.com/wasmerio/wai), where the
[documentation](https://docs.wasmer.io/wai) still points to it as "the" solution (early 2025). As
usual in the Wasm world, even that documentation can only take you so far. None of it actually
mentions hosting WAI in Rust! And it only shows importing interfaces, not exporting them, though I
have managed to learn how to handle exports by delving into the WAI tooling source code. The idea behind
WAI is that you describe your interface in a [`.wai` file](https://github.com/wasmerio/wai/blob/main/WAI.md)
and use tooling (e.g. macros) to generate the boilerplate code for clients and hosts, a lot like how
things work with RPC protocols (e.g. protobufs).

WAI had not been widely adopted, however it does work and is also quite straightforward. We won't be
using it in this example, but it's useful to be aware of its existence.

Also check out [Extism](https://extism.org/), a more comprehensive attempt to fill in the gap.

The Consensus Solution
----------------------

But the consensus now seems to be around the
[Wasm Component Model](https://component-model.bytecodealliance.org/), which expands on the WAI
proposal with proper namespacing, resources, and richer custom data types. The Component Model is
actually part of WASI, and indeed is being used to provide the WASI extensions. So, what's
[WASI](https://wasi.dev/)? It's an initiative by the community to deliver a set of common APIs on top
of Wasm for accessing streams, like files and stdout/stderr, network sockets, and eventually threads.
I say "eventually" because WASI is still very much a work in progress. As of now (early 2025) we just
got "preview 2" of it. Luckily, Rust can target "wasip2", meaning that it can be used to create the
latest and greatest Components. Though, note that wasip2 does produce larger minimal `.wasm` files
than WAI due to the inclusion of the machinery for the Component Model.

Like WAI, the Component Model relies on an interface definition file,
[`.wit`](https://component-model.bytecodealliance.org/design/wit.html).
And [Wasmtime](https://wasmtime.dev/) has the tooling for it! Yay! So, are we finally off to the
races with our plugin system?

Not so fast. Again, finding examples and straightforward documentation is not easy. Wasmtime is a
very comprehensive and performative implementation, but it's also designed by committee and has
a lot of contributors. And due to the fast-moving nature of these things, what you find might not
represent what is actually going on or what you should be using.

Finally We Get to the Point
---------------------------

All that to say, that's why I created this repository. It's intended to be a minimal and
straightforward example of how to build plugins in Rust (as Components) and how to host them in
your application using Wasmtime and its WIT tooling. Well, at least for early 2025... As of now
it does not demonstrate the more advanced features of WIT, such as custom data types, but I might
add those in the future.

How to Build and Run
--------------------

There are two crates here, a plugin, which builds into a wasip2 `.wasm`, and a host, which can be
built into any Rust target architecure.

```sh
rustup target add wasm32-wasip2
git clone https://github.com/tliron/rust-wasm-plugins-examples.git
cd rust-wasm-plugins-examples
cargo build --package=plugin --target=wasm32-wasip2 --release
cargo run --package=host
```

Things to See
-------------

* [Rust to WebAssembly the hard way](https://surma.dev/things/rust-to-webassembly/) by Surma
* [Plugins with Rust and WASI Preview 2](https://benw.is/posts/plugins-with-rust-and-wasi) by Ben Wishovich

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](LICENSE-APACHE)
* [MIT license](LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
