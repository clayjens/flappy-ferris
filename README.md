# Flappy Ferris
<img src="./assets/ferris.png" width="25%" height="25%"/>

A [Flappy Bird](https://wikipedia.org/wiki/Flappy_Bird) clone written in Rust using the Bevy Engine.

---

- [Flappy Ferris](#flappy-ferris)
  - [Running The Binary](#running-the-binary)
  - [Running From Source](#running-from-source)

---

## Running The Binary

## Running From Source
Since Bevy is an intensive framework, it is recommended to run the App using release `-r` and enable dynamic linking `--features bevy/dynamic`. Release mode and dynamic linking enables fast compile times.  
Additionally, ensure that your rust framework is running Nightly. If rustup is smart, it should have detected [rust-toolchain.toml](./rust-toolchain.toml) and enabled the Nightly toolchain already!
If you don't want to run using the Nightly toolchain, follow the comments in [.cargo/config.toml](./.cargo/config.toml) to disable compiler Z-flags.  
Finally, if you truly want blazingly-fast speeds when runnning from Source, set up a LLD Linker. The [Bevy Book](https://bevyengine.org/learn/book/getting-started/setup/) setup chapter describes how to do this.
```sh
cargo run -r --features bevy/dynamic
```