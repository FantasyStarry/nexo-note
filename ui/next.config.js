/** @type {import('next').NextConfig} */
const nextConfig = {
  // Dev proxy: forward /api/* to Rust backend
  async rewrites() {
    return [
      { source: '/api/:path*', destination: 'http://localhost:3456/api/:path*' },
    ];
  },
};

module.exports = nextConfig;
