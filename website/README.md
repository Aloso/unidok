# Unidok website

aloso.github.io/unidok

## Run locally

Make sure you have the following programs installed:

* [Rustup](https://rustup.rs/)
* [Wasm-pack](https://rustwasm.github.io/wasm-pack/)
* [Node.js](https://nodejs.org/)
* [Npm](https://www.npmjs.com/)

First, we have to build the WebAssembly file:

```sh
cd ../unidok-js
rustup default beta # nightly works as well
wasm-pack build
```

Now we can build the website:

```sh
cd ../website
npm install
npm run start
```

This should open the website in your web browser.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE_APACHE](../LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE_MIT](../LICENSE_MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
