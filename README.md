# wordic
[![test](https://github.com/yujikawa/wordic/actions/workflows/test.yml/badge.svg)](https://github.com/yujikawa/wordic/actions/workflows/test.yml)
[![wordic at crates.io](https://img.shields.io/crates/v/wordic.svg)](https://crates.io/crates/wordic)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

This is command line tool to manage word dictionary.

# How to install?
```
cargo install wordic
```

# How to use?
## First
```
wordic init
```

## How to add new item to dictionary?
```
wordic add {KEY} {VALUE}
```

## How to get item created from dictionary?
```
wordic get {KEY}
```

## How to remove item created from dictionary?
```
wordic rm {KEY}
```

## How to show all item created from dictionary?
```
wordic show
```