<p align="center">
  <img width="75%" src="./res/demo.gif" />
</p>

# Lanturn

Lanturn is a website connectivity monitor written in Rust 🦀

Lanturn offers a simple dashboard that lets you quickly check if your internet — or one of your favorite sites — is up or down.

## Building

To manually build the project, you must first [install Rust](https://www.rust-lang.org/tools/install).

Once you have Rust installed, run the following commands:

```bash
git clone https://github.com/massivebird/lanturn
cd lanturn
cargo run # runs unoptimized build
```

> `cargo run`'s build phase will tell you if you need to install other dependencies such as `pkg-config` and `libssl-dev`.

### Nix flake

If you're using Nix, you can add the following to your flake's `inputs`:

```nix
inputs = {
  # ...

  lanturn = {
    url = "github:massivebird/lanturn";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  # ...
}
```

Then, add the following to your `environment.systemPackages`:

```nix
environment.systemPackages = [
  # ...
  inputs.lanturn.packages.${pkgs.system}.default
  # ...
]
```
