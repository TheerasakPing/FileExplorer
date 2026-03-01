## 2024-05-24 - FileList selection uses O(N*M) lookup
**Learning:** `selectedItems.some(...)` inside `sortedItems.map(...)` leads to O(N*M) complexity when rendering `FileList`. `selectedItems` is an array.
**Action:** Use a `Set` or `useMemo` to convert `selectedItems` into a lookup structure with O(1) checks for selected state before the map operation. Wait, this was explicitly mentioned in my memory!

## 2024-05-24 - FileList clipboard checks use O(N*M) lookup
**Learning:** `clipboard.items?.some(...)` inside `sortedItems.map(...)` also leads to O(N*M) complexity.
**Action:** Use a `Set` derived via `useMemo` for O(1) lookups of clipboard items.
