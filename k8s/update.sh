#!/bin/bash
# Update BrewGet Kubernetes deployments
# This script forces recreation of all pods with new versions, even if images haven't changed
#
# Note: kubectl rollout restart works on deployments/statefulsets, not on pods directly
# Correct: kubectl rollout restart deployment/<name>
# Incorrect: kubectl rollout restart pods

set -e

echo "🔄 Updating BrewGet deployments in Kubernetes..."
echo ""

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl not found. Please install kubectl first."
    exit 1
fi

# Check if namespace exists
if ! kubectl get namespace brewget &> /dev/null; then
    echo "❌ BrewGet namespace not found. Please deploy the application first using deploy.sh"
    exit 1
fi

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Apply updated manifests to ensure any configuration changes are picked up
echo "📝 Applying updated configurations..."
kubectl apply -f "$SCRIPT_DIR/01-shared-config.yaml"
kubectl apply -f "$SCRIPT_DIR/02-secrets.yaml"
kubectl apply -f "$SCRIPT_DIR/03-configmaps.yaml"

# Update PostgreSQL configuration (without restart if not needed)
echo "🗄️  Updating PostgreSQL configuration..."
kubectl apply -f "$SCRIPT_DIR/04-postgres.yaml"

# Force restart all service deployments to pick up new images/configs
echo "📧 Restarting email service..."
kubectl apply -f "$SCRIPT_DIR/05-email-service.yaml"
kubectl rollout restart deployment/email-service -n brewget

echo "🔑 Restarting auth service..."
kubectl apply -f "$SCRIPT_DIR/06-auth-service.yaml"
kubectl rollout restart deployment/auth-service -n brewget

echo "⚙️  Restarting settings service..."
kubectl apply -f "$SCRIPT_DIR/07-settings-service.yaml"
kubectl rollout restart deployment/settings-service -n brewget

echo "🎨 Restarting frontend..."
kubectl apply -f "$SCRIPT_DIR/08-frontend.yaml"
kubectl rollout restart deployment/frontend -n brewget

echo "🌐 Restarting nginx..."
kubectl apply -f "$SCRIPT_DIR/09-nginx.yaml"
kubectl rollout restart deployment/nginx -n brewget

echo ""
echo "⏳ Waiting for rollouts to complete..."

# Wait for each deployment to complete its rollout
kubectl rollout status deployment/email-service -n brewget --timeout=300s
kubectl rollout status deployment/auth-service -n brewget --timeout=300s
kubectl rollout status deployment/settings-service -n brewget --timeout=300s
kubectl rollout status deployment/frontend -n brewget --timeout=300s
kubectl rollout status deployment/nginx -n brewget --timeout=300s

echo ""
echo "📊 Current status:"
kubectl get pods -n brewget

echo ""
echo "✅ All deployments updated successfully!"
echo ""
echo "To check the rollout history:"
echo "  kubectl rollout history deployment/<deployment-name> -n brewget"
echo ""
echo "To rollback a deployment if needed:"
echo "  kubectl rollout undo deployment/<deployment-name> -n brewget"
echo ""
echo "To check logs:"
echo "  kubectl logs -n brewget <pod-name>"
