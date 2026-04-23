# LibSoft — Library Management System

A modern desktop application for library management built with **Tauri**, **Rust**, **React**, **Vite**, and **Tailwind CSS**.

## Prerequisites

Before you can run or build this project, you need to install the following dependencies on your system:

1. **Node.js** (v18 or newer)
2. **Rust** (Install via [rustup](https://rustup.rs/))
3. **Tauri CLI Dependencies** (Follow the official Tauri [Windows prerequisite guide](https://tauri.app/v1/guides/getting-started/prerequisites#windows), including the MSVC C++ Build Tools).

## Project Structure

- `src/`: The React frontend codebase (UI components, styling, routing).
- `src-tauri/`: The Rust backend codebase (Database handling, IPC commands, OS interactions).

## Getting Started

### 1. Install Dependencies

First, install the frontend Node modules:

```bash
npm install
```

### 2. Development Mode

To run the application in development mode with Hot-Module Replacement (HMR):

```bash
cargo tauri dev
```
*(Alternatively, you can run `npm run dev` depending on your setup, but `cargo tauri dev` is recommended for full stack development).*

This will compile the Rust backend and launch the frontend on a local Vite server. Any changes you make to `src/` (React) or `src-tauri/src/` (Rust) will automatically trigger a rebuild or hot-reload.

### 3. Building for Production

To compile the application into a standalone native executable for distribution:

```bash
cargo tauri build
```
Once completed, you'll find the generated installer (like an `.exe` or `.msi` file on Windows) inside `src-tauri/target/release/bundle/`.

## Notes on the Backend

The Rust backend handles local SQLite database operations. Database files are typically stored in the user's AppData directory (e.g., `%APPDATA%/com.libsoft.app/libsoft.db`) depending on your Tauri configuration, ensuring data persists across updates.
