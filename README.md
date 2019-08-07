# pasteboard

This crate provides both a CLI and Rust bindings for interacting with the macOS pasteboard.

## CLI

At the command-line, `pasteboard` introduces a `pb` command which can be used like:

```bash
# Copy/paste string
pb copy /path/to/foo.txt
pb paste /path/to/bar.txt

# Copy image/sound
pb copy /path/to/foo.png -t image
pb copy /path/to/foo.mp3 -t sound
```

## Rust bindings

From Rust, `pasteboard` exposes a `Pasteboard` enum with `copy` and `paste` methods which can be used like:

```rust
use pasteboard::Pasteboard;

fn main() {
    unsafe {
        // Copy/paste string
        Pasteboard::String.copy("/path/to/foo.txt");
        Pasteboard::String.paste("/path/to/bar.txt");

        // Copy image/sound
        Pasteboard::Image.copy("/path/to/foo.png");
        Pasteboard::Sound.copy("/path/to/foo.png");
    }
}
```

## Notes

The `pb` command is able to copy files as strings, images, and sounds. However, it can only paste as strings currently, but more coverage is planned in future releases.
