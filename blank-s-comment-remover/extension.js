const vscode = require('vscode');
const fs = require('fs');
const path = require('path');

const defaultCommentPatterns = {
	"js": {
		"singleLine": "//.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"ts": {
		"singleLine": "//.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"py": {
		"singleLine": "#.*$",
		"block": null
	},
	"java": {
		"singleLine": "//.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"c": {
		"singleLine": "//.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"cpp": {
		"singleLine": "//.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"html": {
		"singleLine": "<!--.*?-->",
		"block": "<!--[\\s\\S]*?-->"
	},
	"css": {
		"singleLine": null,
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"php": {
		"singleLine": "//.*$|#.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
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
		"singleLine": "#.*$",
		"block": null
	},
	"toml": {
		"singleLine": "#.*$",
		"block": null
	},
	"rb": {
		"singleLine": "#.*$",
		"block": null
	},
	"swift": {
		"singleLine": "//.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"go": {
		"singleLine": "//.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"rs": {
		"singleLine": "//.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"sql": {
		"singleLine": "--.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"sh": {
		"singleLine": "#.*$",
		"block": null
	},
	"bat": {
		"singleLine": "REM.*$",
		"block": null
	},
	"ps1": {  // PowerShell
		"singleLine": "#.*$",
		"block": null
	},
	"pl": {  // Perl
		"singleLine": "#.*$",
		"block": null
	},
	"lua": {
		"singleLine": "--.*$",
		"block": "--\\[\\[[\\s\\S]*?\\]\\]"
	},
	"kt": {  // Kotlin
		"singleLine": "//.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"scala": {
		"singleLine": "//.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"dart": {
		"singleLine": "//.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"r": {
		"singleLine": "#.*$",
		"block": null
	},
	"m": {  // MATLAB
		"singleLine": "%.*$",
		"block": null
	},
	"asm": {  // Assembly
		"singleLine": ";.*$",
		"block": null
	},
	"hs": {  // Haskell
		"singleLine": "--.*$",
		"block": "{-[\\s\\S]*?-}"
	},
	"elm": {
		"singleLine": "--.*$",
		"block": "{-[\\s\\S]*?-}"
	},
	"clj": {  // Clojure
		"singleLine": ";.*$",
		"block": null
	},
	"scm": {  // Scheme
		"singleLine": ";.*$",
		"block": null
	},
	"lisp": {
		"singleLine": ";.*$",
		"block": null
	},
	"vb": {
		"singleLine": "'.*$",
		"block": "/\\*[\\s\\S]*?\\*/"
	},
	"pas": {  // Pascal
		"singleLine": "//.*$",
		"block": "\\{[\\s\\S]*?\\}|\\(\\*[\\s\\S]*?\\*\\)"
	},
	"f90": {  // Fortran
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

    context.subscriptions.push(disposableFile);
    context.subscriptions.push(disposableFolder);

	context.subscriptions.push(
		vscode.commands.registerCommand('remove-comments.fromFile', function () {
			vscode.window.showInformationMessage('Remove Comments from File triggered!');
		}),
		vscode.commands.registerCommand('remove-comments.fromFolder', function () {
			vscode.window.showInformationMessage('Remove Comments from Folder triggered!');
		})
	);
	
}

function getCommentPatterns() {
    const userPatterns = vscode.workspace.getConfiguration('commentRemover').get('patterns') || {};
    return { ...defaultCommentPatterns, ...userPatterns };
}

function removeCommentsFromFile(filePath) {
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

        let uncommentedText = data;

        if (patterns.singleLine) {
            uncommentedText = uncommentedText.replace(new RegExp(patterns.singleLine, 'gm'), '');

        }

        if (patterns.block) {
			uncommentedText = uncommentedText.replace(new RegExp(patterns.block, 'gm'), '');
        }

		console.log(`Before: ${data}`);
		console.log(`After: ${uncommentedText}`);


        fs.writeFile(filePath, uncommentedText, 'utf8', (err) => {
            if (err) {
                vscode.window.showErrorMessage(`Error writing file: ${err.message}`);
                return;
            }
            vscode.window.showInformationMessage(`Comments removed from file: ${filePath}`);
        });
    });
}

function removeCommentsFromFolder(folderPath) {
    fs.readdir(folderPath, (err, files) => {
        if (err) {
            vscode.window.showErrorMessage(`Error reading folder: ${err.message}`);
            return;
        }

        files.forEach((file) => {
            const filePath = path.join(folderPath, file);
            try {
				const stats = fs.statSync(filePath);
				if (stats.isFile()) {
					removeCommentsFromFile(filePath);
				} else if (stats.isDirectory()) {
					removeCommentsFromFolder(filePath);
				}
			} catch (err) {
				vscode.window.showErrorMessage(`Error processing ${filePath}: ${err.message}`);
			}			
        });
    });
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};