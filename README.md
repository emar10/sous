# Sous
**Work with culinary recipes in Rust**

> **NOTE:** Sous is still in early development, and the API should be
> considered unstable at this time.

Sous provides structs for representing culinary recipes in Rust, along with an
API for loading and converting them between formats. This package also includes
a reference command-line utility demonstrating the library's capabilities.

# Installing

Sous is available on [crates.io](https://crates.io/crates/sous):

```
$ cargo add sous
```

# Command Line Usage

The Sous CLI can ingest recipes in YAML format, and output them as Markdown:

```
$ sous test.yml
```

If a directory is passed to Sous as input, it will operate in Cookbook mode,
converting any YAML files within. If the `--output` directory is not set, Sous
will output to a directory called `render` in the current working directory.

