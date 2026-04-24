# Template Program Ops Specification

> Phenotype Template Platform: Operations & Infrastructure

**Version**: 1.0 | **Status**: Draft | **Last Updated**: 2026-04-02

## Overview

Infrastructure-as-code and DevOps automation templates for the Phenotype ecosystem. Terraform, Ansible, Kubernetes manifests, and CI/CD pipelines.

### Key Features

- **Infrastructure as Code**: Terraform modules for AWS/GCP/Azure
- **Configuration Management**: Ansible playbooks
- **Container Orchestration**: Kubernetes manifests, Helm charts
- **CI/CD Pipelines**: GitHub Actions, GitLab CI, ArgoCD
- **Monitoring**: Prometheus, Grafana, Alertmanager
- **Security**: Vault, cert-manager, network policies

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Template Program Ops                                     │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    Infrastructure (Terraform)                          │   │
│  │                                                                        │   │
│  │   VPC │ EKS/GKE │ RDS │ ElastiCache │ S3 │ IAM                       │   │
│  │                                                                        │   │
│  └────────────────────────────────────────────────────────────────────────┘   │
│                                   │                                         │
│                                   ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    Configuration (Ansible)                             │   │
│  │                                                                        │   │
│  │   OS Hardening │ Package Installation │ Service Configuration         │   │
│  │                                                                        │   │
│  └────────────────────────────────────────────────────────────────────────┘   │
│                                   │                                         │
│                                   ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    Kubernetes (K8s/Helm)                                 │   │
│  │                                                                        │   │
│  │   Namespaces │ Deployments │ Services │ Ingress │ ConfigMaps          │   │
│  │                                                                        │   │
│  └────────────────────────────────────────────────────────────────────────┘   │
│                                   │                                         │
│                                   ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    Observability                                       │   │
│  │                                                                        │   │
│  │   Prometheus │ Grafana │ Jaeger │ Loki │ Alertmanager                │   │
│  │                                                                        │   │
│  └────────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Project Structure

```
template-program-ops/
├── terraform/                  # Infrastructure as Code
│   ├── modules/                # Reusable modules
│   │   ├── vpc/
│   │   ├── eks/
│   │   ├── rds/
│   │   └── iam/
│   ├── environments/           # Environment configs
│   │   ├── dev/
│   │   ├── staging/
│   │   └── prod/
│   └── backend.tf              # State backend
├── ansible/                    # Configuration management
│   ├── playbooks/
│   │   ├── site.yml
│   │   └── setup-k8s.yml
│   ├── roles/
│   │   ├── common/
│   │   ├── docker/
│   │   └── kubernetes/
│   └── inventory/
├── kubernetes/                 # K8s manifests
│   ├── base/                   # Base resources
│   │   ├── namespace.yml
│   │   ├── deployment.yml
│   │   ├── service.yml
│   │   └── ingress.yml
│   └── overlays/               # Kustomize overlays
│       ├── dev/
│       └── prod/
├── helm/                       # Helm charts
│   └── phenotype-app/
├── monitoring/                 # Observability
│   ├── prometheus/
│   ├── grafana/
│   └── alertmanager/
├── .github/
│   └── workflows/              # CI/CD pipelines
└── scripts/                    # Utility scripts
```

## Terraform Modules

### VPC Module

```hcl
# terraform/modules/vpc/main.tf
module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "~> 5.0"

  name = "${var.environment}-vpc"
  cidr = var.vpc_cidr

  azs             = var.availability_zones
  private_subnets = var.private_subnet_cidrs
  public_subnets  = var.public_subnet_cidrs

  enable_nat_gateway     = true
  single_nat_gateway     = var.environment != "prod"
  one_nat_gateway_per_az = var.environment == "prod"

  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = {
    Environment = var.environment
    Project     = var.project_name
  }
}
```

### EKS Module

```hcl
# terraform/modules/eks/main.tf
module "eks" {
  source  = "terraform-aws-modules/eks/aws"
  version = "~> 19.0"

  cluster_name    = "${var.environment}-eks"
  cluster_version = "1.28"

  vpc_id     = var.vpc_id
  subnet_ids = var.private_subnet_ids

  eks_managed_node_groups = {
    general = {
      desired_size = var.node_desired_size
      min_size     = var.node_min_size
      max_size     = var.node_max_size

      instance_types = var.node_instance_types
      capacity_type  = "ON_DEMAND"
    }
  }

  tags = {
    Environment = var.environment
  }
}
```

## Ansible Playbooks

```yaml
# ansible/playbooks/site.yml
---
- name: Setup base system
  hosts: all
  become: yes
  roles:
    - common
    - docker
    
- name: Setup Kubernetes cluster
  hosts: k8s
  become: yes
  roles:
    - kubernetes
    
- name: Deploy application
  hosts: k8s_master
  roles:
    - deploy

# ansible/roles/common/tasks/main.yml
---
- name: Update apt cache
  apt:
    update_cache: yes
    cache_valid_time: 3600
  when: ansible_os_family == "Debian"

- name: Install common packages
  apt:
    name:
      - vim
      - htop
      - curl
      - wget
      - git
    state: present
```

## Kubernetes Manifests

### Base Resources

```yaml
# kubernetes/base/deployment.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: phenotype-app
  namespace: phenotype
spec:
  replicas: 3
  selector:
    matchLabels:
      app: phenotype-app
  template:
    metadata:
      labels:
        app: phenotype-app
    spec:
      containers:
      - name: app
        image: phenotype/app:latest
        ports:
        - containerPort: 8080
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
---
# kubernetes/base/service.yml
apiVersion: v1
kind: Service
metadata:
  name: phenotype-app
  namespace: phenotype
spec:
  selector:
    app: phenotype-app
  ports:
  - port: 80
    targetPort: 8080
  type: ClusterIP
```

### Kustomize Overlays

```yaml
# kubernetes/overlays/dev/kustomization.yml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization

namespace: phenotype-dev

resources:
  - ../../base

patchesStrategicMerge:
  - deployment-patch.yml

images:
  - name: phenotype/app
    newTag: dev-latest

replicas:
  - name: phenotype-app
    count: 1
```

## GitHub Actions CI/CD

```yaml
# .github/workflows/deploy.yml
name: Deploy

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  terraform:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Terraform
        uses: hashicorp/setup-terraform@v3
        with:
          terraform_version: "1.6.0"
      
      - name: Terraform Format
        run: terraform fmt -check -recursive
      
      - name: Terraform Init
        run: terraform init
        working-directory: ./terraform/environments/dev
      
      - name: Terraform Plan
        run: terraform plan
        working-directory: ./terraform/environments/dev
      
      - name: Terraform Apply
        if: github.ref == 'refs/heads/main'
        run: terraform apply -auto-approve
        working-directory: ./terraform/environments/dev

  deploy:
    needs: terraform
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Configure AWS Credentials
        uses: aws-actions/configure-aws-credentials@v4
        with:
          aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
          aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
          aws-region: us-east-1
      
      - name: Update kubeconfig
        run: aws eks update-kubeconfig --name dev-eks
      
      - name: Deploy to Kubernetes
        run: |
          kubectl apply -k kubernetes/overlays/dev
          kubectl rollout status deployment/phenotype-app -n phenotype-dev
```

## Monitoring Stack

```yaml
# monitoring/prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

alerting:
  alertmanagers:
    - static_configs:
        - targets: ['alertmanager:9093']

rule_files:
  - /etc/prometheus/rules/*.yml

scrape_configs:
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']
  
  - job_name: 'kubernetes-pods'
    kubernetes_sd_configs:
      - role: pod
    relabel_configs:
      - source_labels: [__meta_kubernetes_pod_annotation_prometheus_io_scrape]
        action: keep
        regex: true
```

## Quick Start

```bash
# 1. Clone template
git clone https://github.com/KooshaPari/template-program-ops.git
cd template-program-ops

# 2. Configure backend
cp terraform/backend.tf.example terraform/backend.tf
# Edit with your S3 bucket and DynamoDB table

# 3. Initialize Terraform
cd terraform/environments/dev
terraform init

# 4. Plan infrastructure
terraform plan

# 5. Apply infrastructure
terraform apply

# 6. Configure kubectl
aws eks update-kubeconfig --name dev-eks --region us-east-1

# 7. Deploy application
kubectl apply -k ../../../kubernetes/overlays/dev

# 8. Verify deployment
kubectl get pods -n phenotype-dev
kubectl get svc -n phenotype-dev
```

## References

- [Terraform Documentation](https://developer.hashicorp.com/terraform/docs)
- [Ansible Documentation](https://docs.ansible.com/)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Helm Documentation](https://helm.sh/docs/)
- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Documentation](https://grafana.com/docs/)
