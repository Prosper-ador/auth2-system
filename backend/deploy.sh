#!/bin/bash

# Auth API Backend Deployment Script

set -e

echo "🚀 Starting Auth API Backend Deployment..."

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

# Check if docker-compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

# Check if .env file exists
if [ ! -f .env ]; then
    echo "⚠️  .env file not found. Creating from example..."
    if [ -f env.example ]; then
        cp env.example .env
        echo "✅ Created .env file from example. Please update it with your actual values."
        echo "📝 Edit .env file and run this script again."
        exit 1
    else
        echo "❌ env.example file not found. Please create a .env file manually."
        exit 1
    fi
fi

# Build and start the application
echo "🔨 Building and starting the application..."
docker-compose up --build -d

# Wait for the application to start
echo "⏳ Waiting for the application to start..."
sleep 10

# Check if the application is healthy
echo "🏥 Checking application health..."
if curl -f http://localhost:3000/health > /dev/null 2>&1; then
    echo "✅ Application is healthy and running!"
    echo "🌐 API is available at: http://localhost:3000"
    echo "📚 Swagger UI is available at: http://localhost:3000/swagger-ui"
    echo "📋 OpenAPI spec is available at: http://localhost:3000/api-docs/openapi.json"
else
    echo "❌ Application health check failed. Check logs with: docker-compose logs"
    exit 1
fi

echo "🎉 Deployment completed successfully!" 