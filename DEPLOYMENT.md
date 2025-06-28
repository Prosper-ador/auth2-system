# 🚀 Deployment Guide

This guide will help you deploy your fullstack authentication system to production.

## 📋 Prerequisites

- GitHub account
- Railway account (for backend) - [Sign up here](https://railway.app/)
- Vercel account (for frontend) - [Sign up here](https://vercel.com/)

## 🔧 Backend Deployment (Railway)

### Step 1: Prepare Your Repository

1. **Push your code to GitHub:**
   ```bash
   git add .
   git commit -m "Prepare for deployment"
   git push origin main
   ```

2. **Ensure your backend directory structure is correct:**
   ```
   backend/
   ├── src/
   ├── Cargo.toml
   ├── railway.json
   ├── nixpacks.toml
   └── render.yaml
   ```

### Step 2: Deploy to Railway

1. **Go to [Railway Dashboard](https://railway.app/dashboard)**

2. **Click "New Project" → "Deploy from GitHub repo"**

3. **Select your repository**

4. **Configure the deployment:**
   - **Root Directory:** `backend`
   - **Environment Variables:**
     ```
     JWT_SECRET=your-super-secret-jwt-key-here
     JWT_SALT=your-super-secret-salt-here
     JWT_EXPIRATION_SECS=86400
     PORT=3000
     ```

5. **Deploy!** Railway will automatically build and deploy your Rust application.

6. **Get your deployment URL** (e.g., `https://your-app.railway.app`)

### Alternative: Deploy to Render

1. **Go to [Render Dashboard](https://dashboard.render.com/)**

2. **Click "New" → "Web Service"**

3. **Connect your GitHub repository**

4. **Configure:**
   - **Name:** `auth-api`
   - **Root Directory:** `backend`
   - **Environment:** `Rust`
   - **Build Command:** `cargo build --release`
   - **Start Command:** `./target/release/auth_api`

5. **Add Environment Variables:**
   ```
   JWT_SECRET=your-super-secret-jwt-key-here
   JWT_SALT=your-super-secret-salt-here
   JWT_EXPIRATION_SECS=86400
   ```

## 🎨 Frontend Deployment (Vercel)

### Step 1: Update Frontend Configuration

1. **Update the production environment file:**
   ```bash
   cd frontend
   # Edit env.production with your backend URL
   echo "VITE_API_BASE_URL=https://your-backend-url.railway.app" > env.production
   ```

2. **Test locally with production URL:**
   ```bash
   npm run build
   npm run preview
   ```

### Step 2: Deploy to Vercel

1. **Go to [Vercel Dashboard](https://vercel.com/dashboard)**

2. **Click "New Project"**

3. **Import your GitHub repository**

4. **Configure the deployment:**
   - **Framework Preset:** `Vite`
   - **Root Directory:** `frontend`
   - **Build Command:** `npm run build`
   - **Output Directory:** `dist`
   - **Install Command:** `npm install`

5. **Add Environment Variables:**
   ```
   VITE_API_BASE_URL=https://your-backend-url.railway.app
   ```

6. **Deploy!**

## 🔄 Update TypeScript Client

After deploying the backend, regenerate the TypeScript client:

1. **Update the generate-client script with your production URL:**
   ```bash
   # Edit generate-client.sh
   # Change the URL to your production backend URL
   ```

2. **Regenerate the client:**
   ```bash
   ./generate-client.sh
   ```

3. **Commit and push the updated client:**
   ```bash
   git add ts-client/
   git commit -m "Update TypeScript client for production"
   git push origin main
   ```

4. **Redeploy the frontend** (Vercel will automatically redeploy)

## 🧪 Testing Your Deployment

### Backend Testing

1. **Health Check:**
   ```bash
   curl https://your-backend-url.railway.app/health
   ```

2. **API Documentation:**
   - Visit: `https://your-backend-url.railway.app/swagger-ui`

3. **Test Registration:**
   ```bash
   curl -X POST https://your-backend-url.railway.app/register \
     -H "Content-Type: application/json" \
     -d '{
       "first_name": "Test",
       "last_name": "User",
       "email": "test@example.com",
       "password": "password123",
       "confirm_password": "password123"
     }'
   ```

### Frontend Testing

1. **Visit your Vercel URL**
2. **Test registration and login flows**
3. **Verify protected routes work**

## 🔒 Security Checklist

- [ ] JWT secrets are strong and unique
- [ ] Environment variables are set in production
- [ ] CORS is properly configured
- [ ] HTTPS is enabled (automatic with Railway/Vercel)
- [ ] No sensitive data in client-side code

## 📊 Monitoring

### Railway Monitoring
- View logs in Railway dashboard
- Set up alerts for errors
- Monitor resource usage

### Vercel Monitoring
- View deployment logs
- Monitor performance
- Set up error tracking

## 🔄 Continuous Deployment

Both Railway and Vercel will automatically redeploy when you push to your main branch.

## 🆘 Troubleshooting

### Common Issues

1. **Backend won't start:**
   - Check environment variables
   - Verify port configuration
   - Check build logs

2. **Frontend can't connect to backend:**
   - Verify CORS settings
   - Check API base URL
   - Ensure backend is running

3. **TypeScript client errors:**
   - Regenerate client after backend changes
   - Check OpenAPI spec is accessible

### Getting Help

- Check deployment logs in Railway/Vercel dashboards
- Verify environment variables are set correctly
- Test endpoints individually with curl/Postman

## 🎉 Success!

Once deployed, your authentication system will be accessible to everyone at:
- **Frontend:** `https://your-app.vercel.app`
- **Backend:** `https://your-app.railway.app`
- **API Docs:** `https://your-app.railway.app/swagger-ui` 