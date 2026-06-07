# Taman (fork)

This is a personal fork of [taman](https://github.com/harmoneer/taman) by harmoneer, adding a **Todos** tab and a **Garden** collection tab on top of the original app.

## What's added in this fork

### ✅ Todos tab (`2`)
A keyboard-driven task list living alongside the Pomodoro timer.

- **Add** a todo with `n`, type the title, confirm with Enter
- **Edit** a todo in place with `e`
- **Toggle done** with Enter — marks the task complete and plants a flower in your Garden
- **Delete** with Backspace / Delete
- Navigate with ↑/↓

### 🌺 Garden tab (`3`)
A growing collection of everything you've completed.

- Each **completed todo** adds a flower (🌸🌺🌻🌹🌷🪷🌼💐, cycling as you complete more)
- Each **fully grown plant** from the Plant tab also appears here as 🪴
- Items are displayed as an emoji grid, scrollable with ↑/↓

## Tab layout (this fork)

| Key | Tab |
|-----|-----|
| `1` | ⏳ Timer |
| `2` | ✅ Todos |
| `3` | 🌺 Garden |
| `4` | 🌱 Plant |
| `5` | 📊 Stats |
| `6` | ⚙️ Settings |

## Running from source

```bash
git clone <this-repo>
cd taman
cargo run
# or for a release build:
cargo build --release && ./target/release/taman
```

Requires Rust 1.70+ and a terminal with Unicode support.

## Original project

All original features (Pomodoro timer, plant growth, statistics, themes, streak tracking) are by [harmoneer](https://github.com/harmoneer/taman). Check out the original repo and support the author there.
