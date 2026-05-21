# ADEDEJI — Portfolio OS

> A personal developer portfolio built with Dioxus 0.7 and Rust — compiled to WebAssembly. No JavaScript frameworks. No React. Just Rust, running in the browser at native speed.

**Live:** [https://dioxus.netlify.app](https://dioxus.netlify.app)

---

## About

This is not a template. This is a statement.

I started writing Rust because I wanted to build things that didn't apologize for being fast. Dioxus gave me a way to bring that philosophy to the web — full-stack, compiled, real. No runtime bloat. No compromise.

This portfolio is the proof of concept. Every pixel is deliberate. Every animation is intentional. Every line of Rust compiles to WebAssembly that runs at native speed in the browser.

One month into Rust. Already not going back.

---

## Tech Stack

| Layer | Technology |
|---|---|
| **Language** | Rust |
| **Framework** | Dioxus 0.7 |
| **Styling** | Pure CSS (custom design system) |
| **Animations** | GSAP + ScrollTrigger |
| **Build Target** | WebAssembly (WASM) |
| **Deployment** | Netlify |

---

## Design System

### Colors

| Token | Value | Usage |
|---|---|---|
| `--void` | `#080808` | Near-black background |
| `--surface` | `#111111` | Card/panel backgrounds |
| `--border` | `#1E1E1E` | Subtle borders |
| `--mist` | `#2A2A2A` | Hover states, dividers |
| `--text` | `#F0EDE6` | Warm off-white body text |
| `--sub` | `#6B6760` | Secondary text, captions |
| `--brand` | `#E8FF47` | Acid yellow-green accent |
| `--brand-dim` | `#B8CC3A` | Hover state of brand |
| `--rust` | `#C14B2A` | Rust-orange secondary accent |

### Typography

| Role | Font | Weight |
|---|---|---|
| Display / Hero | Syne | 700, 800 |
| Body / Prose | Instrument Serif | 400, italic |
| Code / Labels | DM Mono | 300, 400 |

---

## Project Structure

```
src/
  main.rs              ← Entry point
  app.rs               ← Root component with scroll progress bar
  components/
    mod.rs             ← Module exports
    navbar.rs          ← Fixed navbar with scroll-aware background
    hero.rs            ← Full-viewport hero with GSAP entrance animations
    about.rs           ← About section with scroll-triggered animation
    stack.rs           ← 6-card tech stack grid
    work.rs            ← 3 project cards with conditional styling
    process.rs         ← 4-step horizontal process section
    contact.rs         ← Contact section with footer
  styles/
    input.css          ← Global styles, design tokens, animations
assets/
  style.css            ← Compiled CSS (generated from input.css)
```

---

## Getting Started

### Prerequisites

- Rust toolchain (`rustup`)
- Dioxus CLI (`dx`)

```sh
curl -sSL http://dioxus.dev/install.sh | sh
```

### Development

```sh
dx serve
```

### Production Build

```sh
cargo build --release --features web
dx build --release --platform web
```

Output is in `target/dx/award/release/web/public/`.

---

## Deployment

### Netlify (Drop)

```sh
netlify deploy --dir target/dx/award/release/web/public --prod
```

### Netlify (CI/CD)

Connect your GitHub repository to Netlify and set:
- **Build command:** `dx build --release --platform web`
- **Publish directory:** `target/dx/award/release/web/public`

---

## Philosophy

> Rust developers don't ship excuses. They ship binaries.

This site was built with a single constraint: **no JavaScript frameworks**. Not because JS is bad — because Rust is better for what I build. Every component is a Rust function. Every style is a CSS rule. Every animation is a GSAP call from WASM.

The result is a site that loads fast, looks deliberate, and proves that Rust on the web isn't a novelty — it's a choice.

---

## License

MIT

---

*Built with Rust & Dioxus. Compiled to WebAssembly. Deployed on Netlify.*
