#!/bin/bash

# Build script for Railway deployment

set -e

echo "🔨 Building backend for Railway..."

# Ensure we have the correct Rust version
echo "🦀 Setting up Rust toolchain..."
rustup default stable
rustup show

# Navigate to backend directory
cd backend

# Clean any previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Build the application
echo "📦 Building Rust application..."
cargo build --release

# Copy the binary to a known location
echo "📋 Copying binary..."
cp target/release/auth_api ./auth_api

echo "✅ Build completed successfully!" 