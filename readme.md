# Secure Notes Project 🦀

A basic CLI (Command Line Interface) application built to learn core Rust programming concepts.

## 📖 Description

**The Goal:** To allow an authenticated user to store secure notes, view the list of saved items, or exit the application.

This project demonstrates my understanding of:

- Shadowing variables
- Reading user input
- Parsing data types
- Vector usage
- Ownership and Borrowing

## 🛠️ How It Works

### 1. Authentication

I started by working with a basic user password setup. The user must enter the correct password to gain access.

- **Logic:** Uses `if-else` conditions to verify credentials.
- **Retry Mechanism:** If the user enters the wrong password, an infinite `loop` allows them to "Try again" without restarting the program.

### 2. The Main Application

Once authenticated, the user enters the main menu loop. Based on their choice, they can:

- **Add Note:** Implemented using **Vectors** to store data dynamically.
- **List Notes:** Iterates through the vector to display saved items.
- **Exit:** Breaks the loop to safely close the program.

## 🚀 How to Run

To start the application, navigate to the project folder and run:

```bash
cargo run
```
