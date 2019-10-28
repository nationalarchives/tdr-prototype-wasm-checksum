# Checksum calculator

This package contains a single function, generate_checksum, which takes a file and returns a promise which resolves to the sha256 checksum of this file.

This has been tested using an Intel Pentium N4200 1.1Ghz and this is faster than the Web Crypto digest by around 1 second per 60 Mb of file.

This has also been compared against the javascript function in the [MVC](https://github.com/nationalarchives/tdr-prototype-mvc/blob/master/js-src/upload/checksum.ts) app and this function is around 1 second faster per 80Mb file.

## How to build

Because gcc, which Rust relies on, is at such a low version on Centos 7 with no
easy way to upgrade it, it is difficult to build this project on Centos. The
alternatives are to use the Dockerfile in this repo to run a Docker container,
or install another Linux distro.

### Build locally

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

### Build with Docker

Build the Docker image:

```
sudo docker build . --tag checksumcalculator
```

Start a container:

```
sudo docker run -a stdin -a stdout -v $PWD:/usr/src/checksumcalc  -t -i checksumcalculator:latest /bin/bash
```

This starts an interactive bash shell on the container. The `-v` command mounts
this project folder to the `/usr/src/checksumcalc` directory in the container,
so that you can edit the code and build the npm package without having to
rebuild the container.

From the bash shell, you can run `wasm-pack build` to build the package.

## How to test locally

Once you've run `wasm-pack build` (whether locally or in the Docker container),
your code will be packaged in the `pkg` directory.

To use this package for local development in the [MVC site][tdr-mvc], update
package.json in the MVC code and change the checksum-calculator line to:

```
"@nationalarchives/checksum-calculator": "file:../tdr-prototype-wasm-checksum/pkg"
```

Update the file path if necessary, so that it points to the `pkg` directory in
this project.

Run `npm install` in the MVC codebase, then rebuild the frontend code.

[tdr-mvc]: https://github.com/nationalarchives/tdr-prototype-mvc

## How to publish to npm

```bash
wasm-pack login
wasm-pack publish
```
