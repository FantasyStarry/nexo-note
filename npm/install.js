const https = require('https');
const fs = require('fs');
const path = require('path');
const { platform, arch } = process;

const packageJson = require('./package.json');
const version = packageJson.version;
const repo = 'FantasyStarry/nexo-note';

const assets = {
  'win32-x64': 'nexo-windows-x86_64.exe',
  'linux-x64': 'nexo-linux-x86_64',
  'darwin-x64': 'nexo-macos-x86_64',
  'darwin-arm64': 'nexo-macos-aarch64',
};

const key = `${platform}-${arch}`;
const assetName = assets[key];

if (!assetName) {
  console.error(`Unsupported platform: ${platform} ${arch}`);
  console.error('Supported platforms: ' + Object.keys(assets).join(', '));
  process.exit(1);
}

const binDir = path.join(__dirname, 'bin');
const binaryName = platform === 'win32' ? 'nexo.exe' : 'nexo';
const outputPath = path.join(binDir, binaryName);

if (fs.existsSync(outputPath)) {
  console.log(`nexo binary already exists at ${outputPath}`);
  process.exit(0);
}

const url = `https://github.com/${repo}/releases/download/v${version}/${assetName}`;

console.log(`Downloading nexo v${version} for ${platform}-${arch}...`);
console.log(`URL: ${url}`);

fs.mkdirSync(binDir, { recursive: true });

const file = fs.createWriteStream(outputPath);

https
  .get(url, (response) => {
    if (response.statusCode === 302 || response.statusCode === 301) {
      https
        .get(response.headers.location, (redirectResponse) => {
          redirectResponse.pipe(file);
          file.on('finish', () => {
            file.close();
            if (platform !== 'win32') {
              fs.chmodSync(outputPath, 0o755);
            }
            console.log(`nexo installed at ${outputPath}`);
          });
        })
        .on('error', (err) => {
          fs.unlinkSync(outputPath);
          console.error(`Download failed: ${err.message}`);
          process.exit(1);
        });
    } else if (response.statusCode === 200) {
      response.pipe(file);
      file.on('finish', () => {
        file.close();
        if (platform !== 'win32') {
          fs.chmodSync(outputPath, 0o755);
        }
        console.log(`nexo installed at ${outputPath}`);
      });
    } else {
      console.error(`Download failed with status ${response.statusCode}`);
      process.exit(1);
    }
  })
  .on('error', (err) => {
    fs.unlinkSync(outputPath);
    console.error(`Download failed: ${err.message}`);
    process.exit(1);
  });
