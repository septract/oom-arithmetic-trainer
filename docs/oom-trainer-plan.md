# OOM Arithmetic Trainer

## Overview

A web-based daily practice tool for order-of-magnitude arithmetic—the skill of quickly estimating products/quotients of large numbers without a calculator.

**Example problem:** "8.3 million × 47 thousand" → user answers "~400 billion"

This is the pure arithmetic layer of Fermi estimation, not the decomposition or fact-retrieval parts.

## Goals

- Daily practice habit (like Wordle)
- Fast, frictionless UX
- No accounts, no backend, no data collection
- Scoring based on closeness to correct order of magnitude

## Technical Decisions

### Stack: Rust + WebAssembly

- **Framework:** Leptos (signals-based reactivity, active development, good DX)
- **Build tool:** Trunk
- **RNG:** `rand` + `rand_chacha` for deterministic seeding

Rationale: Learning project, strong typing preference, "if it compiles it works" guarantees. Acknowledged overkill for the problem size.

### Architecture: Fully Static

- No backend server
- No database
- All computation happens in the browser
- Output is static files (HTML + JS + WASM)
- Session stats can persist via localStorage

### Deployment

- Host on Vercel or Netlify (free tier)
- Connect GitHub repo
- Configure build command: `trunk build --release`
- Configure output directory: `dist/`
- Push to deploy

Both platforms have Rust in their build images. Alternative: build locally or via GitHub Actions, deploy only artifacts.

### Security Model

Minimal attack surface:
- No user data to steal
- No secrets to expose
- No server to compromise
- Zero runtime dependencies if desired
- Worst case: someone cheats at their own arithmetic practice

## Daily Challenge System

Use date as seed for deterministic RNG:

```rust
let seed = hash(current_date_string()); // e.g., "2026-01-11"
let mut rng = ChaCha8Rng::seed_from_u64(seed);
let challenge = generate_challenge(&mut rng);
```

Everyone who opens the app on the same day gets the same challenge. No list to maintain, no timer, no backend coordination.

For multiple problems per session: seed once, draw N problems from that seeded RNG. Everyone gets the same sequence.

### Generation Constraints

Procedurally generate problems with constraints:
- Both numbers between 10^3 and 10^9 (adjustable)
- Avoid "too round" numbers (e.g., exactly 1,000,000)
- Mix of multiplication and division
- Mantissas that aren't trivial (not just 1.0 × 10^n)

Optional future enhancement: curated "featured" problems for interesting edge cases (e.g., mantissas that nearly cancel to 10).

## Core Gameplay

### Problem Display

Show two numbers in human-readable form:
- "8.3 million × 47 thousand = ?"
- Or: "8.3 × 10^6 × 4.7 × 10^4 = ?" (user preference toggle?)

### Input

User enters their estimate. Accept flexible formats:
- "400 billion" / "400B"
- "4 × 10^11"
- "4e11"
- Just "11" (interpreted as 10^11)?

Need to decide on input UX—freeform text vs. structured (separate mantissa/exponent fields).

### Scoring

Score based on how close the user's answer is to the actual value:
- Exact order of magnitude: full points
- Off by 1 OOM: partial credit
- Off by 2+: no points

Could also score mantissa accuracy as a bonus.

### Feedback

Show:
- User's answer
- Actual answer
- How close they were (e.g., "Off by 0.3 orders of magnitude")
- Running stats for the session

## UI/UX

Keep it minimal:
- Single page
- Problem prominently displayed
- Input field
- Submit button
- Results/stats below

No signup, no onboarding, no tutorials. User lands and immediately sees today's problem.

### Optional Features (v2)

- Unlimited practice mode (beyond daily challenge)
- Difficulty settings (number ranges, operation types)
- Historical stats (localStorage)
- Share results (like Wordle's grid)
- Timer/speed mode

## Project Structure (Leptos)

```
oom-trainer/
├── Cargo.toml
├── Trunk.toml
├── index.html
├── src/
│   ├── main.rs          # App entry point
│   ├── app.rs           # Main component
│   ├── challenge.rs     # Problem generation, seeding
│   ├── scoring.rs       # Answer evaluation
│   ├── parser.rs        # Parse user input formats
│   └── components/
│       ├── problem.rs   # Problem display
│       ├── input.rs     # Answer input
│       └── results.rs   # Feedback display
├── style/
│   └── main.css
└── dist/                # Build output (gitignored)
```

## Development Workflow

1. Install Rust: `rustup`
2. Add WASM target: `rustup target add wasm32-unknown-unknown`
3. Install Trunk: `cargo install trunk`
4. Dev server: `trunk serve` (hot reload)
5. Build: `trunk build --release`
6. Deploy: push to GitHub, Vercel/Netlify auto-builds

## Open Questions

1. **Input format:** Freeform text parsing vs. structured fields?
2. **Number display:** Words ("million") vs. scientific notation vs. both?
3. **Problem count:** One problem per day, or a set of N?
4. **Division:** Include division problems, or multiplication only?
5. **Negative exponents:** Include small numbers (0.00047), or stick to large?

## Success Criteria

- User can complete a daily challenge in <2 minutes
- Works on mobile browsers
- Deployable with zero ongoing cost
- Codebase simple enough to maintain casually
