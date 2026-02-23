## 2026-02-23 - [O(N^2) Selection Logic in File List]
**Learning:** File selection logic in `FileList.jsx` was O(N*M) where N is file count and M is selection count, causing render lag on large selections.
**Action:** Always use `Set` for O(1) lookups when checking item status in a list map loop, especially for potentially large datasets like file systems.
