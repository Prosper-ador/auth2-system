#!/bin/bash

# Generate TypeScript Client Script

set -e

echo "🔄 Regenerating TypeScript client..."

# Check if backend is running
if ! curl -f http://localhost:3000/health > /dev/null 2>&1; then
    echo "❌ Backend is not running. Please start the backend first:"
    echo "   cd backend && cargo run"
    exit 1
fi

# Navigate to ts-client directory
cd ts-client

# Remove existing generated files
echo "🧹 Cleaning existing generated files..."
rm -rf api.ts base.ts common.ts configuration.ts docs/

# Generate new client
echo "🔨 Generating new TypeScript client..."
npx openapi-generator-cli generate \
    -i http://localhost:3000/api-docs/openapi.json \
    -g typescript-fetch \
    -o . \
    --additional-properties=supportsES6=true,npmName=@auth-api/client,npmVersion=1.0.0

echo "✅ TypeScript client generated successfully!"
echo "📁 Generated files:"
ls -la *.ts docs/

echo "🎉 Client generation complete!" 