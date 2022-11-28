# AoC 2022
Advent of Code 2022

# Setup

## Installation
Install `Node.js`

Install dependencies
```bash
yarn
```

## Using Typescript
No extra steps required.

## Using Kotlin
Install the `Kotlin` compiler
```bash
brew install kotlin
```

Ensure that you have Java runtime installed
```bash
brew install java
```

## Using Rust
Install `Rust` and `Cargo`
```bash
curl https://sh.rustup.rs -sSf | sh
```

# Create a new day
Create a new day directory, fetch day input and generate boiler plate to quicker get started with the problem solving.
It will pick the next following day taken from the current status of the `days` directory. Meaning if you have `day-1` in your `days` directory it will create the `day-2` directory.

* Set `LANGUAGE` to a language of your choice: `kt`- Kotlin, `ts`- Typescript, `rs`- Rust
* **Optional** Set `DAY` to target the day to create

```bash
yarn day:new --lang LANGUAGE [--day DAY]
```

Example:
Create a new day with Kotlin language
```bash
yarn day:new --lang kt
```

The script will the first time prompt you for your session cookie from [Advent of code](https://adventofcode.com/), you find it by loggin in, inspecting the page -> applications tab -> cookies tab, look for the value in `session`.

# Run the code
To run the code for a given day and part

* Set `LANGUAGE` to the language that you want to run: `kt`- Kotlin, `ts`- Typescript, `rs`- Rust
* Set `DAY` to target the day to run
* **Optional** Set `PART` to which part to run, defaults to part `1`

```bash
yarn day:run --lang LANGUAGE --day DAY [--part PART]
```

Example:
Run day 1 part 2 with Kotlin language
```bash
yarn day:run --lang kt --day 1 --part 2
```