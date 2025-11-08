#!/bin/bash
# Update BrewGet Kubernetes deployments
# This script forces recreation of all pods with new versions, even if images haven't changed
#
# Note: This script uses patch with restart annotation to force pod recreation
# This method works with all kubectl versions and doesn't require rollout restart support

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

# Function to restart a deployment by patching with a restart annotation
restart_deployment() {
    local deployment=$1
    local timestamp=$(date +%s)
    echo "   Adding restart annotation to force pod recreation..."
    kubectl patch deployment "$deployment" -n brewget -p \
        "{\"spec\":{\"template\":{\"metadata\":{\"annotations\":{\"kubectl.kubernetes.io/restartedAt\":\"$timestamp\"}}}}}"
}

# Force restart all service deployments to pick up new images/configs
echo "📧 Restarting email service..."
kubectl apply -f "$SCRIPT_DIR/05-email-service.yaml"
restart_deployment email-service

echo "🔑 Restarting auth service..."
kubectl apply -f "$SCRIPT_DIR/06-auth-service.yaml"
restart_deployment auth-service

echo "⚙️  Restarting settings service..."
kubectl apply -f "$SCRIPT_DIR/07-settings-service.yaml"
restart_deployment settings-service

echo "💳 Restarting transaction service..."
kubectl apply -f "$SCRIPT_DIR/07.5-transaction-service.yaml"
restart_deployment transaction-service

echo "🎨 Restarting frontend..."
kubectl apply -f "$SCRIPT_DIR/08-frontend.yaml"
restart_deployment frontend

echo "🌐 Restarting nginx..."
kubectl apply -f "$SCRIPT_DIR/09-nginx.yaml"
restart_deployment nginx

echo ""
echo "⏳ Waiting for rollouts to complete..."

# Wait for each deployment to complete its rollout
kubectl rollout status deployment/email-service -n brewget --timeout=300s
kubectl rollout status deployment/auth-service -n brewget --timeout=300s
kubectl rollout status deployment/settings-service -n brewget --timeout=300s
kubectl rollout status deployment/transaction-service -n brewget --timeout=300s
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
