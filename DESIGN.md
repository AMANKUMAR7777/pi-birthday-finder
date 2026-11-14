# Design System: Observatory Telemetry for Pi Exploration

## 1. Visual World & Character

The interface draws inspiration from precision astronomical observatories and mathematical telemetry displays. It treats $\pi$ not as a toy, but as an infinite landscape to be surveyed. Restraint, clarity, and crisp mechanical typography replace flashy gradients and emoji clutter.

## 2. Palette & Contrast

- **Deep Field (Base Background)**: `#0e131f` (RGB 14, 19, 31)
- **Panel Surface (Instrument Decks)**: `#151c2c` (RGB 21, 28, 44)
- **Panel Inset (Display Cells)**: `#0a0e17` (RGB 10, 14, 23)
- **Hairline Borders**: `#232f45` (RGB 35, 47, 69)
- **Text Primary**: `#f8fafc` (Slate 50) — High contrast ≥ 7:1
- **Text Secondary**: `#94a3b8` (Slate 400) — Contrast ≥ 4.5:1
- **Text Muted**: `#64748b` (Slate 500)
- **Instrument Gold (Telemetry/Index)**: `#f59e0b` / `#fbbf24`
- **Signal Emerald (Target Lock/Match)**: `#10b981` / `#34d399` (on dark inset `#064e3b`)
- **Signal Rose (Error/Alert)**: `#f43f5e`

## 3. Typography & Spacing

- Generous padding (16–24px) separating instrument modules.
- Monospace numerals for all data streams, positions, and Pi tape readouts.
- Proportional sans-serif for labels, explanations, and instructions.
- Strict adherence to craft floor: no kicker labels, no text gradients, no emojis disguised as icons.

## 4. Interaction & Motion

- Live digit streaming with steady clock ticks.
- Instant feedback on format selection.
- Seamless copy-to-clipboard actions with non-intrusive status confirmation.
- Safe historical presets: Albert Einstein (14 Mar 1879 - Pi Day), Moon Landing (20 Jul 1969). Never user private dates.
