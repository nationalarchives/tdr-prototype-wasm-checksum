# Checksum calculator

This package contains a single function, generate_checksum, which takes a file and returns a promise which resolves to the sha256 checksum of this file.

This has been tested using an Intel Pentium N4200 1.1Ghz and this is faster than the Web Crypto digest by around 1 second per 60 Mb of file.

This has also been compared against the javascript function in the [MVC](https://github.com/nationalarchives/tdr-prototype-mvc/blob/master/js-src/upload/checksum.ts) app and this function is around 1 second faster per 80Mb file.

## How to build

Install [rust](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Build

```bash
wasm-pack build
```

## How to publish to npm

```bash
wasm-pack login
wasm-pack publish
```

## A note on Centos 7

Because gcc, which rust relies on, is at such a low version on Centos with no real way to upgrade it, it is difficult to build this project on Centos. The alternatives are to use an Ubuntu docker container with a local volume to build the project or install another distro.
