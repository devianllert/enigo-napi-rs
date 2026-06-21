# @dvnllrt/enigo-napi-rs

Node.js bindings for [enigo](https://github.com/enigo-rs/enigo) — a Rust library for mouse and keyboard control.

## Supported platforms

Build and publish are configured for macOS and Windows only

## Install

```bash
pnpm add @dvnllrt/enigo-napi-rs
# or
npm install @dvnllrt/enigo-napi-rs
```

## API

```ts
import {
  moveMouseRel,
  moveMouseAbs,
  mouseClick,
  mouseDown,
  mouseUp,
  mouseScroll,
} from '@dvnllrt/enigo-napi-rs'

// Move the cursor relative to its current position
moveMouseRel(10, -5)

// Move to absolute screen coordinates
moveMouseAbs(500, 300)

// Click, press, and release a button
mouseClick('left')   // 'left' | 'right' | 'middle'
mouseDown('left')
mouseUp('left')

// Scroll: length is the amount, isVertical is the axis (true = vertical)
mouseScroll(3, true)
```

## Development requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- Node.js 10+
- [pnpm](https://pnpm.io/) — pinned in this project via `corepack`

```bash
corepack enable
pnpm install
```

## Build

### Release (local, current OS)

```bash
pnpm build
```

Produces `enigo-napi-rs.<platform>.node` in the project root.

### Debug build

```bash
pnpm build:debug
```

## CI and release

GitHub Actions runs on every push to `main`:

1. **Lint** — `oxlint`, `cargo fmt`, `clippy`
2. **Build** — compile all targets from `package.json`
3. **Publish** — publish to npm on versioned release commits

To release:

```bash
npm version patch   # or minor / major
git push
```

Do not publish manually with `npm publish` — CI handles that.

## License

MIT
