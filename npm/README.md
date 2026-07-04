# nexo-note (npm wrapper)

This npm package is a thin wrapper around the Rust CLI `nexo`. It downloads the correct pre-built binary for your platform from GitHub Releases during installation.

## Install

```bash
npm install -g nexo-note
```

Or use without installing:

```bash
npx nexo-note --help
```

## Usage

```bash
nexo init --git
nexo create "My first note" -c issues -t "hello"
nn ls
nexo stats
```

## Supported platforms

- Windows x86_64
- Linux x86_64
- macOS x86_64
- macOS ARM64 (Apple Silicon)

## Note

The actual CLI is written in Rust. This npm package only downloads and invokes the binary.
