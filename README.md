<img src="https://github.com/malfisya/salam/blob/main/data/salam.png" alt="drawing" width="200"/>

# Salam

Salam (_sa•lam_ , "greetings" in Bahasa Indonesia) is a simple welcome app for [Solus](https://getsol.us/) Operating System. It is written in [Rust](https://www.rust-lang.org/) and [Slint](https://slint.dev/).

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
- [open-rs](https://github.com/Byron/open-rs)

## Attributions
- `salam.png` is provided by <a href="https://www.flaticon.com/free-icons/hello" title="hello icons">Hello icons created by Vitaly Gorbachev - Flaticon</a>.
- `play.png` is provided by <a href="https://www.flaticon.com/free-icons/play-button" title="play button icons">Play button icons created by Pixel perfect - Flaticon</a>
- `install.png` is provided by <a href="https://www.flaticon.com/free-icons/download" title="download icons">Download icons created by Pixel perfect - Flaticon</a>
- `github.png` is provided by <a href="https://www.flaticon.com/free-icons/github" title="github icons">Github icons created by Pixel perfect - Flaticon</a>
- `web.png` is provided by <a href="https://www.flaticon.com/free-icons/www" title="www icons">Www icons created by Pixel perfect - Flaticon</a>
- `docs.png` is provided by <a href="https://www.flaticon.com/free-icons/user-guide" title="user guide icons">User guide icons created by Prosymbols Premium - Flaticon</a>

## License
This project is licensed under Mozilla Public License Version 2.0.
