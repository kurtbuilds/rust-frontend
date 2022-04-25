# rust-frontend
Experiements in an API for Rust frontend that very closely mimics javascript

# Run

```
npm run serve
```


# Miscellaneous Notes

### Desired properties

- HMR
- Speed as a feature
- API that closely mimics JS. Minimize how much someone has to learn
- Smooth onramp from a JS project
- html! macro with bare words for text.

### Other

- HMR (Perseus does this - https://github.com/arctic-hen7/perseus )
- Speed of wasm-bindgen
    - Docs here: https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html
    - Speedtest src: https://github.com/krausest/js-framework-benchmark/tree/master/frameworks/keyed/wasm-bindgen/src
    - Benchmark results: https://krausest.github.io/js-framework-benchmark/2022/table_chrome_100.0.4896.127.html
- Avoid a VDOM (presumably increases perf? If it doesn't then VDOM all day. Draw inspiration from svelte compiler.)
    - When doing diffing, having some kind of hash to determine if children need to be updated makes intuitive sense. You could use fastmurmur3 for this, since you shouldn't need a cryptographic hashing fn for this.
- html! macro with bare words (Percy has this - https://github.com/chinedufn/percy )
- wasm-bindgen code uses webpack by default. Let's instead use no bundler or snowpack (same as vite)?
- Fundamentally, we're going to need some kind of caching of macros/intermediate artifacts if Rust for frontend ever hopes to be practical. Really Rust for backend could massively use it as well. Segregating by workspace is absurdly heavy handed.



