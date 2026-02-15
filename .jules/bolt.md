## 2024-05-23 - DOM Thrashing in FileList
**Learning:** The `FileList` component was calculating the number of columns by querying the DOM (`querySelectorAll`) and reading layout properties (`getBoundingClientRect`, `getComputedStyle`) inside a `resize` handler (even though it was debounced). This caused significant layout thrashing.
**Action:** When implementing grid layouts, prefer pure mathematical calculations based on container width (observed via `ResizeObserver`) and CSS constants, rather than querying the DOM for item positions. This avoids the Read-Write-Read cycle.
