# Secure Notes Project 🦀

A modular CLI (Command Line Interface) application built to master core Rust programming concepts, focusing on memory safety and data organization.

## 📖 Description

**The Goal:** To allow an authenticated user to create, read, and search secure notes using a persistent runtime storage system.

This project demonstrates my understanding of:

- **Structs & Data Modeling:** Grouping related data (Title + Content) into custom types.
- **Memory Ownership & Borrowing:** Passing mutable (`&mut`) and immutable (`&`) references between functions.
- **Vectors:** managing dynamic lists of complex objects.
- **Algorithms:** Implementing a case-insensitive linear search.
- **Refactoring:** Breaking a monolithic `main` function into clean, reusable components.

## 🛠️ How It Works

### 1. Data Structure (Structs)

Instead of using simple strings, I implemented a custom struct to model a real note. This allows the application to handle multiple pieces of data as a single unit.

```rust
struct Note {
    title: String,
    content: String,
}

```

### 2. Modular Architecture

The code has been refactored from a single script into specialized functions. The `main` function now acts as a clean "command center" that delegates tasks:

- `pass_authenticate()`: Checks whether a user can access the application or not.Handles security.
- `menu()`: Displays a list of choices for the authenticated user to choose.
- `add_note()`: Takes a **mutable reference** to the vector to push new data.
- `list_items()` & `search()`: Take **immutable references** to read data without modifying it.

### 3. Features

#### Authentication

The user must pass a password check to enter. The logic is encapsulated in a dedicated function that loops until success, keeping the main logic clean.

#### Note Management

- **Add Note:** Captures a Title and Content, packages them into a `Note` struct, and pushes it to the heap-allocated Vector.
- **List Notes:** Iterates through the stored structs and prints formatted output.

#### Search Functionality

I implemented a search algorithm that filters notes by keyword.

- **Case-Insensitive:** It converts both the stored title and the search query to lowercase so that "shop" finds "Shopping".
- **Borrowing Logic:** Uses references to avoid unnecessary memory cloning during the search.

#### Graceful Exit option

- Allows the user to close the application cleanly via the menu, rather than force-stopping the terminal (Ctrl+C).

## How to Run

To start the application, navigate to the project folder and run:

```bash
cargo run

```
