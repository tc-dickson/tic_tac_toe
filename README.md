# `tic-tac-toe`
A simple cli tic-tac-toe implementation against a computer opponent.

## Usage
```
tic-tac-toe \[player_piece_type\] \[first_or_second\]
```

### player_piece_type
| Argument     | Result             |
| ------------ | ------------------ |
| "x" (or "X") | Play with X pieces |
| "o" (or "O") | Play with O pieces |

### first_or_second
| Argument | Result      |
| -------- | ----------- |
| "1"      | Play first  |
| "2"      | Play second |

## Things of Note
There are a few things to highlight:

  - Ai opponent implemented using depth-limited minmax with [alpha-beta pruning](https://en.wikipedia.org/wiki/Alpha-beta_pruning).
  - Simple cli arguments to select which pieces to play with (x or o) and whether to go first or second.
  - Error handling allowing the user to try again if the game detected an invalid move input.

## Why?
I am in the midst of learning Rust. This project gave me an excuse to practice what I have been learning. Since the goal of this project was learning Rust, I specifically avoided using external packages (though they could greatly improve this project). Among the things I've learned:

  - Alpha-Beta tree pruning algorithm
  - Simple user input parsing
  - Error handling
  - Testing principles
  - Documentation principles
  - Code structuring with modules
  - Cargo build tool basics

