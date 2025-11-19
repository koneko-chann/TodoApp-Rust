# TodoApp – Rust + Yew Frontend

> Frontend for https://github.com/koneko-chann/TodoApp-Rust  
> Built with Yew, Tailwind CSS, and Trunk.

## Overview

This project is a modern single–page application that talks to the TodoApp-Rust backend.  
It provides login/register screens, a todo dashboard with inline editing, filters, counters, and full CRUD operations.  
The UI is written in Rust using Yew and compiled to WebAssembly.

## Features

- 🔐 **Auth-aware routing** with guarded login/register pages.
- ✅ **Todo management** (create, edit title, update status, delete).
- 📊 **Live counters** showing total/completed/pending tasks.
- 🎨 **Tailwind CSS** styling with responsive, dark-friendly design.
- 🔁 **Yew Router** for routing + redirects (Counter/Home → Todo).
- 🧰 **Auth context** storing tokens in `localStorage`.

## Project Structure

```
src/
├── components/        # Presentational widgets (e.g., Header)
├── context/           # Auth context + provider
├── screens/           # Page-level components (login, register, todo, etc.)
├── styles/            # Tailwind output
├── utils/             # Helpers
└── main.rs            # Router + app entry
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- `wasm32-unknown-unknown` target  
  ```sh
  rustup target add wasm32-unknown-unknown
  ```
- [Trunk](https://trunkrs.dev/#install) for serving/bundling  
  ```sh
  cargo install trunk
  ```
- Node.js + npm (only required if rebuilding Tailwind styles)

## Getting Started

Clone the frontend:

```sh
git clone https://github.com/koneko-chann/TodoApp-Rust.git
cd TodoApp-Rust
```

Install Rust dependencies:

```sh
cargo fetch
```

Install Node dependencies (optional – Tailwind rebuild):

```sh
npm install
```

### Run in Development

```sh
trunk serve --open
```

Trunk watches Rust + CSS changes, rebuilds the Wasm bundle, and opens the app in your browser.

### Backend API

Set the backend origin via the `BACKEND_URL` environment variable before running Trunk
(`BACKEND_URL=https://api.example.com trunk serve`) or place the value in a `.env`
file (`BACKEND_URL="https://api.example.com"`).  
If not provided, it falls back to `http://127.0.0.1:8080`, so with the default value the API
endpoints look like:

- `GET http://127.0.0.1:8080/api/v1/todo` for listing tasks.
- `POST/PUT/DELETE http://127.0.0.1:8080/api/v1/todo[...]` for CRUD operations.

Make sure the backend server (from the linked repository) is running so the UI can load data.

## Build for Production

```sh
trunk build --release
```

The optimized assets are emitted to `dist/`.

## Contributing

1. Fork the repo & create feature branch.  
2. Run `trunk fmt`/`cargo fmt` and ensure `trunk build` succeeds.  
3. Submit a pull request describing the change.

## License

Distributed under the MIT License. See `LICENSE` for details.

---

Made with ❤️ in Rust. Have fun hacking!
