# Blank's Comment Remover

A smart VS Code extension that intelligently removes comments from code files while preserving URLs, string literals, and other non-comment code.

- *Intelligently delete comments in a file while preserving URLs and code structure*
- *Delete all comments in every file in a directory/folder*
- *Preview comment removal before applying changes*

## Features

- **Context-Aware Comment Removal**: Understands string literals, regular expressions, and other contexts where comment-like syntax might appear legitimately
- **Multiple Language Support**: Handles comments in 30+ programming languages
- **Preview Mode**: Preview changes before applying them
- **Customizable Patterns**: Define your own comment patterns for custom languages

## Usage

1. Right-click on a file or folder in the Explorer view.
2. Select one of the following:
   - "Remove Comments from File" - Remove comments directly
   - "Preview Comment Removal" - See changes before applying them
   - "Remove Comments from Folder" - Process multiple files at once
3. For folder processing, you'll be prompted before processing each subfolder.

## What's New in 1.1.0

- Improved URL handling - no more broken links when removing comments!
- Added preview functionality to review changes before applying
- Context-aware parsing that understands string literals and regular expressions
- Better folder handling with confirmation for subfolders

## Custom Settings

Customize comment patterns for your own languages:

```json
"commentRemover.patterns": {
  "customLang": {
    "singleLine": "(?<![:\\w])\\/\\/.*$",
    "block": "\\/\\*[\\s\\S]*?\\*\\/"
  }
}
```

You can also toggle confirmation before processing subfolders:

```json
"commentRemover.confirmSubfolders": true
```

## Supported Languages

JavaScript, TypeScript, Python, Java, C, C++, HTML, CSS, PHP, JSON, XML, YAML, TOML, Ruby, Swift, Go, Rust, SQL, Shell scripts, PowerShell, Perl, Lua, Kotlin, Scala, Dart, R, MATLAB, Assembly, Haskell, Elm, Clojure, Scheme, Lisp, Visual Basic, Pascal, and Fortran.
