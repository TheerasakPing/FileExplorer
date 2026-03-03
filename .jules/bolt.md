# Bolt's Journal

## 2026-03-03 - [Prevent O(N*M) lookups in render loops]
**Learning:** Using `Array.prototype.some` for state checks (like selection or clipboard status) inside a list rendering loop (`map`) leads to O(N*M) complexity, causing significant render blocking for large directories.
**Action:** Pre-compute lookup `Set`s via `useMemo` and use `Set.has()` to achieve O(1) lookups inside list renders, reducing complexity to O(N+M).
