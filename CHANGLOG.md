# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2] - 2023-02-08

### Documentation

- Update README

### Features

- Add support for list all available dictionaries

### Miscellaneous Tasks

- Add CHANGELOG

## [0.1.1] - 2023-02-07

### Bug Fixes

- Typo

### Features

- Add support for specifying dictionary directory again
- Implement sdcv cli interface for rmall
- Add support for interactive mode
- Determine whether to use fuzzy search by the leading character
- Add support for readline history
- Add a switch for non-interactive mode
- Update script to use new rmall options

## [0.1.0] - 2023-02-07

### Bug Fixes

- Correct the help message

### Documentation

- Use asciinema for better execution result display
- Update README.md and Cargo.toml

### Features

- Add option for exact search
- Support the dict dir's name is not the same as its contents'
- Support multiply offline dictionaries

### Performance

- Use a faster fuzzy-search implementation.

### Refactor

- Set 'lookup' the default subcommand

## [0.0.6] - 2023-01-26

### Bug Fixes

- Fix wrong version number

### Documentation

- Update README for multiple dictionaries
- Update README for new version

### Features

- Fuzzy search
- Discard fuzzy search results in which more than half letters are typos.
- Try binary search before fuzzy search.
- Show all possible words in fuzzy search.

### Styling

- Add help information for script tools

## [0.0.5] - 2023-01-24

### Bug Fixes

- Remove output('\n') when there's no exam-type
- Fix a problem with history db
- Change User Agent
- Add prompt when cannot find the word
- Remove the redundant phonetic symbols and mark which accent it belongs
- Fix the functio used by binary searching

### Documentation

- Update README.md
- Update README
- Update README
- Update README
- Update README

### Features

- Naive implementation
- Support restore and list history
- Add phonetic support
- Support `part of speech` search
- List words by exam type
- Add exam type count
- Support outputting the records to a table
- Support local dictionary
- Add binary search suppport
- More options to lookup

### Miscellaneous Tasks

- Add build.sh script
- Use makefile instead of bash script to build the project
- Add `make run` in Makefile
- Provide a tool to read from .idx file
- Add shell wrapper for mutiple dictionaries

### Refactor

- Optimize the structure of the code
- Split the code into multiple files
- Optimize the structure of the code and add word's exam-type support
- Make the code much more rust-like
- Better error handling
- Much better error handling
- More error message

### Styling

- Vs code rust formatting
- Typo in variable naming

### Testing

- Use previously queried records to test

<!-- generated by git-cliff -->