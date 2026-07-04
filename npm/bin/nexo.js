#!/usr/bin/env node
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Find the package root directory
const packageRoot = path.resolve(__dirname, '..');
const binaryName = process.platform === 'win32' ? 'nexo.exe' : 'nexo';
const binaryPath = path.join(packageRoot, 'bin', binaryName);

if (!fs.existsSync(binaryPath)) {
  console.error(`Binary not found: ${binaryPath}`);
  console.error('Please reinstall: npm install -g nexo-note');
  process.exit(1);
}

const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  shell: false,
});

child.on('exit', (code) => {
  process.exit(code ?? 0);
});
