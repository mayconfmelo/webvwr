# Webvwr

**This program is still under early stages of development and is not  functional yet.**

Transform your favorite sites into a native-looking web viewer.

## Usage

You can just run it the program in standalone mode, as showed below:

```
webvwr [URL] --title=<STRING> --icon=<PATH>
```

Or install in your system to run it through a shortcut, like a program:

```
webvwr create [URL] [TITLE] --icon=<PATH>
```

For more informations, run `webvwr --help`.

## Disclaimer

This program was created as a way to help me learn the Rust language, so while it may work to some extent, it may contain some really unoptimized code.

## Features

- [X] Open sites in a web viewer
- [X] JavaScript injection through `./init.js` file
- [ ] Command line interface
- [ ] Use site title when `--title` is not provided
- [ ] Use site icon when `--icon` is not provided
- [ ] Optimize Rust code