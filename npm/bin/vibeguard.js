#!/usr/bin/env node

const { spawnSync } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

// In a real production release, this script would check `os.platform()` 
// and `os.arch()` to download the correct binary from GitHub Releases (e.g., vibeguard-mac-arm64).
//
// For local development, we will just point directly to your locally compiled Rust binary!
const binaryName = os.platform() === 'win32' ? 'vibeguard.exe' : 'vibeguard';
const binaryPath = path.join(__dirname, '..', '..', 'target', 'release', binaryName);

// Check if the binary actually exists
if (!fs.existsSync(binaryPath)) {
    console.error(`❌ Error: VibeGuard binary not found at ${binaryPath}`);
    console.error("Please run 'cargo build --release' in the project root first.");
    process.exit(1);
}

// Extract the arguments the user passed (ignoring 'node' and 'vibeguard.js')
// e.g., if they typed: `vibeguard scan . --json`, args will be['scan', '.', '--json']
const args = process.argv.slice(2);

// Execute the Rust binary synchronously, passing the exact arguments.
// stdio: 'inherit' ensures that CLI colors, logs, and MCP JSON-RPC streams work perfectly.
const result = spawnSync(binaryPath, args, { 
    stdio: 'inherit' 
});

// Exit the Node script with the exact same exit code the Rust binary returned
process.exit(result.status || 0);