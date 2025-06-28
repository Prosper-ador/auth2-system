#!/bin/bash

# Build script for Railway deployment

set -e

echo "🔨 Building backend for Railway..."

# Navigate to backend directory
cd backend

# Build the application
echo "📦 Building Rust application..."
cargo build --release

# Copy the binary to a known location
echo "📋 Copying binary..."
cp target/release/auth_api ./auth_api

echo "✅ Build completed successfully!" 