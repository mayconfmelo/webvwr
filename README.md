# Webvwr

**This program is still under early stages of development.**

Transform your favorite sites into a native-looking web viewer.

## Usage

If you run just `webvwr`, without any arguments, by default it will just open my website. If you want to open any other site as a program just run the program as showed below:

```
webvwr --title=<STRING> --icon=<PATH> [URL]
```

If no `--title` and/or `--icon` flags are provided, the default site title and favicon will be used.

To install the site as a program in your system, just run:

```
webvwr install [URL] [TITLE] --icon=<PATH>
```

This time the title is mandatory, but if `--icon` is not provided, the site favicon will still be used.

## JavaScript Injection

If you create a file named *init.js* in the same directory as the program, like this:

```
../
 ├── webvwr
 └── init.js
```

All JavaScript code inside it will be injected into the site loaded by Webvwr.

## Features and TODOs

- [X] Open sites in a simple web viewer window
- [X] JavaScript injection
- [X] Command line interface
- [X] Use site title when `--title` is not provided
- [ ] Use site favicon when `--icon` is not provided
- [ ] Install works on Windows
- [ ] Install works on Linux
- [ ] Optimize Rust code

## Disclaimer

This program was created as a way to help me learn the Rust language, so while it may work to some extent, it may contain some really unoptimized code.