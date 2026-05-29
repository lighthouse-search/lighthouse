# CLAUDE.md — Lighthouse frontend

## Context: this is an aging codebase mid-transition

Much of this code reflects **older practices** that are being phased out. Treat
existing patterns as legacy reference, not as the standard to copy. The owner is
actively changing conventions, so prefer the guidance below over "matching the
surrounding code" when the two conflict.

### Legacy patterns you'll see (don't propagate these in new code)
- Per-component plain `.css` files in nested `css/` folders.
- Hand-rolled utility classes in `global.css` (`row_gap_6`, `column_gap_10`,
  `column`, `row`, `surface_subtle`, etc.).
- Inline CSS custom properties / ad-hoc color values.

## Styling: prefer Tailwind for new work

Tailwind is set up (`@import "tailwindcss"` in `src/components/global.css`). For
**new** components and edits:

- **Use Tailwind utility classes wherever possible** instead of writing new CSS
  files or adding to `global.css`.
- Reach for a separate `.css` file only when Tailwind genuinely can't express it
  (complex selectors, keyframes, third-party overrides).
- When touching legacy components, it's fine to migrate the styles you're already
  editing over to Tailwind, but don't do sweeping unrelated rewrites unless asked.

When in doubt about a changing convention, ask rather than assuming the existing
code is the intended pattern.
