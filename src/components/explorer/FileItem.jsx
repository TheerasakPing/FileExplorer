import React, { memo } from 'react';
import FileIcon from './FileIcon';
import { formatFileSize, formatDate, getFileType } from '../../utils/formatters';
import './fileItem.css';

/**
 * Component that displays a single file or directory item
 * @param {Object} props - Component properties
 * @param {Object} props.item - The file or directory object to display
 * @param {number} props.index - The index of the item in the list
 * @param {string} [props.viewMode='grid'] - The view mode: 'grid', 'list', or 'details'
 * @param {boolean} [props.isSelected=false] - Whether the item is currently selected
 * @param {boolean} [props.isFocused=false] - Whether the item is currently focused for keyboard navigation
 * @param {boolean} [props.isCut=false] - Whether the item is marked for cut operation
 * @param {Function} props.onItemClick - Click handler function (item, index, event)
 * @param {Function} props.onItemDoubleClick - Double-click handler function (item, index, event)
 * @returns {React.ReactElement} File/directory item component
 */
const FileItem = memo(({
                      item,
                      index,
                      viewMode = 'grid',
                      isSelected = false,
                      isFocused = false,
                      isCut = false,
                      onItemClick,
                      onItemDoubleClick
                  }) => {

    const isDirectory = item.isDirectory || 'sub_file_count' in item;
    const fileType = isDirectory ? 'Folder' : getFileType(item.name);
    const size = isDirectory
        ? `${item.sub_file_count || 0} items`
        : formatFileSize(item.size_in_bytes);

    // Format modified date
    const modified = formatDate(item.last_modified);

    /**
     * Handles click events on the file item
     * @param {React.MouseEvent} e - The click event
     */
    const handleClick = (e) => {
        if (onItemClick) onItemClick(item, index, e);
    };

    /**
     * Handles mouse down events to prevent text selection
     * @param {React.MouseEvent} e - The mouse down event
     */
    const handleMouseDown = (e) => {
        // Prevent text selection on mouse down
        e.preventDefault();
    };

    /**
     * Handles selectstart events to prevent text selection
     * @param {React.SyntheticEvent} e - The selectstart event
     */
    const handleSelectStart = (e) => {
        // Prevent any text selection
        e.preventDefault();
        return false;
    };

    /**
     * Handles double-click events on the file item
     * @param {React.MouseEvent} e - The double-click event
     */
    const handleDoubleClick = (e) => {
        // Prevent text selection on double-click
        e.preventDefault();
        e.stopPropagation();
        
        // Clear any existing text selection
        if (window.getSelection) {
            window.getSelection().removeAllRanges();
        }
        
        if (onItemDoubleClick) onItemDoubleClick(item, index, e);
    };

    return (
        <div
            className={`file-item view-mode-${viewMode.toLowerCase()} ${isSelected ? 'selected' : ''} ${isFocused ? 'focused' : ''} ${isDirectory ? 'directory' : 'file'} ${isCut ? 'cut' : ''}`}
            onClick={handleClick}
            onDoubleClick={handleDoubleClick}
            onMouseDown={handleMouseDown}
            onSelectStart={handleSelectStart}
            data-path={item.path}
        >
            {viewMode === 'grid' && (
                <div className="file-item-grid">
                    <div className="file-icon-container">
                        <FileIcon filename={item.name} isDirectory={isDirectory} />
                    </div>

                    <div className="file-name truncate" title={item.name}>
                        {item.name}
                    </div>
                </div>
            )}

            {viewMode === 'list' && (
                <div className="file-item-list">
                    <div className="file-icon-container">
                        <FileIcon filename={item.name} isDirectory={isDirectory} />
                    </div>

                    <div className="file-details">
                        <div className="file-name truncate" title={item.name}>
                            {item.name}
                        </div>
                        <div className="file-info truncate">
                            {size} • {modified}
                        </div>
                    </div>
                </div>
            )}

            {viewMode === 'details' && (
                <div className="file-item-details">
                    <div className="file-column column-name">
                        <div className="file-icon-container">
                            <FileIcon filename={item.name} isDirectory={isDirectory} />
                        </div>

                        <div className="file-name truncate" title={item.name}>
                            {item.name}
                        </div>
                    </div>

                    <div className="file-column column-size">
                        {size}
                    </div>

                    <div className="file-column column-type">
                        {fileType}
                    </div>

                    <div className="file-column column-modified">
                        {modified}
                    </div>
                </div>
            )}
        </div>
    );
});

export default FileItem;
