# skyline-acmd-template

A template for writing Skyline ACMD plugins for modding Super Smash Bros. Ultimate using Rust and skyline-rs.

[Documentation for skyline-rs](https://ultimate-research.github.io/skyline-rs-template/doc/skyline/index.html)

## Setup

### Local

#### Prerequisites

* [Rust](https://www.rust-lang.org/install.html) - make sure rustup, cargo, and rustc (preferrably nightly) are installed.
* [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

Install [cargo skyline](https://github.com/jam1garner/cargo-skyline).
```bash
# inside a folder where you will dev all of your plugins going forward
cargo install cargo-skyline
cargo skyline new [your_plugin_name]
```

### VS Code with Docker

#### Prerequisites

* [Docker](https://www.docker.com/get-started)
* [Visual Studio Code](https://code.visualstudio.com/)
* [Visual Studio Code Remote Development Extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack)

Simply run `Remote Containers: Reopen in Container` in the Command Palette. 

## Creating and Building a Plugin

To compile your plugin use the following command in the root of the project (beside the `Cargo.toml` file):

    a. Installing via FTP connection

    The suggested workflow is installing and reading logs via FTP, using a sysmodule like [sys-ftpd-light](https://github.com/cathery/sys-ftpd-light/releases).

    Find your Switch's IP and run `cargo skyline set-ip [Your Switch's IP]`.

    When you'd like to test a plugin, you can try `cargo skyline run`, which installs the plugin automatically to the correct folder on your Switch, as well as Skyline and dependency plugins. The command will then hang while waiting for logs from the Switch upon booting Smash. You should be able to see any logs you printed in Rust code using `println!`!

    `cargo skyline run` is shorthand for `cargo skyline install`, which just installs the plugin without waiting for logs + `cargo skyline listen`, which only waits for logs from your system.

    To view installed plugins, you can run `cargo skyline list`.

    b. Installing manually
    ```sh
    cargo skyline build
    ```
    Your resulting plugin will be the `.nro` found in the folder
    ```
    [plugin name]/target/aarch64-skyline-switch
    ```
    To install (you must already have skyline installed on your switch), put the plugin on your SD at:
    ```
    sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins
    ```

`cargo skyline` can also automate some of this process via FTP. If you have an FTP client on your Switch, you can run:
```sh
cargo skyline set-ip [Switch IP]
# install to the correct plugin folder on the Switch and listen for logs
cargo skyline run 
```

`cargo skyline` can also automate some of this process via FTP. If you have an FTP client on your Switch, you can run:
```sh
cargo skyline set-ip [Switch IP]
# install to the correct plugin folder on the Switch and listen for logs
cargo skyline run 
```

## Troubleshooting

**"Cannot be used on stable"**

First, make sure you have a nightly installed:
```
rustup install nightly
```
Second, make sure it is your default channel:
```
rustup default nightly
```
---
```
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/bin/cargo-nro.rs:280:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Make sure you are *inside* the root of the plugin you created before running `cargo skyline build`

Have a problem/solution that is missing here? Create an issue or a PR!

## Updating

For updating your dependencies such as skyline-rs:

```
cargo update
```

For updating cargo skyline:

```
cargo skyline self-update
```

For updating your version of `rust-std-skyline-squashed`:

```
# From inside your plugins folder

cargo skyline update-std
```
