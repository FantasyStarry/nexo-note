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

// Follow redirects (including nested ones) until we reach the final file.
function download(currentUrl, redirectsLeft) {
  if (redirectsLeft <= 0) {
    fail(`Too many redirects while downloading from ${url}`);
    return;
  }
  https
    .get(currentUrl, (response) => {
      const status = response.statusCode;
      if (status === 301 || status === 302 || status === 307 || status === 308) {
        const location = response.headers.location;
        if (!location) {
          fail(`Redirect (${status}) with no Location header from ${currentUrl}`);
          return;
        }
        response.resume(); // discard the redirect response body
        const next = location.startsWith('http')
          ? location
          : new URL(location, currentUrl).toString();
        download(next, redirectsLeft - 1);
        return;
      }
      if (status !== 200) {
        fail(`Download failed with HTTP ${status} for ${currentUrl}`);
        return;
      }
      const file = fs.createWriteStream(outputPath);
      response.pipe(file);
      file.on('finish', () => {
        file.close(() => {
          if (!validateBinary(outputPath)) {
            fail(
              `Downloaded file at ${outputPath} is not a valid nexo binary ` +
                `(it may be an HTML error page from a missing release asset). ` +
                `Expected release asset "${assetName}" for v${version}. ` +
                `Check https://github.com/${repo}/releases/tag/v${version} or run ` +
                '`npm install -g nexo-note` again after the release is published.'
            );
            return;
          }
          if (platform !== 'win32') {
            fs.chmodSync(outputPath, 0o755);
          }
          console.log(`nexo installed at ${outputPath}`);
        });
      });
    })
    .on('error', (err) => {
      fs.existsSync(outputPath) && fs.unlinkSync(outputPath);
      fail(`Download failed: ${err.message}`);
    });
}

// Ensure the downloaded file is a real binary and not an HTML error page.
function validateBinary(p) {
  try {
    const stat = fs.statSync(p);
    if (!stat.isFile() || stat.size < 1024 * 1024) {
      return false;
    }
    const fd = fs.openSync(p, 'r');
    const buf = Buffer.alloc(512);
    fs.readSync(fd, buf, 0, 512, 0);
    fs.closeSync(fd);
    const head = buf.toString('latin1').toLowerCase();
    // Reject obvious HTML error pages returned instead of the binary.
    return !head.includes('<!doctype') && !head.includes('<html');
  } catch (e) {
    return false;
  }
}

function fail(message) {
  if (fs.existsSync(outputPath)) {
    fs.unlinkSync(outputPath);
  }
  console.error('');
  console.error('nexo-note postinstall failed:');
  console.error(message);
  console.error('');
  console.error('You can still use nexo-note by downloading the binary manually from:');
  console.error(`https://github.com/${repo}/releases/tag/v${version}`);
  process.exit(1);
}

download(url, 10);
