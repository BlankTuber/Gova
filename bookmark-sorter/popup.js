document.getElementById('sort').addEventListener('click', handleSortClick);

function handleSortClick() {
    const sortingMethodSelect = document.getElementById('sorting-method-select');
    const selectedMethod = sortingMethodSelect.value;

    chrome.bookmarks.getTree((bookmarkTreeNodes) => {
        const allBookmarks = flattenBookmarkTree(bookmarkTreeNodes[0]);
        let sortedBookmarks = sortBookmarks(allBookmarks, selectedMethod);
        let indexOfBookmarkBar;
        for (let i = 0; i < sortedBookmarks.length; i++) {
            const element = sortedBookmarks[i];
            if (element.title === "Bookmarks bar") {
                indexOfBookmarkBar = i;
                break;
            }
        }

        if (indexOfBookmarkBar !== undefined) {
            const newSortedBookmarks = [];
            newSortedBookmarks.push(sortedBookmarks[indexOfBookmarkBar]);
            newSortedBookmarks.push(...sortedBookmarks.slice(0, indexOfBookmarkBar), ...sortedBookmarks.slice(indexOfBookmarkBar + 1));
            sortedBookmarks = newSortedBookmarks;
        }

        
        function updateBookmarksInChrome(bookmarks, parentId) {
            bookmarks.forEach((bookmark, index) => {
                if (bookmark.url) {
                    
                    chrome.bookmarks.get(bookmark.id, (result) => {
                        if (result && result.length > 0) {
                            chrome.bookmarks.move(bookmark.id, { parentId, index }, () => {
                                console.log(`Moved bookmark ${bookmark.title} to index ${index} under ${parentId}`);
                            });
                        } else {
                            chrome.bookmarks.create({ parentId, index, title: bookmark.title, url: bookmark.url }, () => {
                                console.log(`Created bookmark ${bookmark.title} at index ${index} under ${parentId}`);
                            });
                        }
                    });
                } else {
                    
                    chrome.bookmarks.get(bookmark.id, (result) => {
                        let folderId;
                        if (result && result.length > 0) {
                            folderId = result[0].id;
                            chrome.bookmarks.move(folderId, { parentId, index }, () => {
                                console.log(`Moved folder ${bookmark.title} to index ${index} under ${parentId}`);
                                updateBookmarksInChrome(bookmark.children, folderId);
                            });
                        } else {
                            chrome.bookmarks.create({ parentId, index, title: bookmark.title }, (newFolder) => {
                                folderId = newFolder.id;
                                console.log(`Created folder ${newFolder.title} at index ${index} under ${parentId}`);
                                updateBookmarksInChrome(bookmark.children, folderId);
                            });
                        }
                    });
                }
            });
        }

        
        const bookmarksBarId = allBookmarks.find(bookmark => bookmark.title === "Bookmarks bar").id;

        
        const reconstructedTree = reconstructTree(sortedBookmarks);

        updateBookmarksInChrome(reconstructedTree, bookmarksBarId);
    });
}

function sortBookmarks(bookmarks, method) {
    const sortingFunctions = {
        dateAdded: (bookmarks) => sortBookmarksByDate(bookmarks, "desc"),
        dateAddedOrder: (bookmarks) => sortBookmarksByDate(bookmarks, "asc"),
        title: (bookmarks) => sortBookmarksByTitle(bookmarks, "asc"),
        titleOrder: (bookmarks) => sortBookmarksByTitle(bookmarks, "desc")
    };

    return sortingFunctions[method](bookmarks);
}

function flattenBookmarkTree(node) {
    const bookmarks = [];

    if (node.url) {
        bookmarks.push({
            id: node.id,
            title: node.title,
            url: node.url,
            parentId: node.parentId,
            index: node.index,
            dateAdded: node.dateAdded,
        });
    } else {
        bookmarks.push({
            id: node.id,
            title: node.title,
            parentId: node.parentId,
            index: node.index,
            dateAdded: node.dateAdded,
            children: node.children ? node.children.map(child => child.id) : []
        });
    }

    if (node.children) {
        node.children.forEach(child => {
            bookmarks.push(...flattenBookmarkTree(child));
        });
    }

    return bookmarks;
}

function sortBookmarksByDate(bookmarks, order = "desc") {
    return bookmarks.sort((a, b) => {
        const dateA = a.dateAdded || 0;
        const dateB = b.dateAdded || 0;
        if(order == "desc"){
            return dateB - dateA;
        } else if (order == "asc") {
            return dateA - dateB;
        } else {
            return dateB - dateA;
        }
    });
}

function sortBookmarksByTitle(bookmarks, order = "asc") {
    if (order == "asc") {
        return bookmarks.sort((a, b) => a.title.localeCompare(b.title));
    } else if (order == "desc") {
        return bookmarks.sort((a, b) => b.title.localeCompare(a.title));
    } else {
        return bookmarks.sort((a, b) => a.title.localeCompare(b.title));
    }
}

function reconstructTree(flatBookmarkList) {
    const nodeMap = new Map();
    const rootNode = {
        id: '0',
        children: [],
        title: 'root',
        dateAdded: Date.now()
    };
    nodeMap.set('0', rootNode);

    for (const bookmark of flatBookmarkList) {
        const node = {
            id: bookmark.id,
            title: bookmark.title,
            url: bookmark.url,
            dateAdded: bookmark.dateAdded,
            children: []
        };
        nodeMap.set(bookmark.id, node);
    }

    for (const bookmark of flatBookmarkList) {
        const parentNode = nodeMap.get(bookmark.parentId) || rootNode;
        parentNode.children.push(nodeMap.get(bookmark.id));
    }

    return rootNode.children;
}