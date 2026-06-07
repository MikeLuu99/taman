# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Run the app
cargo run

# Build release binary
cargo build --release

# Check for compile errors without building
cargo check

# Run tests
cargo test

# Run a single test
cargo test <test_name>

# Lint
cargo clippy

# Format
cargo fmt
```

## Architecture

Taman is a terminal Pomodoro timer built with [Ratatui](https://github.com/ratatui-org/ratatui). The main event loop lives in `src/main.rs`: it draws the UI, polls keyboard events via crossterm, and calls `app.tick()` every 250ms.

### Data flow

```
crossterm event → input::handle_key() → InputAction → app.handle_input()
                                                              ↓
                                                     app.tick() (250ms)
                                                              ↓
                                                    timer → plant → garden → statistics
                                                              ↓
                                                    app.save() → storage::save_data()
```

### Core modules

- **`app.rs`** — `App` is the central state struct. It owns all sub-systems and handles user input by dispatching `InputAction` variants. `tick()` is the heartbeat: it decrements the timer and, when a session finishes, awards plant growth, updates statistics, and advances the auto-run queue.
- **`timer.rs`** — `Timer` owns session state (`Idle/Running/Paused/Finished`) and the auto-run queue (`Vec<SessionType>`). `tick()` returns `true` only on session completion.
- **`plant.rs`** — `Plant` grows via `add_growth()` (called once per completed session). 10 growth points = one complete plant; stages map to emoji (🌰→🍃→🌱→🌿→🪴).
- **`garden.rs`** — `Garden` holds completed plants and computes daily streaks from `recent_sessions` timestamps.
- **`storage.rs`** — `Data` is the single serialized struct written to `~/.config/taman/data.json`. `Settings` (durations + theme) and `Statistics` (session logs + per-day breakdowns) are nested inside it.
- **`theme.rs`** — `Theme` is a flat struct of `ratatui::style::Color` values. `Theme::new(ThemeVariant)` is the only constructor; add a new theme by extending the `match` arm there.
- **`input.rs`** — Thin crossterm → `InputAction` mapping; no logic lives here.
- **`ui/`** — One file per tab (`timer_ui`, `plant_ui`, `stats_ui`, `settings_ui`). Each exports a single `draw_*` function that receives `&Frame`, `&App`/`&mut App`, and the layout `Rect`. UI modules are read-only with respect to app state (except `timer_ui` which takes `&mut App` for scrollbar state).

### State persistence

`App::save()` is called both on quit and after each successful session completion. `load_data()` is called once at startup in `App::new()`. The JSON schema is `storage::Data`; adding new persistent fields requires updating `Data`, deriving `Default`, and handling deserialization of old saves via `serde` defaults.

### Adding a new theme

1. Add a variant to `ThemeVariant` in `src/theme.rs`.
2. Add a `match` arm in `Theme::new()` returning the color values.
3. Add the variant to the `themes` slice in `App::adjust_setting()` in `src/app.rs`.
