# ♟️ Andrej

A chess engine written in Rust. Named after my brother who always beats me at chess.

Following the [Chess Engine in C](https://www.youtube.com/playlist?list=PLZ1QII7yudbc-Ky058TEaOstZHVbT-2hg) tutorial series.

## Quick Start

```bash
cargo run      # Run the program
cargo test     # Run tests
```

## Implementation

**Board representation:** 120-square mailbox (10x12 array)
- 8x8 chess board embedded in a larger array with sentinel squares
- Simplifies move generation by avoiding boundary checks
- Valid squares: 21 (a1) to 98 (h8)

**Current features:**
- Core types: `Piece`, `Square`, `Position`, `Board`
- Type-safe coordinates using `File` and `Rank` enums
- Position indexing formula: `(rank + 2) * 10 + (file + 1)`
- Terminal rendering with grayscale board
- Filled Unicode chess pieces (♟♞♝♜♛♚) for both sides

## Roadmap

- [x] Board representation
- [ ] Move generation
- [ ] Position evaluation
- [ ] Search algorithm
- [ ] UCI protocol
