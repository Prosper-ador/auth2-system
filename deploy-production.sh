#!/bin/bash

# Production Deployment Script

set -e

echo "🚀 Production Deployment Script"
echo "================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if git is clean
if [ -n "$(git status --porcelain)" ]; then
    print_error "Git working directory is not clean. Please commit your changes first."
    exit 1
fi

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    print_warning "You're not on the main branch. Current branch: $CURRENT_BRANCH"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

print_status "Starting production deployment..."

# Step 1: Test backend locally
print_status "Testing backend locally..."
cd backend

# Check if .env exists
if [ ! -f .env ]; then
    print_warning "Backend .env file not found. Creating from example..."
    if [ -f env.example ]; then
        cp env.example .env
        print_error "Please edit backend/.env with your production JWT secrets, then run this script again."
        exit 1
    else
        print_error "env.example file not found in backend directory."
        exit 1
    fi
fi

# Test build
print_status "Testing backend build..."
cargo build --release
print_success "Backend builds successfully"

cd ..

# Step 2: Test frontend build
print_status "Testing frontend build..."
cd frontend

# Check if production env file exists
if [ ! -f env.production ]; then
    print_warning "Frontend env.production file not found. Creating template..."
    echo "# Replace with your actual deployed backend URL" > env.production
    echo "VITE_API_BASE_URL=https://your-backend-url.railway.app" >> env.production
    print_error "Please edit frontend/env.production with your actual backend URL, then run this script again."
    exit 1
fi

# Test build
print_status "Testing frontend build..."
npm run build
print_success "Frontend builds successfully"

cd ..

# Step 3: Push to GitHub
print_status "Pushing to GitHub..."
git add .
git commit -m "Prepare for production deployment"
git push origin main
print_success "Code pushed to GitHub"

# Step 4: Instructions
echo
print_success "Local testing completed successfully!"
echo
echo "📋 Next Steps:"
echo "=============="
echo
echo "1. 🔧 Deploy Backend to Railway:"
echo "   - Go to https://railway.app/dashboard"
echo "   - Create new project from GitHub repo"
echo "   - Set root directory to 'backend'"
echo "   - Add environment variables:"
echo "     JWT_SECRET=your-production-secret"
echo "     JWT_SALT=your-production-salt"
echo "     JWT_EXPIRATION_SECS=86400"
echo
echo "2. 🎨 Deploy Frontend to Vercel:"
echo "   - Go to https://vercel.com/dashboard"
echo "   - Import your GitHub repository"
echo "   - Set root directory to 'frontend'"
echo "   - Add environment variable:"
echo "     VITE_API_BASE_URL=https://your-backend-url.railway.app"
echo
echo "3. 🔄 Update TypeScript Client:"
echo "   - After backend deployment, update generate-client.sh with your backend URL"
echo "   - Run: ./generate-client.sh"
echo "   - Commit and push the updated client"
echo
echo "📖 See DEPLOYMENT.md for detailed instructions"
echo
print_success "Deployment preparation complete!" 