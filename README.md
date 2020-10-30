# Visual Studio Setup Configuration

[Microsoft Visual Studio](https://visualstudio.microsoft.com) 2017 and newer uses a new setup engine that allows multiple instances to be installed quickly and in different configuration. To enumerate these instances and find one that fulfills your requirements, the [Setup Configuration API](https://devblogs.microsoft.com/setup/documentation-available-for-the-setup-configuration-api) provides a set of interface. This crate provides a safe and idiomatic wrapper for [Rust](https://www.rust-lang.org).

## Example

Coming soon...

## FAQ

* **On what platforms does this work?**

  While the package should compile on all platforms, it'll only work on Windows. Public APIs on other platforms should, for example, return an empty iterator.

* **Is this project supported by Microsoft?**

  While I am the developer who wrote the Setup Configuration API while working for Microsoft, this crate is unsupported by Microsoft.
