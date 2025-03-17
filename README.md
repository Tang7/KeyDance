# Key Dance App

A web application that can automatically detect songs and show their piano staff notation.

## Setup

1. Install Rust and Cargo: <https://www.rust-lang.org/tools/install>
2. Install Node.js and npm: <https://nodejs.org/>
3. Install TypeScript: `npm install -g typescript`
4. Sign up for a free ACRCloud account at <https://www.acrcloud.com/>
5. Create a project in ACRCloud and get your access key and access secret
6. Update the credentials in `src/recognition.rs`
7. Clone this repository
8. Build and run the application: `cargo run`

## Development

The application runs in HTTP mode for development at <http://127.0.0.1:8080>

- Frontend code is in TypeScript in the `static` directory
- Backend code is in Rust in the `src` directory

## Features

- Record audio from the browser
- Recognize songs from the recorded audio using ACRCloud
- Display piano staff notation for recognized songs

## API Usage

This application uses ACRCloud for song recognition, which provides:

- 100 free recognitions per day
- Accurate song identification with confidence scores
- A database of millions of songs
