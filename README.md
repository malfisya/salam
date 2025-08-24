# Salam

Salam (_sa•lam_ , "greetings" in Bahasa Indonesia) is a simple welcome app for [Solus](https://getsol.us/) Operating System. It is written in [Rust](https://www.rust-lang.org/) with [Slint](https://slint.dev/).

## Building
To build this app, it requires these dependencies:
- Meson
- Rust (cargo)

Run these commands to build the app

```
meson setup build/ --buildtype=release
meson compile -C build/
```

The resulting binary will be available at `target/release/` directory.

## Credits
Salam take many inspirations from these wonderful projects:

- [parch-welcome](https://github.com/parchlinux/parch-welcome)
- [slint-rust-template](https://github.com/slint-ui/slint-rust-template/)

## Temporary Icon
`salam.png` is provided by <a href="https://www.flaticon.com/free-icons/hello" title="hello icons">Hello icons created by Vitaly Gorbachev - Flaticon</a>.

## License
This project is licensed under Mozilla Public License Version 2.0.
