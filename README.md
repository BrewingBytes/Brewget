# Full Stack Application

This project consists of a Vue 3 frontend with PrimeVue components and a Rust Axum backend.

## Project Structure
- `frontend/`: Vue 3 + PrimeVue application
- `backend/`: Rust Axum server

## Backend Setup
1. Navigate to the backend directory:
```bash
cd backend
```

2. Run the server:
```bash
cargo run
```
The server will start on http://localhost:3000

## Frontend Setup
1. Navigate to the frontend directory:
```bash
cd frontend
```

2. Install dependencies:
```bash
npm install
```

3. Run the development server:
```bash
npm run dev
```
The frontend will be available at http://localhost:5173

## Features
- Vue 3 with Composition API
- PrimeVue UI components
- Rust Axum backend with health check endpoint
- CORS configured for frontend-backend communication
- Proxy configuration for development