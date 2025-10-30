# Kubernetes Deployment for BrewGet

This directory contains the Kubernetes manifests for deploying the BrewGet application to a Kubernetes cluster.

## Architecture

The BrewGet application consists of the following components:

- **nginx**: Reverse proxy and API gateway
- **frontend**: Vue.js frontend application
- **auth-service**: Authentication and authorization service (Rust)
- **settings-service**: User settings service (Rust)
- **email-service**: Email notification service (Rust)
- **postgres**: PostgreSQL database (StatefulSet with persistent storage)

## Prerequisites

- A Kubernetes cluster (minikube, kind, GKE, EKS, AKS, etc.)
- `kubectl` CLI tool installed and configured
- Access to pull the container images from GitHub Container Registry

## Quick Start

### 1. Deploy to Kubernetes

Apply all manifests in order:

```bash
kubectl apply -f k8s/00-namespace.yaml
kubectl apply -f k8s/01-shared-config.yaml
kubectl apply -f k8s/02-secrets.yaml
kubectl apply -f k8s/03-configmaps.yaml
kubectl apply -f k8s/04-postgres.yaml
kubectl apply -f k8s/05-email-service.yaml
kubectl apply -f k8s/06-auth-service.yaml
kubectl apply -f k8s/07-settings-service.yaml
kubectl apply -f k8s/08-frontend.yaml
kubectl apply -f k8s/09-nginx.yaml
```

Or apply all at once:

```bash
kubectl apply -f k8s/
```

### 2. Wait for Pods to be Ready

```bash
kubectl wait --for=condition=ready pod --all -n brewget --timeout=300s
```

### 3. Access the Application

#### For LoadBalancer (Cloud Providers)

```bash
kubectl get service nginx -n brewget
```

Access the application using the EXTERNAL-IP shown.

#### For NodePort (Local Clusters)

If your cluster doesn't support LoadBalancer, change the service type:

```bash
kubectl patch service nginx -n brewget -p '{"spec":{"type":"NodePort"}}'
```

Then get the NodePort:

```bash
kubectl get service nginx -n brewget
```

Access the application at `http://<node-ip>:<node-port>`

#### For Port Forwarding (Development)

```bash
kubectl port-forward -n brewget service/nginx 8080:80
```

Then access at `http://localhost:8080`

## Configuration

### Secrets

The application requires several secrets configured in `02-secrets.yaml`:

- **Database credentials**: `postgres-user`, `postgres-password`
- **JWT configuration**: `jwt-secret`
- **SMTP credentials**: `smtp-email`, `smtp-name`, `smtp-relay`, `smtp-username`, `smtp-password`
- **Captcha configuration**: `turnstile-secret` (Cloudflare Turnstile secret key)

**⚠️ IMPORTANT**: Before deploying to production, update the secrets with secure values:

```bash
# Edit the secrets file
nano k8s/02-secrets.yaml

# Or create a secret from command line
kubectl create secret generic brewget-secrets \
  --from-literal=postgres-user=<your-user> \
  --from-literal=postgres-password=<your-password> \
  --from-literal=jwt-secret=<your-jwt-secret> \
  --from-literal=smtp-email=<your-email> \
  --from-literal=smtp-name=<your-name> \
  --from-literal=smtp-relay=<your-smtp-server> \
  --from-literal=smtp-username=<your-smtp-user> \
  --from-literal=smtp-password=<your-smtp-password> \
  --from-literal=turnstile-secret=<your-turnstile-secret> \
  -n brewget --dry-run=client -o yaml > k8s/02-secrets.yaml
```

For detailed captcha setup instructions, see [CAPTCHA_SETUP.md](../CAPTCHA_SETUP.md).

### ConfigMaps

The `03-configmaps.yaml` file contains:

1. **nginx-config**: Nginx configuration files
2. **postgres-init**: Database initialization script

The postgres-init script automatically creates the required databases:
- `brewget_auth` - Authentication service database
- `brewget_settings` - Settings service database

## Database Migrations

Database migrations are handled automatically by each service using SQLX:

1. When a service starts, it connects to its dedicated database
2. Checks for pending migrations in its `migrations/` directory
3. Applies any unapplied migrations
4. Tracks migration history in the `_sqlx_migrations` table

The PostgreSQL initialization script (`postgres-init` ConfigMap) creates the separate databases on first startup:
- `brewget_auth`
- `brewget_settings`

This ensures each service has its own isolated database for security and maintainability.

## Persistent Storage

The PostgreSQL database uses a hostPath PersistentVolume for data persistence. This ensures that data is stored on the minikube node and persists even when minikube is stopped or restarted.

```yaml
apiVersion: v1
kind: PersistentVolume
metadata:
  name: brewget-postgres-pv
spec:
  storageClassName: manual
  capacity:
    storage: 2Gi
  accessModes:
    - ReadWriteOnce
  hostPath:
    path: "/data/brewget-postgres"
    type: DirectoryOrCreate
  persistentVolumeReclaimPolicy: Retain
```

### Data Persistence in Minikube

For minikube, the hostPath `/data/brewget-postgres` is created inside the minikube VM and persists across `minikube stop` and `minikube start` commands. The data is only lost if you run `minikube delete`.

**Note**: The data is stored inside the minikube VM, not directly on the host machine. This means:
- ✅ Data persists through `minikube stop` and `minikube start`
- ❌ Data is lost when running `minikube delete`

### Manual Backup and Restore

If you need to backup data manually before deleting minikube:

```bash
# Get postgres username
POSTGRES_USER=$(kubectl get secret brewget-secrets -n brewget -o jsonpath='{.data.postgres-user}' | base64 -d)

# Backup a database
kubectl exec postgres-0 -n brewget -- pg_dump -U $POSTGRES_USER brewget_auth > brewget_auth_backup.sql
kubectl exec postgres-0 -n brewget -- pg_dump -U $POSTGRES_USER brewget_settings > brewget_settings_backup.sql

# Restore a database (after recreating cluster and redeploying)
kubectl exec -i postgres-0 -n brewget -- psql -U $POSTGRES_USER -d brewget_auth < brewget_auth_backup.sql
kubectl exec -i postgres-0 -n brewget -- psql -U $POSTGRES_USER -d brewget_settings < brewget_settings_backup.sql
```

### Increasing Storage

To increase storage size, you may need to:

1. Check if your StorageClass supports volume expansion
2. Edit the PVC to request more storage
3. The exact steps depend on your Kubernetes provider and storage backend

## Monitoring

### Check Pod Status

```bash
kubectl get pods -n brewget
```

### View Pod Logs

```bash
# View logs for a specific pod
kubectl logs -n brewget <pod-name>

# Follow logs
kubectl logs -n brewget <pod-name> -f

# View logs from all containers in a pod
kubectl logs -n brewget <pod-name> --all-containers
```

### Check Service Endpoints

```bash
kubectl get endpoints -n brewget
```

### Describe Resources

```bash
kubectl describe pod <pod-name> -n brewget
kubectl describe service <service-name> -n brewget
```

## Scaling

To scale individual services:

```bash
# Scale frontend
kubectl scale deployment frontend -n brewget --replicas=3

# Scale auth-service
kubectl scale deployment auth-service -n brewget --replicas=2

# Scale settings-service
kubectl scale deployment settings-service -n brewget --replicas=2
```

**Note**: PostgreSQL is deployed as a StatefulSet with 1 replica. For high availability, you would need to configure PostgreSQL replication.

## Updates

### Using the Update Script (Recommended)

The easiest way to update all services with new versions is to use the provided update script:

```bash
./k8s/update.sh
```

This script will:
1. Apply any configuration changes from manifests
2. Force restart all deployments to create new pods with the latest images (using annotation patch method)
3. Wait for all rollouts to complete
4. Show the status of all pods

This is useful when you want to ensure all services pick up new versions, even if the image tags haven't changed.

### Manual Update of Individual Services

To update a specific service to a new version:

```bash
# Update the image
kubectl set image deployment/auth-service \
  auth-service=ghcr.io/brewingbytes/brewget-auth-service:0.0.10 \
  -n brewget

# Check rollout status
kubectl rollout status deployment/auth-service -n brewget

# Rollback if needed
kubectl rollout undo deployment/auth-service -n brewget
```

### Force Restart Without Image Change

To restart a deployment without changing the image, you can use one of these methods:

#### Method 1: Using rollout restart (requires kubectl 1.15+)
```bash
kubectl rollout restart deployment/auth-service -n brewget
```

#### Method 2: Using patch with annotation (works with all kubectl versions)
```bash
kubectl patch deployment auth-service -n brewget -p \
  "{\"spec\":{\"template\":{\"metadata\":{\"annotations\":{\"kubectl.kubernetes.io/restartedAt\":\"$(date +%s)\"}}}}}"
```

The update script uses Method 2 for maximum compatibility.

## Cleanup

To remove the entire application:

```bash
kubectl delete namespace brewget
```

**⚠️ WARNING**: This will delete all data including the PostgreSQL database.

To delete specific components:

```bash
kubectl delete -f k8s/08-nginx.yaml
kubectl delete -f k8s/07-frontend.yaml
# ... etc
```

## Troubleshooting

### Pods Not Starting

1. Check pod status:
   ```bash
   kubectl get pods -n brewget
   ```

2. Describe the pod to see events:
   ```bash
   kubectl describe pod <pod-name> -n brewget
   ```

3. Check logs:
   ```bash
   kubectl logs <pod-name> -n brewget
   ```

### Database Connection Issues

1. Check if PostgreSQL is running:
   ```bash
   kubectl get pods -n brewget | grep postgres
   ```

2. Test database connectivity:
   ```bash
   kubectl exec -it postgres-0 -n brewget -- psql -U user-name -d brewget_auth -c "\dt"
   ```

3. Verify secrets are correct:
   ```bash
   kubectl get secret brewget-secrets -n brewget -o yaml
   ```

### Service Not Accessible

1. Check service status:
   ```bash
   kubectl get service -n brewget
   ```

2. Check if pods are running and ready:
   ```bash
   kubectl get pods -n brewget
   ```

3. For LoadBalancer services, ensure your cluster supports LoadBalancer

4. Try port-forwarding to test directly:
   ```bash
   kubectl port-forward -n brewget service/nginx 8080:80
   ```

### Init Container Failures

Init containers wait for dependencies to be ready. If they're failing:

1. Check if the dependency service is running:
   ```bash
   kubectl get pods -n brewget
   ```

2. Check init container logs:
   ```bash
   kubectl logs <pod-name> -n brewget -c <init-container-name>
   ```

3. The init containers will retry automatically

## Production Considerations

### Security

1. **Use strong secrets**: Replace default secrets with strong, randomly generated values
2. **Network policies**: Implement NetworkPolicies to restrict traffic between pods
3. **RBAC**: Configure Role-Based Access Control for the namespace
4. **Image scanning**: Scan container images for vulnerabilities
5. **Pod Security Standards**: Enable and enforce pod security standards

### High Availability

1. **Multiple replicas**: Scale deployments to multiple replicas
2. **Pod Disruption Budgets**: Configure PDBs to ensure availability during updates
3. **PostgreSQL HA**: Consider using a PostgreSQL operator for high availability
4. **Multi-zone deployment**: Spread pods across multiple availability zones

### Performance

1. **Resource limits**: Tune resource requests and limits based on actual usage
2. **HorizontalPodAutoscaler**: Configure HPA for automatic scaling
3. **Database connection pooling**: Already configured in services
4. **Caching**: Consider adding Redis for caching

### Monitoring & Observability

1. **Prometheus**: Deploy Prometheus for metrics collection
2. **Grafana**: Use Grafana for visualization
3. **Logging**: Configure centralized logging (ELK, Loki, etc.)
4. **Tracing**: Implement distributed tracing (Jaeger, Zipkin)
5. **Alerts**: Set up alerting for critical issues

### Backup & Recovery

1. **Database backups**: Regular automated backups of PostgreSQL
2. **Backup testing**: Regularly test backup restoration
3. **Disaster recovery plan**: Document and test DR procedures
4. **Version control**: Keep all manifests in version control

## Differences from Docker Compose

The Kubernetes deployment differs from the Docker Compose setup in the following ways:

1. **Orchestration**: Kubernetes provides built-in orchestration, health checks, and self-healing
2. **Scaling**: Easy horizontal scaling with `kubectl scale`
3. **Secrets management**: Kubernetes Secrets for sensitive data
4. **ConfigMaps**: Externalized configuration separate from images
5. **Service Discovery**: Built-in DNS for service-to-service communication
6. **Load Balancing**: Built-in load balancing for scaled deployments
7. **Rolling Updates**: Zero-downtime deployments with rolling updates
8. **Init Containers**: Proper dependency management with init containers
9. **Persistent Storage**: PersistentVolumeClaims for stateful workloads
10. **Resource Management**: CPU and memory limits/requests for better resource allocation

## Support

For issues or questions:

- GitHub Issues: https://github.com/BrewingBytes/Brewget/issues
- Documentation: See individual service READMEs in the `backend/` directory
