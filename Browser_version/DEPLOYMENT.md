# Deployment Guide for Static Hosting

For deploying to static hosting providers like **GitHub Pages**, **Netlify**, **Vercel**, or **Cloudflare Pages**, you only need the following files:

## Required Files for Static Hosting

After building the project, upload these files and directories:

```
├── index.html          ← Main HTML file (REQUIRED)
└── pkg/                ← Built WASM package (REQUIRED)
    ├── bitfile_reader_prototype_1_bg.wasm
    ├── bitfile_reader_prototype_1.js
    ├── bitfile_reader_prototype_1.d.ts
    └── package.json
```

## Step-by-Step Deployment

### 1. Build the WASM Module

First, build the WebAssembly package:

```bash
wasm-pack build --target web --out-dir pkg
```

This creates the `pkg/` directory with all necessary files.

### 2. Files to Upload

**Minimum files needed:**
- `index.html`
- `pkg/` directory (entire directory with all its contents)

**Optional files (for documentation):**
- `README_WASM.md` (if you want to include documentation)

**Files you DON'T need:**
- `src/` directory (Rust source code)
- `Cargo.toml` and `Cargo.lock`
- `Kaynak_data/` directory (test files)
- `target/` directory (build artifacts)
- `.gitignore`

### 3. GitHub Pages Deployment

1. Create a new repository on GitHub
2. Upload only `index.html` and the `pkg/` directory
3. Go to repository Settings → Pages
4. Select the branch and folder (usually `main` and `/root`)
5. Your site will be available at `https://yourusername.github.io/repository-name/`

**Note:** Make sure `index.html` is in the root of the repository or adjust the path in GitHub Pages settings.

### 4. Netlify Deployment

1. Drag and drop the folder containing `index.html` and `pkg/` directory
2. Or connect your GitHub repository and set build command to:
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```
3. Set publish directory to the root (where `index.html` is)

### 5. Vercel Deployment

1. Install Vercel CLI: `npm i -g vercel`
2. In your project directory, run: `vercel`
3. Or connect via GitHub and set build command:
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```

### 6. Cloudflare Pages

1. Connect your repository
2. Set build command: `wasm-pack build --target web --out-dir pkg`
3. Set build output directory: `/` (root)

## Quick Deployment Checklist

- [ ] Run `wasm-pack build --target web --out-dir pkg`
- [ ] Verify `pkg/` directory exists with `.wasm` and `.js` files
- [ ] Upload `index.html` and `pkg/` directory to your hosting provider
- [ ] Ensure `index.html` references `./pkg/bitfile_reader_prototype_1.js` correctly
- [ ] Test the deployed application

## File Structure for Deployment

Your deployment directory should look like this:

```
deployment/
├── index.html
└── pkg/
    ├── bitfile_reader_prototype_1_bg.wasm
    ├── bitfile_reader_prototype_1.js
    ├── bitfile_reader_prototype_1.d.ts
    └── package.json
```

That's it! Just these files are needed for the application to work.

