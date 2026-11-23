# Build your own Paint

A small drawing app written in Rust from scratch, and the code that goes with the tutorial of the same name.

You open a window, draw rectangles, circles and lines with the mouse, select them, move them around, pick a color and save the result as an image. It's about 800 lines of Rust with a single dependency, [minifb](https://crates.io/crates/minifb), which only opens the window and gives us the keyboard and the mouse. Everything else is done by hand: filling pixels, drawing lines, hit-testing shapes, converting colors.

The tutorial walks through building it step by step, starting from an empty `cargo new`. It's aimed at people who have read the first chapters of the Rust Book and want to write a real program with it. It's available in English and French.

The website is coming soon. I'll put the link here.

## Running it

You need Rust 1.88 or newer (`rustup update` if you're not sure).

```bash
git clone https://github.com/theZackD/build-your-own-paint.git
cd build-your-own-paint
cargo run --release
```

On Linux, minifb needs a few system libraries. On Debian or Ubuntu:

```bash
sudo apt install libxkbcommon-dev libwayland-cursor0 libwayland-dev
```

Nothing to install on macOS or Windows.

## How to use it

- Click and drag to draw. The buttons in the top-right corner switch between rectangle, circle and line, or use the R, C and L keys.
- Click on a shape to select it, drag it to move it, and press Delete (or Backspace) to remove it.
- The last button opens the color picker. Click it again to close it.
- Press S to save the drawing in the current folder, as `export_1.ppm`, `export_2.ppm` and so on. Only the shapes are saved, not the toolbar.
- Escape quits.

The images are saved in PPM, an old and very simple format. Preview opens them on macOS, and most image viewers do on Linux. On Windows you'll need something like GIMP or IrfanView.

## How the repo is organized

- `master` has the final code.
- `chapters` has one commit per chapter of the tutorial, starting from an empty project. Each commit is tagged, from `ch00-setup` to `ch10-export`, so you can get the code at the end of any chapter:

```bash
git checkout ch05-mouse
```

And see exactly what a chapter adds:

```bash
git diff ch04-window ch05-mouse
```

Every tag compiles and runs. The last one, `ch10-export`, is the same code as `master`.

| Tag | Chapter |
|---|---|
| `ch00-setup` | Setup |
| `ch01-vec2` | 2D vectors |
| `ch02-ppm` | An image in memory |
| `ch03-shapes` | Drawing shapes |
| `ch04-window` | A window |
| `ch05-mouse` | The mouse |
| `ch06-shapes-enum` | Several shapes |
| `ch07-select-move` | Selecting and moving |
| `ch08-toolbar` | The toolbar |
| `ch09-colors` | Colors |
| `ch10-export` | Saving |

## Videos

Later on, I'm planning to record a video for each chapter, for anyone who'd rather code along with me than read. I'll link them here once they're up.

## Found a problem?

If something doesn't compile, doesn't match the tutorial, or isn't clear, please open an issue. Tell me which chapter, your OS and your Rust version (`rustc --version`). It really helps.

## License

MIT, see [LICENSE](LICENSE). Use the code however you like, in your own projects or to teach.
