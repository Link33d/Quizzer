# Quizzer
Quizzer is a quiz program designed for desktop, tested primarily on Windows.

## TODO
- **Add**: Option to load a `.json` file from the device.
- **Fix**: Improve the program's CSS styling.

## How to Use
Before compiling or running the program, ensure you have **Rust** and **Node.js** installed.

### Check Requirements
Verify if the required tools are installed:
```sh
node -v
cargo -V
```
- If the commands above return their respective versions, the tools are installed correctly.

### Install the Dependencies
Navigate to the project directory and run:
1. Install Node.js dependencies:
```sh
npm install
```
2. Build the Rust backend:
```sh
cargo build
```

### Run the Application
To start the application in development mode:
```sh
npm run tauri dev
```

### Build the Application for Release
If you want to create a production-ready build:
```sh
npm run tauri build
```

## Observation
I'm still learning **Rust**, **Tauri**, **TypeScript**, and **React**, so any corrections or pull requests are welcome as they will support my growth in these technologies!
