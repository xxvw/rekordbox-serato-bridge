# rekordbox-serato-bridge

> Convert and bridge DJ library exports between **rekordbox** and **Serato**.

`rsb` is a standalone CLI for moving track metadata, hot cues, beatgrids, and memory cues across the two dominant pro-DJ ecosystems. It accepts and produces:

| Format | Read | Write | Notes |
|--------|------|-------|-------|
| `rekordbox-xml` | planned | planned | `DJ_PLAYLISTS` XML exported from rekordbox.app |
| `rekordbox-usb` | planned | planned | CDJ-2000 compatible USB layout (`PIONEER/` + `Contents/`) |
| `serato` | planned | planned | ID3 `GEOB` frames embedded in audio files |

This project is intentionally separate from any GUI host. It is also the conversion backend used by [Conduction](https://github.com/xxvw/Conduction) internally — but the CLI is self-contained.

License: [MIT](./LICENSE)
日本語: [README_ja.md](./README_ja.md)

---

## Status

Pre-alpha. The CLI surface (`convert` / `inspect`) is in place but no real conversion is wired up yet. Track progress in the issues tab.

---

## Install

Requires Rust stable (pinned via `rust-toolchain.toml`).

```bash
# Run from source
cargo run -- --help

# Install the binary as `rsb`
cargo install --path .
```

---

## Usage

```bash
# Convert a rekordbox XML library into a Serato-tagged folder
rsb convert \
  --from rekordbox-xml \
  --to   serato \
  --input  ./rekordbox.xml \
  --output ./music/

# Convert a Serato-tagged folder back into a rekordbox XML
rsb convert \
  --from serato \
  --to   rekordbox-xml \
  --input  ./music/ \
  --output ./rekordbox.xml

# Inspect a library without writing anything
rsb inspect --format rekordbox-xml --input ./rekordbox.xml
```

Use `--dry-run` on `convert` to print the planned operations without touching the filesystem.

---

## Design

- **Single binary, no daemon.** All conversion is stateless.
- **Lossless mapping where possible.** When a destination format lacks a concept the source has (e.g. typed cues like Drop/Intro), the CLI records the loss in the output report rather than silently dropping it.
- **No audio analysis.** Beatgrids and waveforms are passed through, not recomputed. Pair with [Conduction](https://github.com/xxvw/Conduction) or another analyzer if you need to (re)generate them.

---

## Roadmap

1. rekordbox XML reader + writer + roundtrip fixtures
2. Serato GEOB tag reader + writer
3. rekordbox-xml ↔ serato bridge (the main use case)
4. rekordbox USB (PIONEER/ANLZ + PDB) reader and writer via [`rekordcrate`](https://github.com/Holzhaus/rekordcrate)
5. Lossless-roundtrip integration tests against real fixtures

---

## License

MIT — see [`LICENSE`](./LICENSE).
