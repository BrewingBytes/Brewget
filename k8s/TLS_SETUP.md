# TLS/HTTPS Setup with Let's Encrypt

This guide explains how to configure HTTPS for BrewGet using Let's Encrypt certificates.

## Prerequisites

- A Kubernetes cluster with BrewGet deployed
- A domain name pointing to your cluster's external IP (e.g., `brewget.brewingbytes.com`)
- `kubectl` configured to access your cluster
- `certbot` installed on your local machine or a server with access to the cluster

## Option 1: Using cert-manager (Recommended)

cert-manager is a Kubernetes add-on that automates certificate management.

### 1. Install cert-manager

```bash
# Install cert-manager
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.13.0/cert-manager.yaml

# Verify installation
kubectl get pods --namespace cert-manager
```

### 2. Create ClusterIssuer for Let's Encrypt

Create a file `cert-manager-issuer.yaml`:

```yaml
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: letsencrypt-prod
spec:
  acme:
    # Let's Encrypt production server
    server: https://acme-v02.api.letsencrypt.org/directory
    # Email for certificate expiration notifications
    email: admin@brewingbytes.com
    privateKeySecretRef:
      name: letsencrypt-prod-key
    solvers:
    - http01:
        ingress:
          class: nginx
```

Apply it:

```bash
kubectl apply -f cert-manager-issuer.yaml
```

### 3. Create Certificate Resource

Create a file `k8s/10-certificate.yaml`:

```yaml
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: brewget-tls
  namespace: brewget
spec:
  secretName: brewget-tls
  issuerRef:
    name: letsencrypt-prod
    kind: ClusterIssuer
  commonName: brewget.brewingbytes.com
  dnsNames:
  - brewget.brewingbytes.com
```

Apply it:

```bash
kubectl apply -f k8s/10-certificate.yaml
```

### 4. Verify Certificate

```bash
# Check certificate status
kubectl get certificate -n brewget

# Check certificate details
kubectl describe certificate brewget-tls -n brewget

# Verify the secret was created
kubectl get secret brewget-tls -n brewget
```

The certificate will be automatically renewed before expiration.

## Option 2: Manual Certificate Generation

If you prefer manual certificate generation or cannot use cert-manager:

### 1. Generate Certificate with Certbot

On your local machine or a server:

```bash
# Install certbot if not already installed
# On Ubuntu/Debian:
sudo apt-get update
sudo apt-get install certbot

# On macOS:
brew install certbot

# Generate certificate (standalone mode)
sudo certbot certonly --standalone \
  -d brewget.brewingbytes.com \
  --email admin@brewingbytes.com \
  --agree-tos \
  --non-interactive
```

**Note**: For standalone mode, ensure port 80 is accessible and not in use. You may need to temporarily scale down the nginx deployment:

```bash
kubectl scale deployment nginx -n brewget --replicas=0
```

### 2. Create Kubernetes Secret from Certificate

After certbot generates the certificate (typically in `/etc/letsencrypt/live/brewget.brewingbytes.com/`):

```bash
# Create TLS secret from certificate files
kubectl create secret tls brewget-tls \
  --cert=/etc/letsencrypt/live/brewget.brewingbytes.com/fullchain.pem \
  --key=/etc/letsencrypt/live/brewget.brewingbytes.com/privkey.pem \
  -n brewget
```

### 3. Scale nginx Back Up

```bash
kubectl scale deployment nginx -n brewget --replicas=1
```

### 4. Verify HTTPS

```bash
# Check if the secret exists
kubectl get secret brewget-tls -n brewget

# Test HTTPS access
curl -I https://brewget.brewingbytes.com
```

## Option 3: Using Certbot with Webroot

This method allows certificate generation without taking down nginx.

### 1. Create a Job to Run Certbot

Create a file `certbot-job.yaml`:

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: certbot-manual
  namespace: brewget
spec:
  template:
    spec:
      containers:
      - name: certbot
        image: certbot/certbot:latest
        command:
        - certbot
        - certonly
        - --webroot
        - -w
        - /var/www/certbot
        - -d
        - brewget.brewingbytes.com
        - --email
        - admin@brewingbytes.com
        - --agree-tos
        - --non-interactive
        volumeMounts:
        - name: certbot-webroot
          mountPath: /var/www/certbot
        - name: letsencrypt
          mountPath: /etc/letsencrypt
      restartPolicy: Never
      volumes:
      - name: certbot-webroot
        persistentVolumeClaim:
          claimName: certbot-webroot
      - name: letsencrypt
        persistentVolumeClaim:
          claimName: letsencrypt-certs
```

### 2. Create PersistentVolumeClaims

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: certbot-webroot
  namespace: brewget
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 100Mi
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: letsencrypt-certs
  namespace: brewget
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
```

## Certificate Renewal

### With cert-manager

Certificates are automatically renewed. No action needed.

### Manual Renewal

Certificates expire every 90 days. Set up a cron job or CronJob to renew:

```yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: certbot-renewal
  namespace: brewget
spec:
  # Run every day at 2 AM
  schedule: "0 2 * * *"
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: certbot
            image: certbot/certbot:latest
            command:
            - certbot
            - renew
            - --webroot
            - -w
            - /var/www/certbot
            volumeMounts:
            - name: certbot-webroot
              mountPath: /var/www/certbot
            - name: letsencrypt
              mountPath: /etc/letsencrypt
          restartPolicy: OnFailure
          volumes:
          - name: certbot-webroot
            persistentVolumeClaim:
              claimName: certbot-webroot
          - name: letsencrypt
            persistentVolumeClaim:
              claimName: letsencrypt-certs
```

After renewal, you'll need to update the Kubernetes secret:

```bash
# Extract renewed certificates and update secret
kubectl create secret tls brewget-tls \
  --cert=/path/to/fullchain.pem \
  --key=/path/to/privkey.pem \
  -n brewget \
  --dry-run=client -o yaml | kubectl apply -f -

# Restart nginx to pick up new certificates
kubectl rollout restart deployment/nginx -n brewget
```

## Troubleshooting

### Certificate Not Generated

1. Check if your domain points to the correct IP:
   ```bash
   nslookup brewget.brewingbytes.com
   kubectl get service nginx -n brewget
   ```

2. Ensure port 80 is accessible for ACME challenge:
   ```bash
   curl http://brewget.brewingbytes.com/.well-known/acme-challenge/test
   ```

3. Check certbot/cert-manager logs:
   ```bash
   # For cert-manager
   kubectl logs -n cert-manager deployment/cert-manager
   
   # For manual job
   kubectl logs -n brewget job/certbot-manual
   ```

### HTTPS Not Working

1. Verify the TLS secret exists:
   ```bash
   kubectl get secret brewget-tls -n brewget
   kubectl describe secret brewget-tls -n brewget
   ```

2. Check nginx logs:
   ```bash
   kubectl logs -n brewget deployment/nginx
   ```

3. Verify nginx configuration:
   ```bash
   kubectl exec -it -n brewget deployment/nginx -- nginx -t
   ```

### Certificate Expiration Warnings

1. Check certificate expiration:
   ```bash
   kubectl get secret brewget-tls -n brewget -o json | \
     jq -r '.data."tls.crt"' | base64 -d | \
     openssl x509 -noout -enddate
   ```

2. Manually trigger renewal if needed (see renewal section above)

## Security Best Practices

1. **Use strong SSL protocols**: The nginx configuration already uses TLSv1.2 and TLSv1.3
2. **Enable HSTS**: Already configured with `Strict-Transport-Security` header
3. **Monitor certificate expiration**: Set up alerts for certificates expiring in < 30 days
4. **Keep certbot/cert-manager updated**: Regularly update to latest versions
5. **Backup certificates**: Ensure certificates are backed up as part of your backup strategy
6. **Use production Let's Encrypt**: Only use staging for testing to avoid rate limits

## Testing HTTPS Configuration

```bash
# Test SSL configuration
curl -vI https://brewget.brewingbytes.com

# Check SSL certificate details
echo | openssl s_client -connect brewget.brewingbytes.com:443 -servername brewget.brewingbytes.com 2>/dev/null | openssl x509 -noout -text

# Test SSL Labs (external)
# Visit: https://www.ssllabs.com/ssltest/analyze.html?d=brewget.brewingbytes.com
```

## Additional Resources

- [Let's Encrypt Documentation](https://letsencrypt.org/docs/)
- [cert-manager Documentation](https://cert-manager.io/docs/)
- [Certbot Documentation](https://certbot.eff.org/docs/)
- [Nginx SSL Configuration](https://nginx.org/en/docs/http/configuring_https_servers.html)
