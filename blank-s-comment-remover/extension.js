const vscode = require('vscode');
const fs = require('fs');
const path = require('path');

const defaultCommentPatterns = {
	"js": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"ts": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"jsx": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"tsx": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"py": {
		"singleLine": "(?<![\\w:&#])[#].*$",
		"block": "\"\"\"[\\s\\S]*?\"\"\"|'''[\\s\\S]*?'''"
	},
	"java": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"c": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"cpp": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"html": {
		"singleLine": null,
		"block": "<!--[\\s\\S]*?-->"
	},
	"css": {
		"singleLine": null,
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"php": {
		"singleLine": "(?<![:\\w])\\/\\/.*$|(?<![\\w:&#])[#].*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"json": {
		"singleLine": null,
		"block": null
	},
	"xml": {
		"singleLine": null,
		"block": "<!--[\\s\\S]*?-->"
	},
	"yaml": {
		"singleLine": "(?<![\\w:&#])[#].*$",
		"block": null
	},
	"yml": {
		"singleLine": "(?<![\\w:&#])[#].*$",
		"block": null
	},
	"toml": {
		"singleLine": "(?<![\\w:&#])[#].*$",
		"block": null
	},
	"rb": {
		"singleLine": "(?<![\\w:&#])[#].*$",
		"block": "^=begin[\\s\\S]*?^=end"
	},
	"swift": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"go": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"rs": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"sql": {
		"singleLine": "--(?!\\[|\\w).*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"sh": {
		"singleLine": "(?<![\\w:&#])[#].*$",
		"block": null
	},
	"bash": {
		"singleLine": "(?<![\\w:&#])[#].*$",
		"block": null
	},
	"bat": {
		"singleLine": "(?<!\\w)REM\\s.*$",
		"block": null
	},
	"ps1": {
		"singleLine": "(?<![\\w:&#])[#].*$",
		"block": "<#[\\s\\S]*?#>"
	},
	"pl": {
		"singleLine": "(?<![\\w:&#])[#].*$",
		"block": "^=pod[\\s\\S]*?^=cut"
	},
	"lua": {
		"singleLine": "--(?!\\[\\[).*$",
		"block": "--\\[\\[[\\s\\S]*?\\]\\]"
	},
	"kt": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"scala": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"dart": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"r": {
		"singleLine": "(?<![\\w:&#])[#].*$",
		"block": null
	},
	"m": {
		"singleLine": "(?<!\\w)%.*$",
		"block": "%\\{[\\s\\S]*?%\\}"
	},
	"asm": {
		"singleLine": ";.*$",
		"block": null
	},
	"s": {
		"singleLine": ";.*$|(?<![:\\w])\\/\\/.*$",
		"block": "\\/\\*[\\s\\S]*?\\*\\/"
	},
	"hs": {
		"singleLine": "(?<![:\\-])--.*$",
		"block": "\\{-[\\s\\S]*?-\\}"
	},
	"elm": {
		"singleLine": "(?<![:\\-])--.*$",
		"block": "\\{-[\\s\\S]*?-\\}"
	},
	"clj": {
		"singleLine": ";.*$",
		"block": null
	},
	"scm": {
		"singleLine": ";.*$",
		"block": "#\\|[\\s\\S]*?\\|#"
	},
	"lisp": {
		"singleLine": ";.*$",
		"block": "#\\|[\\s\\S]*?\\|#"
	},
	"vb": {
		"singleLine": "'.*$",
		"block": null
	},
	"pas": {
		"singleLine": "(?<![:\\w])\\/\\/.*$",
		"block": "\\{[\\s\\S]*?\\}|\\(\\*[\\s\\S]*?\\*\\)"
	},
	"f90": {
		"singleLine": "!.*$",
		"block": null
	},
	"f": {
		"singleLine": "!.*$",
		"block": null
	}
};

function activate(context) {
	const disposableFile = vscode.commands.registerCommand('extension.removeCommentsFromFile', (uri) => {
		removeCommentsFromFile(uri.fsPath);
	});

	const disposableFolder = vscode.commands.registerCommand('extension.removeCommentsFromFolder', (uri) => {
		removeCommentsFromFolder(uri.fsPath);
	});

	// Add a new command for previewing comment removal
	const disposablePreviewFile = vscode.commands.registerCommand('extension.previewCommentsFromFile', (uri) => {
		previewCommentsFromFile(uri.fsPath);
	});

	context.subscriptions.push(disposableFile);
	context.subscriptions.push(disposableFolder);
	context.subscriptions.push(disposablePreviewFile);
}

function getCommentPatterns() {
    const userPatterns = vscode.workspace.getConfiguration('commentRemover').get('patterns') || {};
    return { ...defaultCommentPatterns, ...userPatterns };
}

// New function to handle code with awareness of string literals
function removeCommentsWithContextAwareness(code, patterns) {
    if (!patterns) {
        return code;
    }

    // Handle common languages like JS that have string literals
    // which might contain things that look like comments
    const isJSLike = patterns.singleLine && patterns.singleLine.includes("//");
    
    if (isJSLike) {
        return removeCommentsFromJSLikeLanguage(code, patterns);
    }
    
    // For other languages, use the regex approach but with improved patterns
    let result = code;
    
    if (patterns.singleLine) {
        result = result.replace(new RegExp(patterns.singleLine, 'gm'), '');
    }
    
    if (patterns.block) {
        result = result.replace(new RegExp(patterns.block, 'gs'), '');
    }
    
    return result;
}

// Special handling for JavaScript-like languages
function removeCommentsFromJSLikeLanguage(code, patterns) {
    let result = '';
    let inSingleQuote = false;
    let inDoubleQuote = false;
    let inRegex = false;
    let inBlockComment = false;
    let inLineComment = false;
    let escaped = false;
    
    for (let i = 0; i < code.length; i++) {
        // Handle escape sequences
        if (escaped) {
            escaped = false;
            if (!inLineComment && !inBlockComment) {
                result += code[i];
            }
            continue;
        }
        
        if (code[i] === '\\' && (inSingleQuote || inDoubleQuote || inRegex)) {
            escaped = true;
            result += code[i];
            continue;
        }
        
        // Check for end of line which ends line comments
        if (code[i] === '\n') {
            inLineComment = false;
            result += code[i];
            continue;
        }
        
        // Handle end of block comment
        if (inBlockComment && code[i] === '*' && code[i+1] === '/') {
            inBlockComment = false;
            i++; // Skip the closing slash
            continue;
        }
        
        // Skip if we're in any type of comment
        if (inLineComment || inBlockComment) {
            continue;
        }
        
        // Handle string literals
        if (code[i] === '"' && !inSingleQuote && !inRegex) {
            inDoubleQuote = !inDoubleQuote;
            result += code[i];
            continue;
        }
        
        if (code[i] === "'" && !inDoubleQuote && !inRegex) {
            inSingleQuote = !inSingleQuote;
            result += code[i];
            continue;
        }
        
        // Handle regex literals - basic detection (this could be improved)
        if (code[i] === '/' && !inSingleQuote && !inDoubleQuote && !inRegex) {
            // Check if this could be the start of a regex
            // This is a simplification; proper detection would need more context
            const prevChar = i > 0 ? code[i-1] : '';
            if (/[({=:,;!&|?*+\-~^%]/.test(prevChar) || prevChar === ' ' || prevChar === '\t' || prevChar === '\n') {
                // Check if it's not a comment
                if (code[i+1] !== '/' && code[i+1] !== '*') {
                    inRegex = true;
                    result += code[i];
                    continue;
                }
            }
        }
        
        if (code[i] === '/' && inRegex) {
            inRegex = false;
            result += code[i];
            continue;
        }
        
        // Check for start of comments, but only if we're not in a string or regex
        if (!inSingleQuote && !inDoubleQuote && !inRegex) {
            // Check for line comment
            if (code[i] === '/' && code[i+1] === '/') {
                // Make sure it's not part of a URL by looking at previous character
                const prevChar = i > 0 ? code[i-1] : '';
                if (prevChar !== ':' && !/[a-zA-Z0-9]/.test(prevChar)) {
                    inLineComment = true;
                    i++; // Skip the second slash
                    continue;
                }
            }
            
            // Check for block comment
            if (code[i] === '/' && code[i+1] === '*') {
                inBlockComment = true;
                i++; // Skip the asterisk
                continue;
            }
        }
        
        // Add character to result if not in a comment
        if (!inLineComment && !inBlockComment) {
            result += code[i];
        }
    }
    
    return result;
}

function removeCommentsFromFile(filePath) {
    try {
        const stats = fs.statSync(filePath);
        if (!stats.isFile()) {
            vscode.window.showErrorMessage(`This is not a file, or file does not exist: ${filePath}`);
            return;
        }
    } catch (err) {
        vscode.window.showErrorMessage(`Error checking file: ${err.message}`);
        return;
    }

    fs.readFile(filePath, 'utf8', (err, data) => {
        if (err) {
            vscode.window.showErrorMessage(`Error reading file: ${err.message}`);
            return;
        }
        console.log(`Processing file: ${filePath}`);

        const fileExtension = path.extname(filePath).substring(1);
        const patterns = getCommentPatterns()[fileExtension];

        if (!patterns) {
            vscode.window.showErrorMessage(`Unsupported file extension: ${fileExtension}`);
            return;
        }

        const uncommentedText = removeCommentsWithContextAwareness(data, patterns);

        fs.writeFile(filePath, uncommentedText, 'utf8', (err) => {
            if (err) {
                vscode.window.showErrorMessage(`Error writing file: ${err.message}`);
                return;
            }
            vscode.window.showInformationMessage(`Comments removed from file: ${filePath}`);
        });
    });
}

// New function to preview comment removal
function previewCommentsFromFile(filePath) {
    try {
        const stats = fs.statSync(filePath);
        if (!stats.isFile()) {
            vscode.window.showErrorMessage(`This is not a file, or file does not exist: ${filePath}`);
            return;
        }
    } catch (err) {
        vscode.window.showErrorMessage(`Error checking file: ${err.message}`);
        return;
    }

    fs.readFile(filePath, 'utf8', (err, data) => {
        if (err) {
            vscode.window.showErrorMessage(`Error reading file: ${err.message}`);
            return;
        }

        const fileExtension = path.extname(filePath).substring(1);
        const patterns = getCommentPatterns()[fileExtension];

        if (!patterns) {
            vscode.window.showErrorMessage(`Unsupported file extension: ${fileExtension}`);
            return;
        }

        const uncommentedText = removeCommentsWithContextAwareness(data, patterns);
        
        // Create a new document to preview the changes
        vscode.workspace.openTextDocument({ content: uncommentedText, language: fileExtension })
            .then(doc => {
                vscode.window.showTextDocument(doc, { preview: true, viewColumn: vscode.ViewColumn.Beside });
                vscode.window.showInformationMessage('Comment removal preview. Save this file if you want to keep the changes.');
            });
    });
}

function removeCommentsFromFolder(folderPath) {
    fs.readdir(folderPath, (err, files) => {
        if (err) {
            vscode.window.showErrorMessage(`Error reading folder: ${err.message}`);
            return;
        }

        const totalFiles = files.length;
        let processedCount = 0;
        
        vscode.window.showInformationMessage(`Found ${totalFiles} files/folders to process.`);
        
        files.forEach((file) => {
            const filePath = path.join(folderPath, file);
            
            try {
                const stats = fs.statSync(filePath);
                if (stats.isFile()) {
                    const fileExt = path.extname(filePath).substring(1);
                    // Only process files with supported extensions
                    if (getCommentPatterns()[fileExt]) {
                        removeCommentsFromFile(filePath);
                    }
                    processedCount++;
                } else if (stats.isDirectory()) {
                    // Ask user if they want to process subdirectories
                    vscode.window.showInformationMessage(
                        `Process subdirectory ${file}?`,
                        'Yes', 'No'
                    ).then(answer => {
                        if (answer === 'Yes') {
                            removeCommentsFromFolder(filePath);
                        }
                        processedCount++;
                        if (processedCount === totalFiles) {
                            vscode.window.showInformationMessage(`Folder processing complete.`);
                        }
                    });
                    return; // Skip incrementing processedCount since it's done in the callback
                }
            } catch (err) {
                vscode.window.showErrorMessage(`Error processing ${filePath}: ${err.message}`);
                processedCount++;
            }
            
            if (processedCount === totalFiles) {
                vscode.window.showInformationMessage(`Folder processing complete.`);
            }
        });
    });
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};