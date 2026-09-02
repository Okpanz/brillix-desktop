# Brillix Desktop

Native desktop shell for [Brillix](../biz-insight), built with [Tauri](https://tauri.app) v2.

This project does not contain its own frontend source. It wraps the existing
`biz-insight` React app (a sibling folder) as-is:

- `tauri dev` starts `biz-insight`'s Vite dev server (`npm run dev`, port 8080) and opens a
  native window pointed at it.
- `tauri build` runs `biz-insight`'s production build (`npm run build`) and bundles the
  resulting `biz-insight/dist` into a native app.

Any UI change lives in `../biz-insight/src`, not here. This folder only holds the
native shell: window/menu/tray configuration, OS-level plugins (dialogs, notifications,
filesystem access, etc.), and packaging.

## Prerequisites

- Node.js (matches whatever `biz-insight` requires — no version currently pinned there).
- Rust (via [rustup](https://rustup.rs)).
- Platform build tools:
  - **macOS**: Xcode Command Line Tools.
  - **Windows**: WebView2 runtime (preinstalled on Windows 11; usually present on Windows 10
    via Edge updates).
  - **Linux**: `webkit2gtk-4.1`, `libayatana-appindicator3`, `librsvg2-dev`, and standard
    build tooling — see the [Tauri Linux prerequisites](https://tauri.app/start/prerequisites/)
    for exact package names on your distro.

## Development

```bash
npm install
npm run dev
```

The desktop app talks to the same NestJS API `biz-insight` talks to
(`VITE_NESTJS_API_URL`, defaulting to the production Render-hosted API — see
`../biz-insight/src/integrations/nestjs/client.ts`). To point dev builds at a local
backend instead, export `VITE_NESTJS_API_URL=http://localhost:3005/api` in this shell
before running `npm run dev`.

## Building

```bash
npm run build
```

Produces platform-native installers/bundles under `src-tauri/target/release/bundle/`.

**Important:** production builds must not pick up `biz-insight/.env`'s local
`VITE_NESTJS_API_URL=http://localhost:3005/api` value — confirm the build step consults
the right Vite mode/env file before shipping a build to real users.
