# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/claude-code) when working with code in this repository.

## Project Overview

OOM Arithmetic Trainer is a web-based daily practice tool for order-of-magnitude arithmetic estimation. Users practice quickly estimating products/quotients of large numbers (e.g., "8.3 million × 47 thousand" → "~400 billion").

## Tech Stack

- **Language:** Rust compiled to WebAssembly
- **Framework:** Leptos (signals-based reactivity)
- **Build Tool:** Trunk
- **RNG:** `rand` + `rand_chacha` for deterministic daily seeding

## Common Commands

```bash
# Install dependencies (first time setup)
rustup target add wasm32-unknown-unknown
cargo install trunk

# Development server with hot reload
trunk serve

# Production build
trunk build --release
```

## Architecture

- **Fully static:** No backend, no database, all computation in browser
- **Daily challenges:** Date string (e.g., "2026-01-11") seeds ChaCha8Rng for deterministic problem generation
- **Output:** Static files (HTML + JS + WASM) in `dist/`
- **Persistence:** localStorage for session stats (optional)

## Project Structure

```
src/
├── main.rs          # App entry point
├── app.rs           # Main component
├── challenge.rs     # Problem generation, seeding
├── scoring.rs       # Answer evaluation
├── parser.rs        # Parse user input formats
└── components/
    ├── problem.rs   # Problem display
    ├── input.rs     # Answer input
    └── results.rs   # Feedback display
```

## Key Design Decisions

1. **Scoring:** Based on closeness to correct order of magnitude (exact OOM = full points, off by 1 = partial, off by 2+ = none)
2. **Number constraints:** Both operands between 10^3 and 10^9, avoid "too round" numbers
3. **Input flexibility:** Accept formats like "400 billion", "4e11", "4 × 10^11"
4. **No accounts:** Zero data collection, no backend dependencies

## Deployment

Deploy to Vercel or Netlify (free tier):
- Build command: `trunk build --release`
- Output directory: `dist/`
