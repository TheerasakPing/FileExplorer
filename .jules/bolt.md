
## 2024-05-17 - O(N*M) Array.prototype.some within large render loops
**Learning:** Using `Array.prototype.some` inside the `map` iteration of a large list (like checking if an item `isSelected` within `FileList.jsx`) results in an O(N*M) operation, severely degrading render performance when dealing with directories with thousands of files and many selected items.
**Action:** Replace `Array.prototype.some` lookup checks inside loops with a pre-calculated `Set` created using `useMemo`, reducing the O(N*M) time complexity to O(N) by turning the lookup into an O(1) operation.
