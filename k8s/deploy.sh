#!/bin/bash
# Deploy BrewGet to Kubernetes
# This script applies all Kubernetes manifests in the correct order

set -e

echo "🚀 Deploying BrewGet to Kubernetes..."
echo ""

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl not found. Please install kubectl first."
    exit 1
fi

# Start minikube tunnel if minikube is available
if command -v minikube &> /dev/null; then
    echo "🌐 Starting minikube tunnel..."
    sudo minikube tunnel --bind-address=0.0.0.0 &
    echo "✅ Minikube tunnel started in background"
    echo ""
fi

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Apply manifests in order
echo "📦 Creating namespace..."
kubectl apply -f "$SCRIPT_DIR/00-namespace.yaml"

echo "📝 Creating shared configuration..."
kubectl apply -f "$SCRIPT_DIR/01-shared-config.yaml"

echo "🔐 Creating secrets..."
kubectl apply -f "$SCRIPT_DIR/02-secrets.yaml"

echo "📝 Creating configmaps..."
kubectl apply -f "$SCRIPT_DIR/03-configmaps.yaml"

echo "🗄️  Deploying PostgreSQL..."
kubectl apply -f "$SCRIPT_DIR/04-postgres.yaml"

echo "⏳ Waiting for PostgreSQL PVC to be bound..."
kubectl wait --for=jsonpath='{.status.phase}'=Bound pvc/brewget-postgres-pvc -n brewget --timeout=60s || echo "⚠️  PVC binding may take some time, continuing..."

echo "📧 Deploying email service..."
kubectl apply -f "$SCRIPT_DIR/05-email-service.yaml"

echo "🔑 Deploying auth service..."
kubectl apply -f "$SCRIPT_DIR/06-auth-service.yaml"

echo "⚙️  Deploying settings service..."
kubectl apply -f "$SCRIPT_DIR/07-settings-service.yaml"

echo "🎨 Deploying frontend..."
kubectl apply -f "$SCRIPT_DIR/08-frontend.yaml"

echo "🌐 Deploying nginx..."
kubectl apply -f "$SCRIPT_DIR/09-nginx.yaml"

echo ""
echo "✅ All manifests applied successfully!"
echo ""
echo "⏳ Waiting for pods to be ready..."
kubectl wait --for=condition=ready pod --all -n brewget --timeout=300s || true

echo ""
echo "📊 Current status:"
kubectl get pods -n brewget

echo ""
echo "🌐 Services:"
kubectl get services -n brewget

echo ""
echo "✅ Deployment complete!"
echo ""
echo "To access the application:"
echo "  1. Get the nginx service external IP:"
echo "     kubectl get service nginx -n brewget"
echo ""
echo "  2. Or use port forwarding for local access:"
echo "     kubectl port-forward -n brewget service/nginx 8080:80"
echo "     Then access at http://localhost:8080"
echo ""
echo "To check logs:"
echo "  kubectl logs -n brewget <pod-name>"
echo ""
echo "To check all resources:"
echo "  kubectl get all -n brewget"
