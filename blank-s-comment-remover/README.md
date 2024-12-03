# Blank's Comment Remover

We all need a comment remover in this day and age if you know what I mean :P

- *Delete all comments in a file.*
- *Delete all comments in every file in a directory/folder.*

## Usage

1. Right-click on a file or folder in the Explorer view.
2. Select "Remove Comments from File" or "Remove Comments from Folder".
3. Review the changes to ensure correctness.

## 1.0.0

Release of this extension

Custom settings example:

```json
"commentRemover.patterns": {
  "customLang": {
    "singleLine": "//.*$",
    "block": "/\\*[\\s\\S]*?\\*/"
  }
}
```
