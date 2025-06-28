#!/bin/bash

# Development Script - Start Backend and Frontend

set -e

echo "🚀 Starting development environment..."

# Function to cleanup background processes
cleanup() {
    echo "🛑 Stopping development servers..."
    kill $BACKEND_PID $FRONTEND_PID 2>/dev/null || true
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Check if backend environment is set up
if [ ! -f backend/.env ]; then
    echo "⚠️  Backend .env file not found. Creating from example..."
    if [ -f backend/env.example ]; then
        cp backend/env.example backend/.env
        echo "✅ Created backend/.env file from example."
        echo "📝 Please edit backend/.env with your actual JWT secrets."
        echo "   Then run this script again."
        exit 1
    else
        echo "❌ backend/env.example file not found."
        exit 1
    fi
fi

# Start backend
echo "🔧 Starting backend server..."
cd backend
cargo run &
BACKEND_PID=$!
cd ..

# Wait for backend to start
echo "⏳ Waiting for backend to start..."
sleep 5

# Check if backend is healthy
if curl -f http://localhost:3000/health > /dev/null 2>&1; then
    echo "✅ Backend is running at http://localhost:3000"
else
    echo "❌ Backend failed to start. Check logs above."
    kill $BACKEND_PID 2>/dev/null || true
    exit 1
fi

# Start frontend
echo "🎨 Starting frontend development server..."
cd frontend
npm run dev &
FRONTEND_PID=$!
cd ..

echo "⏳ Waiting for frontend to start..."
sleep 3

echo "🎉 Development environment is ready!"
echo ""
echo "📱 Frontend: http://localhost:5173"
echo "🔧 Backend:  http://localhost:3000"
echo "📚 Swagger:  http://localhost:3000/swagger-ui"
echo ""
echo "Press Ctrl+C to stop all servers"

# Wait for user to stop
wait 