# Kubernetes Quick Reference for BrewGet

## Deployment

```bash
# Deploy everything
./deploy.sh

# Or apply manually
kubectl apply -f k8s/
```

## Monitoring

```bash
# Check pod status
kubectl get pods -n brewget

# Check all resources
kubectl get all -n brewget

# Watch pods in real-time
kubectl get pods -n brewget --watch

# Check pod logs
kubectl logs -f <pod-name> -n brewget

# Check logs from all containers in a pod
kubectl logs <pod-name> -n brewget --all-containers

# Describe a resource
kubectl describe pod <pod-name> -n brewget
```

## Access Application

```bash
# Get service info
kubectl get svc nginx -n brewget

# Port forward to localhost
kubectl port-forward -n brewget service/nginx 8080:80

# Access at http://localhost:8080
```

## Database Operations

**Note:** Replace `user-name` with your actual PostgreSQL username from the secrets. To get the username:
```bash
kubectl get secret brewget-secrets -n brewget -o jsonpath='{.data.postgres-user}' | base64 -d
```

```bash
# Connect to PostgreSQL
kubectl exec -it postgres-0 -n brewget -- psql -U user-name -d brewget_auth

# List databases
kubectl exec -it postgres-0 -n brewget -- psql -U user-name -l

# Check if databases exist
kubectl exec -it postgres-0 -n brewget -- psql -U user-name -c "\l"

# View tables in auth database
kubectl exec -it postgres-0 -n brewget -- psql -U user-name -d brewget_auth -c "\dt"

# View tables in settings database
kubectl exec -it postgres-0 -n brewget -- psql -U user-name -d brewget_settings -c "\dt"
```

## Backup and Restore

```bash
# Create manual backup
cd k8s
./backup-postgres.sh

# Restore from backup
cd k8s
./restore-postgres.sh

# List available backups
kubectl exec postgres-0 -n brewget -- ls -lh /backup/

# Download backup to local machine
kubectl cp brewget/postgres-0:/backup/latest ./postgres-backup-$(date +%Y%m%d)

# View backup CronJob status
kubectl get cronjob postgres-backup -n brewget

# Trigger manual backup job
kubectl create job --from=cronjob/postgres-backup manual-backup-$(date +%Y%m%d-%H%M%S) -n brewget
```

## Scaling

```bash
# Scale deployment
kubectl scale deployment frontend -n brewget --replicas=3

# Check horizontal pod autoscaler
kubectl get hpa -n brewget
```

## Updates

```bash
# Update image
kubectl set image deployment/auth-service auth-service=ghcr.io/brewingbytes/brewget-auth-service:0.0.9 -n brewget

# Check rollout status
kubectl rollout status deployment/auth-service -n brewget

# View rollout history
kubectl rollout history deployment/auth-service -n brewget

# Rollback to previous version
kubectl rollout undo deployment/auth-service -n brewget
```

## Troubleshooting

```bash
# Check events
kubectl get events -n brewget --sort-by='.lastTimestamp'

# Check pod events
kubectl describe pod <pod-name> -n brewget

# Check service endpoints
kubectl get endpoints -n brewget

# Check configmaps
kubectl get configmap -n brewget

# Check secrets
kubectl get secrets -n brewget

# Test connectivity from a pod
kubectl exec -it <pod-name> -n brewget -- wget -qO- http://auth-service:8000/health

# Shell into a pod
kubectl exec -it <pod-name> -n brewget -- sh

# Check resource usage
kubectl top pods -n brewget
kubectl top nodes
```

## Cleanup

```bash
# Delete everything
./cleanup.sh

# Or manually delete namespace
kubectl delete namespace brewget

# Delete specific resource
kubectl delete deployment frontend -n brewget
```

## Configuration

```bash
# Edit secret
kubectl edit secret brewget-secrets -n brewget

# Update configmap
kubectl edit configmap nginx-config -n brewget

# View secret (base64 encoded)
kubectl get secret brewget-secrets -n brewget -o yaml

# Decode secret
kubectl get secret brewget-secrets -n brewget -o jsonpath='{.data.postgres-password}' | base64 -d
```

## Logs

```bash
# Stream logs
kubectl logs -f deployment/auth-service -n brewget

# Get logs from all pods of a deployment
kubectl logs -f deployment/auth-service --all-containers=true -n brewget

# Get previous pod logs (if pod crashed)
kubectl logs <pod-name> -n brewget --previous

# Save logs to file
kubectl logs <pod-name> -n brewget > pod-logs.txt
```

## Network

```bash
# Test DNS resolution
kubectl run -it --rm debug --image=busybox --restart=Never -n brewget -- nslookup auth-service

# Test connectivity
kubectl run -it --rm debug --image=busybox --restart=Never -n brewget -- wget -qO- http://auth-service:8000/health

# Port forward a service
kubectl port-forward -n brewget svc/auth-service 8000:8000
```

## Persistent Storage

```bash
# List persistent volume claims
kubectl get pvc -n brewget

# Describe PVC
kubectl describe pvc postgres-data-postgres-0 -n brewget

# List persistent volumes
kubectl get pv
```

## Health Checks

```bash
# Check if all pods are ready
kubectl wait --for=condition=ready pod --all -n brewget --timeout=300s

# Get pod readiness
kubectl get pods -n brewget -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.status.conditions[?(@.type=="Ready")].status}{"\n"}{end}'
```
