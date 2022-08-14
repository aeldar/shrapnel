# shrapnel [![Build Status][build-badge]][build-url] [![Linux Release Status][release-linux-badge]][release-url] [![Windows Release Status][release-windows-badge]][release-url]

[build-url]: https://github.com/aeldar/shrapnel/actions
[release-url]: https://github.com/aeldar/shrapnel/releases
[build-badge]: https://github.com/aeldar/shrapnel/workflows/build/badge.svg
[release-linux-badge]: https://github.com/aeldar/shrapnel/workflows/release%20(linux)/badge.svg
[release-windows-badge]: https://github.com/aeldar/shrapnel/workflows/release%20(windows)/badge.svg

A dumb simple runner of multipple commands at once. Works on Windows and Linux.

## Usage

Add a dumb simple `shrapnel.yml` configuration to the directory you are going to run the commands in:

```yaml
fragments:
  - name: Description of the run process
    dir: /path/to/dir/to/run/command/in # Optional
    cmd: your-marvelous-command
  - name: Description of another run process
    dir: ../../relative/path/to/dir/to/run/another/command/in
    cmd: ./your-marvelous-command-like-npm-start
```

... and run it:

```bash
shrapnel
```

## Installation

Download the binary from the Releases section of the [Github repository][releases] and place it somewhere in your PATH.

## Special notes

The `shrapnel` runner is just an excercise in using rust for simple cli applications. It is not supposed to be a robust tool. Please use it for your own purposes.

## TODO

- The `shrapnel` starts the processes in parallel and joins all the IO streams without any differentiation of them in the console. There should be a way to distinguish the streams and print them in the console with colors (or/and prefixed).
- The `shrapnel` starts the processes, but cannot stop them. There should be a way to stop the processes on killing the `shrapnel` itself.
- The `shrapnel` should be able to run the commands in a different directory than the current one.
