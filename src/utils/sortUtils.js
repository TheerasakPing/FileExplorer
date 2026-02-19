import { getFileType } from './formatters';

/**
 * Sorts files and directories based on the provided configuration.
 * @param {Object} data - The data object containing directories and files arrays.
 * @param {Object} sortConfig - The sort configuration object { key, direction }.
 * @returns {Array} The sorted array of items.
 */
export const sortItems = (data, sortConfig) => {
    if (!data || (!data.directories?.length && !data.files?.length)) {
        return [];
    }

    const { key, direction } = sortConfig;

    // Combine directories and files for sorting
    const combinedItems = [
        ...(data.directories || []).map(dir => ({ ...dir, isDirectory: true })),
        ...(data.files || []).map(file => ({ ...file, isDirectory: false }))
    ];

    // Always put directories first
    return combinedItems.sort((a, b) => {
        // Directories always come before files
        if (a.isDirectory && !b.isDirectory) return -1;
        if (!a.isDirectory && b.isDirectory) return 1;

        let aValue, bValue;
        if (key === 'size_in_bytes') {
            // Folders: always sort as 0 (or -1) so they group together and don't mix with files
            aValue = a.isDirectory ? -1 : a.size_in_bytes || 0;
            bValue = b.isDirectory ? -1 : b.size_in_bytes || 0;
        } else if (key === 'type') {
            // Folders: always 'Folder', Files: use getFileType
            aValue = a.isDirectory ? 'Folder' : (a.name ? getFileType(a.name) : '');
            bValue = b.isDirectory ? 'Folder' : (b.name ? getFileType(b.name) : '');
            aValue = aValue.toLowerCase();
            bValue = bValue.toLowerCase();
        } else if (key === 'created' || key === 'last_modified' || key === 'accessed') {
            aValue = new Date(a[key]).getTime();
            bValue = new Date(b[key]).getTime();
        } else {
            aValue = a[key];
            bValue = b[key];
            if (typeof aValue === 'string' && typeof bValue === 'string') {
                aValue = aValue.toLowerCase();
                bValue = bValue.toLowerCase();
            }
        }

        if (aValue < bValue) return direction === 'asc' ? -1 : 1;
        if (aValue > bValue) return direction === 'asc' ? 1 : -1;
        return 0;
    });
};
