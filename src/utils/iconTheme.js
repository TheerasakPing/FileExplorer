import { invoke } from '@tauri-apps/api/core';
import { convertFileSrc } from '@tauri-apps/api/core';

export const importIconTheme = async (filePath) => {
    try {
        return await invoke('import_icon_theme', { filePath });
    } catch (error) {
        console.error('Failed to import icon theme:', error);
        throw error;
    }
};

export const getAvailableThemes = async () => {
    try {
        return await invoke('get_available_themes');
    } catch (error) {
        console.error('Failed to get available themes:', error);
        return [];
    }
};

export const setActiveTheme = async (themeId) => {
    try {
        await invoke('set_active_theme', { themeId });
        // Dispatch event to notify components to re-render icons if needed
        window.dispatchEvent(new Event('icon-theme-changed'));
    } catch (error) {
        console.error('Failed to set active theme:', error);
    }
};

export const getFileIconPath = async (filename, isDir, isOpened = false) => {
    try {
        const iconPath = await invoke('get_file_icon', { filename, isDir, isOpened });
        if (iconPath) {
            return convertFileSrc(iconPath);
        }
    } catch (error) {
        // console.warn('Failed to get file icon:', error);
    }
    return null;
};
