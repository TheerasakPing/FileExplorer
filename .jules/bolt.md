## 2024-03-08 - FileList Calculate Columns Thrashing
**Learning:** `FileList.jsx` recalculates columns using DOM reads (`window.getComputedStyle`, `getBoundingClientRect`) directly in a `ResizeObserver` callback. This is an anti-pattern as it causes layout thrashing and forced synchronous layouts.
**Action:** Replace direct DOM reads with a lightweight CSS variable approach or use standard ResizeObserver sizing data (`entry.contentRect.width`) instead of manual DOM measuring.
