# wordic
[![test](https://github.com/yujikawa/wordic/actions/workflows/test.yml/badge.svg)](https://github.com/yujikawa/wordic/actions/workflows/test.yml)
[![wordic at crates.io](https://img.shields.io/crates/v/wordic.svg)](https://crates.io/crates/wordic)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A command-line tool to manage your personal word dictionary on your local PC.
Store words with values and descriptions, then look them up instantly from the terminal.

## Installation

```bash
cargo install wordic
```

## Quick Start

```bash
# 1. Initialize (creates ~/.wordic/wordic.json)
wordic init

# 2. Add a word
wordic add rust "A systems programming language" "memory safe, fast"

# 3. Look it up
wordic get rust

# 4. Show all words
wordic show
```

## Commands

| Command | Description |
|---|---|
| `wordic init` | Initialize the dictionary |
| `wordic add <key> <value> [description]` | Add or update a word |
| `wordic get <key>` | Get the value for a key |
| `wordic get <key> --full` | Get full entry (value + description + timestamp) |
| `wordic rm <key>` | Remove a word |
| `wordic show` | Show all words in a table |
| `wordic search <keyword>` | Search by keyword across key, value, and description |
| `wordic export <path>` | Export dictionary to a JSON file |
| `wordic import <path>` | Import dictionary from a JSON file (replaces current data) |

## Examples

```bash
# Add words
wordic add python "A scripting language" "easy to learn"
wordic add docker "Container platform" "devops"

# Show all
wordic show
# No    | Key                  | Value                     | Description               | Created at
# ------+----------------------+---------------------------+---------------------------+------------------------------
# 0     | python               | A scripting language      | easy to learn             | 2026-03-21 ...
# 1     | docker               | Container platform        | devops                    | 2026-03-21 ...

# Get just the value (useful in scripts)
wordic get python
# A scripting language

# Get full details
wordic get python --full
# Key        : python
# Value      : A scripting language
# Description: easy to learn
# Created at : 2026-03-21 ...

# Search across all fields
wordic search language
# 2 result(s) found.

# Backup and restore
wordic export ~/backup.json
wordic import ~/backup.json
```

## Data Storage

Dictionary is stored at `~/.wordic/wordic.json`.
