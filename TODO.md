# TODO

## Phase 1: Project Setup

- [ ] Initialize Cargo project with Leptos dependencies
- [ ] Configure Trunk.toml for WASM build
- [ ] Create index.html with basic structure
- [ ] Set up main.css with minimal styling
- [ ] Verify `trunk serve` works with hello world

## Phase 2: Core Logic

- [ ] Implement date-seeded RNG (ChaCha8Rng)
- [ ] Build problem generator with constraints (10^3 to 10^9 range)
- [ ] Add mantissa variation (avoid trivial 1.0 × 10^n)
- [ ] Create number formatter (display as "8.3 million", etc.)
- [ ] Write input parser (handle "400B", "4e11", "4 × 10^11" formats)
- [ ] Implement scoring algorithm (OOM distance calculation)

## Phase 3: Components

- [ ] Problem display component
- [ ] Answer input component
- [ ] Submit button with keyboard support (Enter)
- [ ] Results/feedback component
- [ ] Session stats display

## Phase 4: App Integration

- [ ] Wire up main app component with state management
- [ ] Connect problem generation to daily seed
- [ ] Handle answer submission flow
- [ ] Show feedback after submission
- [ ] Add "next problem" or session completion logic

## Phase 5: Polish

- [ ] Mobile-responsive layout
- [ ] localStorage for session stats persistence
- [ ] Accessible markup (ARIA labels, focus management)
- [ ] Loading state for WASM initialization

## Phase 6: Deployment

- [ ] Production build verification
- [ ] Deploy to Vercel or Netlify
- [ ] Test on multiple browsers/devices

## Future (v2)

- [ ] Unlimited practice mode
- [ ] Difficulty settings
- [ ] Historical stats view
- [ ] Shareable results (Wordle-style grid)
- [ ] Timer/speed mode
- [ ] Division problems
- [ ] Scientific notation toggle for display
