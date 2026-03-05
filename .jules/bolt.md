## 2024-03-24 - [Avoid O(N*M) lookups in React renders]
**Learning:** Checking state derived from arrays (like `selectedItems.some()` and `clipboard.items?.some()`) inside a large `.map()` render loop causes $O(N \times M)$ complexity. When rendering hundreds of files with multiple selections, this leads to significant layout thrashing and slow render times.
**Action:** Always derive `Set` objects using `useMemo` for selection/cut state lookups *outside* of list render loops to achieve $O(1)$ lookup performance and maintain linear render complexity.
