# Visual Studio Setup Configuration

[Microsoft Visual Studio](https://visualstudio.microsoft.com) 2017 and newer uses a new setup engine that allows multiple instances to be installed quickly and in different configuration. To enumerate these instances and find one that fulfills your requirements, the [Setup Configuration API](https://devblogs.microsoft.com/setup/documentation-available-for-the-setup-configuration-api) provides a set of interface. This crate provides a safe and idiomatic wrapper for [Rust](https://www.rust-lang.org).

## Example

First add this crate to your Cargo.toml file:

```toml
[dependencies]
vssetup = "0.1.0"
```

You'll then need to use the crate in your project:

```rust
extern crate vssetup;
use vssetup::SetupConfiguration;
```

Make sure you initialize COM in your application. You can then enumerate and display instances:

```rust
use windows::{initialize_sta, Result};

fn main() -> Result<()> {
    initialize_sta()?;

    let config = SetupConfiguration::new();
    if let Some(e) = config.instances() {
        for instance in e {
            println!("{}", instance.installation_path()?);
        }
    }

    Ok(())
}
```

See [src/bin/vswhere.rs](https://github.com/heaths/vssetup-rs/blob/master/src/bin/vswhere.rs) for more examples.

## FAQ

* **On what platforms does this work?**

  This crate will only compile and work on Windows.

* **Is this project supported by Microsoft?**

  Though I am the developer who wrote the Setup Configuration API while working for Microsoft, this crate is unsupported by Microsoft.
