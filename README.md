# @natsuneko-laboratory/rust-sass

This package is an alternative to the [`sass`](https://www.npmjs.com/package/sass) package but use [`grass`](https://github.com/connorskees/grass) as Sass compiler.

## Why use this package?

- `sass` (JS-compiled) is an easy to use but 2.5x and 10x slower than Dart VM.
- `sass` (Dart VM) is a high-performance but dependent on platform outside of Node.js.

## Performance

- `dart` : ~10000ms (10s)
  - Run: `pnpm run test:dart`
- `node` : ~2000ms (2s)
  - Run: `pnpm run test:node`
- `rust` : ~1500ms (1.5s)
  - Run: `pnpm run test:rust`

## License

MIT by [@6jz](https://twitter.com/6jz)
