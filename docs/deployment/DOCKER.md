# Docker Deployment Guide

This guide explains how to build and deploy the finance-query-rust application using Docker.

## Quick Start

### Build the Docker Image

```bash
docker build -t finance-query-rust:latest .
```

### Run with Docker Compose (Recommended for Local Development)

**Note**: This setup uses external cloud Redis (e.g., Sevalla Redis). Set `REDIS_URL` in your `.env` file or environment.

```bash
# For local development (without HTTPS)
docker-compose -f docker-compose.local.yml up -d

# For production with HTTPS
docker-compose up -d
```

The API will be available at `http://localhost:8080` (local) or `https://yourdomain.com` (production).

### Run Standalone Container

```bash
docker run -d \
  --name finance-query-rust \
  -p 8080:8080 \
  -e REDIS_URL=redis://your-redis-host:6379 \
  -e RATE_LIMIT_PER_DAY=10000 \
  -e RUST_LOG=info \
  finance-query-rust:latest
```

## Environment Variables

The following environment variables can be configured:

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `REDIS_URL` | External Redis connection string (e.g., Sevalla Redis) for caching and rate limiting | None | No |
| `RATE_LIMIT_PER_DAY` | Daily request limit per IP address | 10000 | No |
| `PROXY_URL` | Proxy server URL for HTTP requests | None | No |
| `RUST_LOG` | Logging level (trace/debug/info/warn/error) | info | No |
| `RUST_BACKTRACE` | Enable backtrace on panic (0=off, 1=on) | 0 | No |

**Note**: All environment variables are optional. The application will run with sensible defaults if they are not provided. **Redis is external** - use a cloud Redis service like Sevalla Redis. If `REDIS_URL` is not set, the app will work but caching and rate limiting will be disabled.

## HTTPS Setup (Expose to Internet)

To expose your API over HTTPS, see the comprehensive guide in **[HTTPS_SETUP.md](./HTTPS_SETUP.md)**.

### Quick Start with HTTPS

**Option 1: nginx-proxy (Recommended - Easiest)**

1. Create a `.env` file:
```bash
DOMAIN=api.yourdomain.com
EMAIL=your-email@example.com
REDIS_URL=redis://your-redis-host:6379
```

**Note**: Set `REDIS_URL` to your external cloud Redis service (e.g., Sevalla Redis). The app will work without it, but caching and rate limiting will be disabled.

2. Start services:
```bash
docker-compose up -d
```

The nginx-proxy will automatically obtain and renew SSL certificates from Let's Encrypt.

For detailed instructions, troubleshooting, and alternative methods (Cloudflare Tunnel, ngrok), see **[HTTPS_SETUP.md](./HTTPS_SETUP.md)**.

## Production Deployment

### Build for Production

```bash
docker build -t finance-query-rust:latest .
```

### Run in Production

```bash
docker run -d \
  --name finance-query-rust \
  --restart unless-stopped \
  -p 8080:8080 \
  -e REDIS_URL=redis://your-redis-host:6379 \
  -e RATE_LIMIT_PER_DAY=10000 \
  -e RUST_LOG=info \
  finance-query-rust:latest
```

### Using Docker Compose in Production

1. Copy `docker-compose.yml` to your server
2. Modify environment variables as needed
3. Run:

```bash
docker-compose up -d
```

## Health Checks

The container includes a built-in health check that verifies the `/health` endpoint. You can check the health status:

```bash
docker ps  # Check STATUS column
docker inspect finance-query-rust | grep -A 10 Health
```

## Building for Different Platforms

### Build for Linux/AMD64 (default)

```bash
docker build -t finance-query-rust:latest .
```

### Build for ARM64 (Apple Silicon, Raspberry Pi)

```bash
docker buildx build --platform linux/arm64 -t finance-query-rust:latest .
```

### Build for Multiple Platforms

```bash
docker buildx build --platform linux/amd64,linux/arm64 -t finance-query-rust:latest .
```

## Deployment Platforms

### AWS ECS/Fargate

1. Build and push to ECR:
```bash
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin <account-id>.dkr.ecr.us-east-1.amazonaws.com
docker tag finance-query-rust:latest <account-id>.dkr.ecr.us-east-1.amazonaws.com/finance-query-rust:latest
docker push <account-id>.dkr.ecr.us-east-1.amazonaws.com/finance-query-rust:latest
```

2. Create ECS task definition with the image and environment variables

### Google Cloud Run

```bash
# Build and push
gcloud builds submit --tag gcr.io/<project-id>/finance-query-rust

# Deploy
gcloud run deploy finance-query-rust \
  --image gcr.io/<project-id>/finance-query-rust \
  --platform managed \
  --region us-central1 \
  --port 8080 \
  --set-env-vars REDIS_URL=redis://...,RATE_LIMIT_PER_DAY=10000
```

### Azure Container Instances

```bash
# Build and push to ACR
az acr build --registry <registry-name> --image finance-query-rust:latest .

# Deploy
az container create \
  --resource-group <resource-group> \
  --name finance-query-rust \
  --image <registry-name>.azurecr.io/finance-query-rust:latest \
  --dns-name-label finance-query-rust \
  --ports 8080 \
  --environment-variables REDIS_URL=redis://... RATE_LIMIT_PER_DAY=10000
```

### DigitalOcean App Platform

1. Connect your GitHub repository
2. Configure build settings:
   - Build Command: `docker build -t finance-query-rust .`
   - Run Command: `./finance-query-rust`
3. Set environment variables in the dashboard

### Railway

1. Connect your GitHub repository
2. Railway will auto-detect the Dockerfile
3. Set environment variables in the dashboard

### Render

1. Connect your GitHub repository
2. Select "Docker" as the environment
3. Set environment variables in the dashboard

## Troubleshooting

### Container exits immediately

Check logs:
```bash
docker logs finance-query-rust
```

### Health check failing

Verify the health endpoint:
```bash
curl http://localhost:8080/health
```

### Redis connection issues

Since Redis is external (cloud service), ensure:
1. `REDIS_URL` is correctly set in your `.env` file
2. Your server can reach the Redis host (network/firewall allows connection)
3. Redis credentials are correct (if using authentication)

Test Redis connection:
```bash
# Test from your server
redis-cli -u $REDIS_URL ping
# or
docker exec finance-query-rust sh -c 'curl -v telnet://your-redis-host:6379'
```

### Build fails

Clear Docker cache and rebuild:
```bash
docker builder prune
docker build --no-cache -t finance-query-rust:latest .
```

## Image Size Optimization

The Dockerfile uses a multi-stage build to minimize the final image size. The runtime image is based on `debian:bookworm-slim` and only contains the compiled binary and necessary runtime libraries.

Current image size: ~50-80MB (depending on dependencies)

## Security Best Practices

1. **Non-root user**: The container runs as a non-root user (`appuser`)
2. **Minimal base image**: Uses `debian:bookworm-slim` for smaller attack surface
3. **No build tools in runtime**: Build dependencies are excluded from final image
4. **Health checks**: Built-in health monitoring

## Monitoring

Monitor container resource usage:

```bash
docker stats finance-query-rust
```

View logs:

```bash
docker logs -f finance-query-rust
```

