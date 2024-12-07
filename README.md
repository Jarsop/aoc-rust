# Advent of Code (Rust)

This repository contains my solutions to the [Advent of Code](https://adventofcode.com/) challenges.

It's organized as a Cargo workspace, with each year's challenges in its own module.

## Automatic input fetching

This mechanism is shamelessly and barely copied from [jontmy aoc-rust repository](https://github.com/jontmy/aoc-rust).

The `input` files are directly fetched from the Advent of Code website, so you'll need to have a session cookie in order to download them.
The session cookie should be stored in `.env` file in the root of the repository, like so:

```sh
SESSION_TOKEN=your_session_cookie
```

You can find your session cookie by inspecting the network requests in your browser's developer tools.

## Run the solvers

By default the CLI will run the daily solvers for the current year.
You can specify a different year and day by passing the `--year` and `--day` flags.

```sh
cargo run -- --year 2024 --day 1
```
