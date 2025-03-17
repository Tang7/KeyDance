# Key Dance App

A web application that can automatically detect songs and show their piano staff notation.

## Setup

1. Install Rust and Cargo: <https://www.rust-lang.org/tools/install>
2. Install Node.js and npm: <https://nodejs.org/>
3. Install TypeScript: `npm install -g typescript`
4. Clone this repository
5. Build and run the application: `cargo run`

## Development

The application runs in HTTP mode for development at <http://127.0.0.1:8080>

- Frontend code is in TypeScript in the `static` directory
- Backend code is in Rust in the `src` directory

## Features

- Record audio from the browser
- Recognize songs from the recorded audio
- Display piano staff notation for recognized songs
