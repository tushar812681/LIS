# LIS Modern - Production Deployment Guide

## Table of Contents
- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Pre-Deployment Checklist](#pre-deployment-checklist)
- [Infrastructure Setup](#infrastructure-setup)
- [Configuration](#configuration)
- [Database Setup](#database-setup)
- [Kubernetes Deployment](#kubernetes-deployment)
- [Monitoring & Observability](#monitoring--observability)
- [Security Hardening](#security-hardening)
- [Backup & Disaster Recovery](#backup--disaster-recovery)
- [Troubleshooting](#troubleshooting)
- [Rollback Procedures](#rollback-procedures)

---

## Overview

This guide provides comprehensive instructions for deploying the LIS Modern system to a production environment with high availability, security, and reliability.

### Architecture Summary
- **Microservices**: 14 independent services
- **Runtime**: Rust-based containerized applications
- **Orchestration**: Kubernetes
- **Databases**: PostgreSQL (primary), Redis (cache), MongoDB (analytics), Kafka (events)
- **Load Balancing**: NGINX Ingress Controller
- **Monitoring**: Prometheus + Grafana
- **Logging**: ELK Stack (Elasticsearch, Logstash, Kibana)
- **Tracing**: Jaeger

---

## Prerequisites

### Infrastructure Requirements

#### Minimum Hardware (Production)
- **Kubernetes Cluster**:
  - 3+ master nodes (4 CPU, 16GB RAM each)
  - 6+ worker nodes (8 CPU, 32GB RAM each)
  - 1TB+ SSD storage per node
- **Network**: 1 Gbps bandwidth, low latency (<50ms)
- **Load Balancer**: Managed load balancer or dedicated hardware

#### Recommended Cloud Providers
- **India**: AWS Mumbai (ap-south-1), Azure India Central
- **Backup Region**: AWS Singapore (ap-southeast-1)

### Software Requirements
```bash
# Required tools
kubectl >= 1.28
helm >= 3.12
docker >= 24.0
terraform >= 1.5 (if using IaC)
```

### Access Requirements
- Kubernetes cluster admin access
- Docker registry credentials (GitHub Container Registry, Docker Hub, or ECR)
- SSL/TLS certificates
- DNS management access

---

## Pre-Deployment Checklist

### Security
- [ ] Generate strong secrets (JWT, encryption keys, database passwords)
- [ ] Obtain SSL/TLS certificates
- [ ] Configure firewall rules
- [ ] Set up VPN for administrative access
- [ ] Enable audit logging
- [ ] Configure RBAC policies
- [ ] Scan Docker images for vulnerabilities
- [ ] Set up secrets management (Vault, AWS Secrets Manager)

### Configuration
- [ ] Update `.env.example` with production values
- [ ] Configure database connection strings
- [ ] Set up external API credentials (UIDAI, ABDM, WhatsApp, Razorpay)
- [ ] Configure CORS allowed origins
- [ ] Set appropriate rate limits
- [ ] Configure monitoring endpoints

### Infrastructure
- [ ] Provision Kubernetes cluster
- [ ] Set up persistent storage (PVC)
- [ ] Configure networking (CNI plugin)
- [ ] Set up Ingress controller
- [ ] Configure DNS records
- [ ] Set up CDN (optional)

### Data
- [ ] Plan database migration strategy
- [ ] Set up database backups
- [ ] Configure replication
- [ ] Test restore procedures
- [ ] Plan data retention policies

---

## Infrastructure Setup

### 1. Kubernetes Cluster Setup

#### Using AWS EKS
```bash
# Install eksctl
brew install eksctl

# Create cluster
eksctl create cluster \
  --name lis-modern-prod \
  --region ap-south-1 \
  --version 1.28 \
  --nodegroup-name standard-workers \
  --node-type t3.xlarge \
  --nodes 6 \
  --nodes-min 3 \
  --nodes-max 12 \
  --managed

# Configure kubectl
aws eks update-kubeconfig --region ap-south-1 --name lis-modern-prod
```

#### Using Azure AKS
```bash
# Create resource group
az group create --name lis-modern-rg --location centralindia

# Create AKS cluster
az aks create \
  --resource-group lis-modern-rg \
  --name lis-modern-prod \
  --node-count 6 \
  --node-vm-size Standard_D4s_v3 \
  --enable-managed-identity \
  --network-plugin azure \
  --kubernetes-version 1.28

# Get credentials
az aks get-credentials --resource-group lis-modern-rg --name lis-modern-prod
```

### 2. Install Required Operators

```bash
# Install cert-manager for SSL
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.13.0/cert-manager.yaml

# Install NGINX Ingress Controller
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
helm install nginx-ingress ingress-nginx/ingress-nginx \
  --namespace ingress-nginx \
  --create-namespace \
  --set controller.replicaCount=3 \
  --set controller.nodeSelector."beta\.kubernetes\.io/os"=linux

# Install Prometheus & Grafana
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm install prometheus prometheus-community/kube-prometheus-stack \
  --namespace monitoring \
  --create-namespace
```

---

## Configuration

### 1. Create Namespace
```bash
kubectl create namespace lis-modern
kubectl label namespace lis-modern environment=production
```

### 2. Configure Secrets

**IMPORTANT**: Never store secrets in Git. Use a secrets manager.

```bash
# Create PostgreSQL secret
kubectl create secret generic lis-secrets \
  --namespace lis-modern \
  --from-literal=POSTGRES_PASSWORD='your-strong-password' \
  --from-literal=JWT_SECRET='your-jwt-secret-minimum-32-characters' \
  --from-literal=ENCRYPTION_KEY='your-encryption-key-32-chars'

# Create external API secrets
kubectl create secret generic external-api-secrets \
  --namespace lis-modern \
  --from-literal=UIDAI_API_KEY='your-uidai-key' \
  --from-literal=ABDM_CLIENT_SECRET='your-abdm-secret' \
  --from-literal=WHATSAPP_ACCESS_TOKEN='your-whatsapp-token' \
  --from-literal=RAZORPAY_KEY_SECRET='your-razorpay-secret'
```

### 3. Configure ConfigMaps

```bash
kubectl apply -f infrastructure/kubernetes/base/configmap.yaml
```

### 4. SSL/TLS Certificates

```yaml
# certificate.yaml
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: lis-modern-tls
  namespace: lis-modern
spec:
  secretName: lis-modern-tls-secret
  issuerRef:
    name: letsencrypt-prod
    kind: ClusterIssuer
  dnsNames:
    - app.lismodern.com
    - api.lismodern.com
```

```bash
kubectl apply -f certificate.yaml
```

---

## Database Setup

### 1. PostgreSQL Setup

#### Using Managed Database (Recommended)
- **AWS RDS**: PostgreSQL 16.x with Multi-AZ deployment
- **Azure Database**: PostgreSQL Flexible Server
- **Google Cloud SQL**: PostgreSQL with HA configuration

```bash
# AWS RDS Example
aws rds create-db-instance \
  --db-instance-identifier lis-modern-prod \
  --db-instance-class db.r6g.2xlarge \
  --engine postgres \
  --engine-version 16.1 \
  --master-username postgres \
  --master-user-password YOUR_PASSWORD \
  --allocated-storage 500 \
  --storage-type gp3 \
  --multi-az \
  --backup-retention-period 30 \
  --preferred-backup-window "03:00-04:00" \
  --preferred-maintenance-window "sun:04:00-sun:05:00"
```

#### Using Kubernetes (For smaller deployments)
```bash
helm repo add bitnami https://charts.bitnami.com/bitnami
helm install postgresql bitnami/postgresql \
  --namespace lis-modern \
  --set auth.postgresPassword=YOUR_PASSWORD \
  --set primary.persistence.size=100Gi \
  --set replication.enabled=true \
  --set replication.replicaCount=2
```

### 2. Run Database Migrations

```bash
# From backend directory
cd backend

# Run migrations for all services
make migrate

# Or manually for each service
cd services/patient-service
sqlx migrate run
```

### 3. Initialize Databases

```bash
# Create individual databases for each service
psql -h YOUR_DB_HOST -U postgres << EOF
CREATE DATABASE lis_patient;
CREATE DATABASE lis_sample;
CREATE DATABASE lis_order;
CREATE DATABASE lis_result;
CREATE DATABASE lis_user;
CREATE DATABASE lis_organization;
CREATE DATABASE lis_equipment;
CREATE DATABASE lis_qc;
CREATE DATABASE lis_billing;
CREATE DATABASE lis_report;
CREATE DATABASE lis_inventory;
CREATE DATABASE lis_notification;
CREATE DATABASE lis_analytics;
CREATE DATABASE lis_compliance;
EOF
```

---

## Kubernetes Deployment

### 1. Build and Push Docker Images

```bash
cd backend

# Build all service images
make docker-build IMAGE_TAG=v1.0.0

# Push to registry
make docker-push IMAGE_TAG=v1.0.0
```

### 2. Deploy Infrastructure Services

```bash
# Deploy PostgreSQL (if using in-cluster)
kubectl apply -f infrastructure/kubernetes/base/postgres.yaml

# Deploy Redis
kubectl apply -f infrastructure/kubernetes/base/redis.yaml

# Deploy Kafka
kubectl apply -f infrastructure/kubernetes/base/kafka.yaml
```

### 3. Deploy Application Services

```bash
# Apply base configuration
kubectl apply -k infrastructure/kubernetes/overlays/prod

# Verify deployment
kubectl get pods -n lis-modern
kubectl get services -n lis-modern
```

### 4. Configure Ingress

```yaml
# ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: lis-modern-ingress
  namespace: lis-modern
  annotations:
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
    nginx.ingress.kubernetes.io/rate-limit: "100"
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
spec:
  ingressClassName: nginx
  tls:
    - hosts:
        - api.lismodern.com
      secretName: lis-modern-tls-secret
  rules:
    - host: api.lismodern.com
      http:
        paths:
          - path: /patient
            pathType: Prefix
            backend:
              service:
                name: patient-service
                port:
                  number: 8081
          # ... other services
```

### 5. Verify Deployment

```bash
# Check pod status
kubectl get pods -n lis-modern -w

# Check logs
kubectl logs -f deployment/patient-service -n lis-modern

# Test health endpoints
kubectl port-forward -n lis-modern svc/patient-service 8081:8081
curl http://localhost:8081/health
```

---

## Monitoring & Observability

### 1. Prometheus Setup

```bash
# Prometheus is already installed via kube-prometheus-stack
# Access Prometheus UI
kubectl port-forward -n monitoring svc/prometheus-kube-prometheus-prometheus 9090:9090
```

### 2. Grafana Dashboards

```bash
# Access Grafana
kubectl port-forward -n monitoring svc/prometheus-grafana 3000:80

# Default credentials
# Username: admin
# Password: kubectl get secret -n monitoring prometheus-grafana -o jsonpath="{.data.admin-password}" | base64 -d
```

Import dashboards:
- Kubernetes Cluster Monitoring
- Rust Application Metrics
- PostgreSQL Monitoring
- Redis Monitoring

### 3. Logging (ELK Stack)

```bash
# Install ELK stack
helm repo add elastic https://helm.elastic.co
helm install elasticsearch elastic/elasticsearch --namespace logging --create-namespace
helm install kibana elastic/kibana --namespace logging
helm install filebeat elastic/filebeat --namespace logging
```

### 4. Distributed Tracing (Jaeger)

```bash
# Install Jaeger
helm repo add jaegertracing https://jaegertracing.github.io/helm-charts
helm install jaeger jaegertracing/jaeger --namespace tracing --create-namespace
```

---

## Security Hardening

### 1. Network Policies

```yaml
# network-policy.yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: lis-network-policy
  namespace: lis-modern
spec:
  podSelector:
    matchLabels:
      app: patient-service
  policyTypes:
    - Ingress
    - Egress
  ingress:
    - from:
        - podSelector:
            matchLabels:
              app: api-gateway
      ports:
        - protocol: TCP
          port: 8081
  egress:
    - to:
        - podSelector:
            matchLabels:
              app: postgres
      ports:
        - protocol: TCP
          port: 5432
```

### 2. Pod Security Policies

```bash
# Apply pod security standards
kubectl label namespace lis-modern pod-security.kubernetes.io/enforce=restricted
```

### 3. RBAC Configuration

```yaml
# rbac.yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: lis-service-account
  namespace: lis-modern
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: lis-role
  namespace: lis-modern
rules:
  - apiGroups: [""]
    resources: ["secrets", "configmaps"]
    verbs: ["get", "list"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: lis-role-binding
  namespace: lis-modern
subjects:
  - kind: ServiceAccount
    name: lis-service-account
roleRef:
  kind: Role
  name: lis-role
  apiGroup: rbac.authorization.k8s.io
```

### 4. Enable Audit Logging

```bash
# Configure kube-apiserver audit policy
# /etc/kubernetes/audit-policy.yaml
```

---

## Backup & Disaster Recovery

### 1. Database Backups

#### Automated Backups (PostgreSQL)
```bash
# Using pg_dump in CronJob
apiVersion: batch/v1
kind: CronJob
metadata:
  name: postgres-backup
  namespace: lis-modern
spec:
  schedule: "0 2 * * *"  # Daily at 2 AM
  jobTemplate:
    spec:
      template:
        spec:
          containers:
            - name: backup
              image: postgres:16-alpine
              command:
                - /bin/sh
                - -c
                - |
                  pg_dump -h $POSTGRES_HOST -U $POSTGRES_USER lis_patient | gzip > /backup/patient-$(date +%Y%m%d).sql.gz
                  # Upload to S3
                  aws s3 cp /backup/patient-$(date +%Y%m%d).sql.gz s3://lis-backups/
```

### 2. Kubernetes State Backup

```bash
# Install Velero
helm repo add vmware-tanzu https://vmware-tanzu.github.io/helm-charts
helm install velero vmware-tanzu/velero \
  --namespace velero \
  --create-namespace \
  --set configuration.provider=aws \
  --set configuration.backupStorageLocation.bucket=lis-k8s-backups

# Create backup schedule
velero schedule create daily-backup --schedule="0 3 * * *"
```

### 3. Disaster Recovery Plan

**RTO (Recovery Time Objective)**: 4 hours
**RPO (Recovery Point Objective)**: 1 hour

#### Recovery Steps:
1. Provision new cluster in backup region
2. Restore Kubernetes state from Velero backup
3. Restore databases from latest backup
4. Update DNS to point to new cluster
5. Verify all services are healthy
6. Run smoke tests

---

## Troubleshooting

### Common Issues

#### Pods Not Starting
```bash
# Check pod events
kubectl describe pod POD_NAME -n lis-modern

# Check logs
kubectl logs POD_NAME -n lis-modern

# Common fixes:
# - Check resource limits
# - Verify secrets exist
# - Check image pull permissions
```

#### Database Connection Errors
```bash
# Test database connectivity
kubectl run -it --rm debug --image=postgres:16 --restart=Never -- psql -h POSTGRES_HOST -U postgres

# Check connection pool limits
# Increase DATABASE_MAX_CONNECTIONS in config
```

#### High Memory Usage
```bash
# Check memory usage
kubectl top pods -n lis-modern

# Adjust resource limits in deployment
```

---

## Rollback Procedures

### Rolling Back a Deployment

```bash
# Check deployment history
kubectl rollout history deployment/patient-service -n lis-modern

# Rollback to previous version
kubectl rollout undo deployment/patient-service -n lis-modern

# Rollback to specific revision
kubectl rollout undo deployment/patient-service -n lis-modern --to-revision=3

# Monitor rollback
kubectl rollout status deployment/patient-service -n lis-modern
```

### Database Rollback

```bash
# Restore from backup
pg_restore -h POSTGRES_HOST -U postgres -d lis_patient backup.dump

# Or revert migrations
cd services/patient-service
sqlx migrate revert
```

---

## Post-Deployment Checklist

- [ ] All pods are running and healthy
- [ ] Health check endpoints returning 200 OK
- [ ] Database connections successful
- [ ] External API integrations working
- [ ] Monitoring dashboards showing metrics
- [ ] Logging pipeline receiving logs
- [ ] SSL certificates valid
- [ ] DNS resolution working
- [ ] Load balancing distributing traffic
- [ ] Backups running successfully
- [ ] Alerts configured and tested
- [ ] Documentation updated
- [ ] Runbooks created
- [ ] Team trained on operations

---

## Support & Escalation

### On-Call Rotation
- **Primary**: DevOps Team
- **Secondary**: Backend Team
- **Escalation**: CTO

### Communication Channels
- **Incidents**: #incidents (Slack)
- **Alerts**: PagerDuty
- **Status Page**: status.lismodern.com

### SLA Commitments
- **P0 (Critical)**: 15 min response, 4 hour resolution
- **P1 (High)**: 1 hour response, 1 day resolution
- **P2 (Medium)**: 4 hour response, 3 day resolution
- **P3 (Low)**: 1 day response, 1 week resolution

---

## Additional Resources

- [Kubernetes Official Docs](https://kubernetes.io/docs/)
- [Rust Production Best Practices](https://rust-lang.github.io/api-guidelines/)
- [PostgreSQL High Availability](https://www.postgresql.org/docs/current/high-availability.html)
- [NABL Compliance Requirements](https://www.nabl-india.org/)

---

**Last Updated**: 2025-11-15
**Version**: 1.0.0
**Maintained By**: LIS Modern DevOps Team
