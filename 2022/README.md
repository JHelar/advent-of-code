# AoC 2022
Advent of Code 2022

# Setup

## Installation
Install dependencies

```bash
yarn
```

## Using Typescript
No extra steps required.

## Using Kotlin
Install the Kotlin compiler
```bash
brew install kotlin
```

Ensure that you have Java runtime installed
```bash
brew install java
```

# Create a new day
Create a new day directory, fetch day input and generate boilder plate.
It will pick the next following day taken from the current status of the `days` directory. Meaning if you have `day-1` in your `days` directory it will create the `day-2` directory.

* Set `LANGUAGE` to a language of your choice: `kt`- Kotlin, `ts`- Typescript

```bash
yarn day:new LANGUAGE
```

Example:
Create a new day with Kotlin language
```bash
yarn day:new kt
```

# Run the code
To run the code for a given day and part

* Set `LANGUAGE` to the language that you want to run: `kt`- Kotlin, `ts`- Typescript
* Set `DAY` to target the day to run
* **Optional** Set `PART` to which part to run, defaults to part `1`

```bash
yarn day:run LANGUAGE DAY PART?
```

Example:
Run day 1 part 2 with Kotlin language
```bash
yarn day:run kt 1 2
```