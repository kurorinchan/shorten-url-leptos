Shortens Amazon URLs by removing unnecessary parts of the URL.

https://kurorinchan.github.io/shorten-url-leptos/

# Build

```
RUSTFLAGS=--cfg=web_sys_unstable_apis trunk serve --open
```

# VScode
When using vscode, put the following in workspace's `settings.json`

```json
{
    "rust-analyzer.rustfmt.overrideCommand": [
        "leptosfmt",
        "--stdin",
        "--rustfmt"
    ],
    "rust-analyzer.cargo.extraEnv": {
        "RUSTFLAGS": "--cfg=web_sys_unstable_apis"
    },
    // for tests to build.
    "rust-analyzer.runnables.extraEnv": {
        "RUSTFLAGS": "--cfg=web_sys_unstable_apis"
    }
}
```