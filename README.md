# `tic-tac-toe`
A simple CLI tic-tac-toe implementation against a computer opponent.

![Gameplay GIF](./media/game.gif)

## Overview
A classic game implemented in Rust to reinforce foundational skills and gain hands-on experience.

## Features / Capabilities
  - AI opponent implemented using depth-limited minimax algorithm with [alpha-beta pruning](https://en.wikipedia.org/wiki/Alpha-beta_pruning).
  - Simple CLI arguments to select which pieces to play with (x or o) and whether to go first or second.
  - Error handling allowing the user to try again if the game detected an invalid move input.

## Key Technologies

| Programming Language  | Platform                 |
| --------------------- | ------------------------ |
| - Rust                | - Command Line Interface |

## What I Learned
I'm currently learning Rust, and this project was a hands-on opportunity to apply what I've been studying. As the primary goal was to deepen my understanding of core Rust concepts, I intentionally avoided using external libraries—even when they could have streamlined the development process.

Through this project, I gained experience with:

  - Implementing the alpha-beta pruning algorithm for AI decision-making
  - Parsing user input and handling invalid entries gracefully
  - Applying Rust’s error handling patterns
  - Writing and organizing unit tests
  - Following good documentation practices
  - Structuring code using modules for clarity and reusability
  - Working with Cargo for building and testing

## Usage
```
tic-tac-toe [player_piece_type] [first_or_second]
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

