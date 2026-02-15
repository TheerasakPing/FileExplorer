
    // Watch directory changes
    useEffect(() => {
        let unlisten = null;

        const setupWatcher = async () => {
            if (currentPath && !isSftpPath(currentPath)) {
                try {
                    // Start watching the directory
                    await invoke('watch_directory', { path: currentPath });

                    // Listen for changes
                    unlisten = await listen('fs-change', (event) => {
                        // Reload the current directory
                        loadDirectory(currentPath);
                    });
                } catch (err) {
                    console.error('Failed to setup file watcher:', err);
                }
            } else {
                try {
                    await invoke('unwatch_directory');
                } catch (e) {
                    // Ignore errors if unwatch fails
                }
            }
        };

        setupWatcher();

        return () => {
            if (unlisten) unlisten();
        };
    }, [currentPath, loadDirectory, isSftpPath]);
