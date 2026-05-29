/** @type {import('next').NextConfig} */
const nextConfig = {
    // output: 'export',
    distDir: '_static',
    // Allow dev resources (incl. /_next/webpack-hmr) to be requested when the
    // page is loaded from these hosts. Without this, Next 16 blocks the HMR
    // channel cross-origin and the browser never receives hot updates.
    allowedDevOrigins: ['127.0.0.1'],
    images: {
        unoptimized: true
    },
    trailingSlash: true,
    basePath: ''
}

module.exports = nextConfig
