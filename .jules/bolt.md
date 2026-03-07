## 2024-05-24 - [Optimize selected item check in FileList rendering]
**Learning:** Checking for selected items inside `sortedItems.map` in `FileList.jsx` using `selectedItems.some` causes an O(N*M) lookup per render, where N is total items and M is selected items. This becomes noticeably slow when dealing with large folders and large selections.
**Action:** Always pre-compute a `Set` of selected paths using `useMemo` when rendering lists with selectable items to achieve O(1) lookups during the list render loop.
