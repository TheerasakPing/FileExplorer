## 2024-05-18 - [Fix hidden O(N*M) complexity in React renders]
**Learning:** Checking for selection or cut state inside a large `.map` by calling `Array.prototype.some` on an array of selected/cut items creates a hidden O(N*M) complexity during rendering. For thousands of files and many selected items, this can freeze the UI.
**Action:** Use `useMemo` to convert arrays of selected/cut item identifiers into a `Set` before mapping over large lists. `Set.prototype.has` provides O(1) lookups, reducing render complexity to O(N+M).
