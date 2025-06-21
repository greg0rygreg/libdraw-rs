# libdraw-rs
remake of [libdraw](https://github.com/greg0rygreg/libdraw) in rust

## making your own app
1. clone this repo

2. rename the `lib` folder to `libdraw-rs` and the example folder to anything you want

3. in the `Cargo.toml` file of the parent directory, replace:

```toml
members = [ "lib", "example" ]
```

with:

```toml
members = [ "libdraw-rs", "your_app_name" ]
```

and replace `your_app_name` with the new name of the just-renamed `example` folder

5. in `your_app_name/Cargo.toml`, replace:

```toml
libdraw-rs = { path = "../lib" }
```

with:

```toml
libdraw-rs = { path = "../libdraw-rs" }
```

6. in the parent directory, run your app with:

```sh
cargo run -p your_app_name --release
```