## 2024-05-24 - [Avoid O(N*M) Array Iteration in Render Loops]
**Learning:** Found an anti-pattern in `FileList.jsx` where `.some()` was used inside a `.map()` during rendering to check if items were selected or cut, causing O(N*M) complexity.
**Action:** Use `useMemo` to convert arrays to `Set` objects for O(1) lookups before rendering lists to prevent performance degradation with large data sets.
