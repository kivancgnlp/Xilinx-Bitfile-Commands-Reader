# Xilinx Bitfile Reader - Web Application

This is a browser-based version of the Xilinx Bitfile Reader, built with Rust and WebAssembly.

## Prerequisites

1. **Rust** - Install from [rustup.rs](https://rustup.rs/)
2. **wasm-pack** - Install with:
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```
   Or using cargo:
   ```bash
   cargo install wasm-pack
   ```

## Building the WebAssembly Module

1. Build the WASM package:
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```

   This will create a `pkg` directory containing the compiled WebAssembly module and JavaScript bindings.

## Running the Application

### Option 1: Using a Local Web Server

Since browsers require HTTP/HTTPS to load WebAssembly modules, you need to serve the files through a web server:

**Using Python 3:**
```bash
python3 -m http.server 8000
```

**Using Python 2:**
```bash
python -m SimpleHTTPServer 8000
```

**Using Node.js (http-server):**
```bash
npx http-server -p 8000
```

Then open your browser and navigate to:
```
http://localhost:8000/index.html
```

### Option 2: Using wasm-pack's built-in server

```bash
wasm-pack build --target web --out-dir pkg && python3 -m http.server 8000
```

## Usage

1. Open `index.html` in your browser (via a web server)
2. Click "Choose Bitfile to Upload" or drag and drop a `.bit` or `.bin` file
3. The decoded results will appear in the text area below
4. You can copy the results or clear them using the buttons

## Project Structure

- `src/lib.rs` - Main WASM-compatible library with `process_bitfile` function
- `index.html` - Web frontend with file upload and results display
- `pkg/` - Generated WebAssembly package (created after building)

## Development

To rebuild after making changes:

```bash
wasm-pack build --target web --out-dir pkg
```

Then refresh your browser to see the changes.

## Notes

- The application processes bitfiles entirely in the browser - no data is sent to any server
- Large files may take a moment to process
- The ID codes are embedded in the WASM module, so no external file access is needed

