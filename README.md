# slomore

slower `more` = `slomore`

A custom pager that outputs lines with a delay, allowing control over pacing.

![A GIF showing slomore usage](./gifs/slomore.gif)

## Introduction

Slomore is a pager (like `more`, or `less`) that slows down screen output.

This is useful when making tutorials where command line tools spew lots of output that scrolls by too fast. If you're capturing the output for use in a video (or gif) without `slomore`, your video recorder may 'miss' frames making the output appear 'jumpy'.

## Installation

Slomore is written in Rust. Binary builds are available with tagged [releases](https://github.com/popey/slomore/releases).

## Install the snap

```bash
snap install slomore
```

## Build from source

Compile using `cargo`.

```bash
git clone https://github.com/popey/slomore
cd slomore
cargo build --release
```

Copy the binary to your `$PATH`.

```bash
cp target/release/slomore /usr/local/bin
```

## Usage

```
slomore 0.1.0
A custom pager that outputs lines with a delay, allowing control over pacing.

Usage: slomore [OPTIONS]

Options:
  -s, --seconds-per-line <SECONDS>  Set delay in seconds between lines. Must be greater than 0.
  -l, --lines-per-second <LINES>    Set the number of lines to display per second. Must be greater than 0.
  -h, --help                        Print help
  -V, --version                     Print version
```

### Pipe command output to `slomore`

slomore outputs 10 lines per second by default.

```bash
find /bin | slomore
```

![find /bin piped through slomore](./gifs/findpipeslomore.gif)

### Change output speed

slomore supports speeding up and slowing down the output by setting 'lines per second' or 'seconds per line'. 

**Faster**

```bash
export SLOMORE_LINES_PER_SECOND=20
find /bin | slomore
```

![Speed up output](./gifs/findpipeslomore20.gif)

**Slower**

```bash
export SLOMORE_SECONDS_PER_LINE=2
find /bin | slomore
```

![Slow down output](./gifs/findpipeslomore2.gif)

## Caveats

I am an open-source enthusiast and self-taught coder creating projects driven by curiosity and a love for problem-solving. The code may have bugs or sharp edges. Kindly let me know if you find one, via an [issue](https://github.com/popey/slomore/issues). Thanks.
