# **Projects & Roadmap**

- *Yes, this was made in collaboration with ChatGPT*

---

## **1. Bookmark Sorting Browser Extension**  

- **Goal**: Build a browser extension that organizes bookmarks by tags or categories, with options for sorting and filtering.  
- **What to Use**: JavaScript (for browser extension compatibility) or Go (for backend services if needed).  
- **What to Do**:  
  - Create the basic extension interface.  
  - Implement sorting and filtering logic.  
  - Add storage for user preferences.

---

## **2. VS Code Comment Remover Extension**  

- **Goal**: Create an extension for VS Code that removes or toggles comments from code files.  
- **What to Use**: TypeScript (for extension logic) and JavaScript (if interacting with the webview).  
- **What to Do**:  
  - Develop the extension structure.  
  - Use VS Code API to manipulate code files.  
  - Handle multiple programming languages for comment styles.

---

## **3. Markdown to HTML Converter**  

- **Goal**: Convert markdown documents to HTML using a simple command-line tool.  
- **What to Use**: Go or D (due to simplicity and strong string manipulation capabilities).  
- **What to Do**:  
  - Parse markdown syntax (headers, lists, links).  
  - Convert markdown elements to appropriate HTML tags.  
  - Support for optional features like table of contents.

---

## **4. Procedurally Generated Roguelike**  

- **Goal**: Develop a simple roguelike game with procedurally generated maps, enemies, and loot.  
- **What to Use**: Go (for performance and simplicity).  
- **What to Do**:  
  - Create a random dungeon generator.  
  - Implement basic game mechanics (combat, inventory).  
  - Add procedurally generated content like enemies and loot.

---

## **5. Screen Time Tracker**  

- **Goal**: Track screen time for users, including statistics for different applications.  
- **What to Use**: Go (for system monitoring) or V (if using a minimalistic approach).  
- **What to Do**:  
  - Track active applications and their run times.  
  - Display data in a user-friendly format.  
  - Optionally, include break reminders and time limits.

---

## **6. Custom VPN**  

- **Goal**: Build a simple VPN server and client.  
- **What to Use**: Go (for networking features) or Nim (for low-level control over networking).  
- **What to Do**:  
  - Implement VPN tunneling protocol (e.g., OpenVPN, WireGuard).  
  - Set up secure communication between the client and server.  
  - Support encryption and data routing.

---

## **7. Peer-to-Peer File Sharing Program**  

- **Goal**: Develop a P2P file-sharing application without relying on a server.  
- **What to Use**: Go (for networking) or Nim (for low-level control).  
- **What to Do**:  
  - Implement file transfer logic.  
  - Set up peer discovery using a decentralized protocol.  
  - Handle file encryption and compression.

---

## **8. CSV Processor**  

- **Goal**: Build a tool to filter, sort, and summarize CSV files, with support for command-line arguments.  
- **What to Use**: D (due to powerful file handling) or Go (due to simplicity in file I/O and argument parsing).  
- **What to Do**:  
  - Implement functions for parsing CSV files.  
  - Add filtering and sorting capabilities.  
  - Provide a summary or statistics from the CSV data.

---

## **9. Image Metadata Viewer**  

- **Goal**: Extract and display metadata from image files (EXIF data, resolution, timestamp, etc.).  
- **What to Use**: Go (easy integration with libraries for image processing) or D (due to system-level control).  
- **What to Do**:  
  - Parse image files to retrieve EXIF metadata.  
  - Display the extracted metadata in a clean format.  
  - Support multiple image formats (JPEG, PNG, TIFF).

---

## **10. HTTP Request Inspector**  

- **Goal**: A tool to send GET/POST requests and display the server responses, useful for API testing.  
- **What to Use**: Go (due to built-in HTTP support).  
- **What to Do**:  
  - Implement GET and POST request methods.  
  - Allow users to customize headers and payloads.  
  - Display responses and handle errors gracefully.

---

## **11. Static Site Generator**  

- **Goal**: Convert a folder of Markdown files into a complete HTML website.  
- **What to Use**: D (for fast processing) or Go (for performance with web server integration).  
- **What to Do**:  
  - Parse Markdown files and convert them into HTML.  
  - Generate static pages, including an index and template support.  
  - Handle links, images, and optional metadata.

---

## **12. Custom Shell**  

- **Goal**: Create a simple custom shell for executing commands, piping input/output, and managing processes.  
- **What to Use**: Go (for simplicity and performance) or Nim (for low-level system access).  
- **What to Do**:  
  - Implement basic shell features (command execution, arguments, piping).  
  - Manage background tasks and job control.  
  - Provide custom commands and error handling.

## **13. Desktop Note-Taking Application**  

- **Goal**: Build a desktop application for creating, organizing, and searching notes.  
- **What to Use**: Go (for simplicity and GUI libraries), or D (if using bindings for a GUI toolkit).  
- **What to Do**:  
  - Develop a basic GUI with support for creating, editing, and deleting notes.  
  - Implement features like search, categorization, and persistent storage (e.g., local file storage or a simple database).  
  - Add optional features like encryption for sensitive notes or synchronization across multiple devices.

---

### **Roadmap (From Easy & Fast to Hard & Slow)**

1. **VS Code Comment Remover Extension**: Fast to implement with basic file manipulation.
2. **Bookmark Sorting Browser Extension**: Simple extension with UI, fast and easy.
3. **Markdown to HTML Converter**: Basic parsing and conversion.
4. **CSV Processor**: Requires parsing and handling of data structures.
5. **Screen Time Tracker**: Simple system monitoring with file output.
6. **Desktop Note-Taking Application**: GUI development and data persistence.
7. **HTTP Request Inspector**: Basic networking and API interaction.
8. **Static Site Generator**: Requires parsing and basic template handling.
9. **Peer-to-Peer File Sharing Program**: Networking complexity increases.
10. **Image Metadata Viewer**: File parsing and handling, moderate difficulty.
11. **Custom Shell**: Involves system-level programming and process management.
12. **Custom VPN**: Networking and encryption complexities.
13. **Procedurally Generated Roguelike**: Full game development with random generation.
