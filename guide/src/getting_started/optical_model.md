# Optical Model

For our first optical model, let's start by creating a new binary crate with [cargo]

```bash
cargo new --bin optical-model 
```
and move to the new crate:
```bash
cd optical-model
```
The `Cargo.toml` file is called the *manifest* and it is where the crate dependencies are listed.
Let's already add some dependencies, again using [cargo]:
```bash
cargo add gmt_dos-clients_crseo
cargo add anyhow
cargo add tokio --features rt-multi-thread
```

The `main` script is in the `src` folder and it is compiled by invoking [cargo]:
```bash
cargo build
```
Once the program has compiled succesfully, it can be run with 
```bash
cargo run
```
that prints
```
Hello, world!
```

Let's replace the contents of the `main` script with the following:

 ```rust,no_run,noplayground
{{#include ../../../optical-model/src/main.rs:3}}
{{#include ../../../optical-model/src/main.rs:6:8}}
{{#include ../../../optical-model/src/main.rs:18:19}}
 ```

 The default [OpticalModel] is just the GMT segmented optical model with an on-axis source.
 The GMT model uses M1 [bending modes] and a [Karhunen-Loeve] modal basis to set the figures of M1 and M2 segments, respectively.
 The environment variable `GMT_MODES_PATH` must be set to the path to the folder that hold both `.ceo` files, e.g.
 ```bash
 export GMT_MODES_PATH=<path-to-ceo-files>
 ```
 With the environment variable set, we can run again the new main script:
 ```bash
cargo run
```
At that stage, the model is not doing anything beside building the GMT optical model.

[cargo]: https://doc.rust-lang.org/cargo/
[OpticalModel]: https://docs.rs/gmt_dos-clients_crseo/3.4.1/gmt_dos_clients_crseo/struct.OpticalModel.html
[bending modes]: https://s3.us-west-2.amazonaws.com/gmto.modeling/bending+modes.ceo
[Karhunen-Loeve]: https://s3.us-west-2.amazonaws.com/gmto.modeling/Karhunen-Loeve.ceo