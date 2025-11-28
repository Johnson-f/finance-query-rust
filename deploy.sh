#!/bin/bash

# Deploy script for Finance Query Rust to VPS
# Usage: ./deploy.sh [domain]
# Example: ./deploy.sh api.tradstry.com

set -e

# Configuration
VPS_USER="root"
VPS_IP="95.216.219.131"
SSH_KEY="$HOME/.ssh/id_ed25519_vps"
REMOTE_DIR="/opt/tradstry"
DOCKER_IMAGE="johnsonf/finance-query-rust:latest"

# Optional: Override domain via argument
DOMAIN="${1:-api.tradstry.com}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Deploying Finance Query Rust to VPS${NC}"
echo "========================================"
echo "VPS: $VPS_USER@$VPS_IP"
echo "Domain: $DOMAIN"
echo "Image: $DOCKER_IMAGE"
echo ""

# Check if SSH key exists
if [ ! -f "$SSH_KEY" ]; then
    echo -e "${RED}❌ SSH key not found at $SSH_KEY${NC}"
    echo "Run: ssh-keygen -t ed25519 -f $SSH_KEY"
    exit 1
fi

# Get script directory (where docker-compose.yml and Caddyfile are)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check required files exist
if [ ! -f "$SCRIPT_DIR/docker-compose.yml" ]; then
    echo -e "${RED}❌ docker-compose.yml not found${NC}"
    exit 1
fi

if [ ! -f "$SCRIPT_DIR/Caddyfile" ]; then
    echo -e "${RED}❌ Caddyfile not found${NC}"
    exit 1
fi

# Step 1: Create remote directory and copy files
echo -e "${YELLOW}📁 Creating remote directory and copying files...${NC}"
ssh -i "$SSH_KEY" "$VPS_USER@$VPS_IP" "mkdir -p $REMOTE_DIR"

scp -i "$SSH_KEY" "$SCRIPT_DIR/docker-compose.yml" "$VPS_USER@$VPS_IP:$REMOTE_DIR/"
scp -i "$SSH_KEY" "$SCRIPT_DIR/Caddyfile" "$VPS_USER@$VPS_IP:$REMOTE_DIR/"

echo -e "${GREEN}✅ Files copied${NC}"

# Step 2: Deploy on VPS
echo -e "${YELLOW}🐳 Deploying on VPS...${NC}"
ssh -i "$SSH_KEY" "$VPS_USER@$VPS_IP" << EOF
set -e
cd $REMOTE_DIR

# Create .env file with domain
echo "DOMAIN=$DOMAIN" > .env
echo "RUST_LOG=info" >> .env
echo "RATE_LIMIT_PER_DAY=10000" >> .env

# Pull latest images
echo "📥 Pulling latest images..."
docker compose pull

# Stop existing containers (if any)
echo "🛑 Stopping existing containers..."
docker compose down --remove-orphans 2>/dev/null || true

# Start services
echo "🚀 Starting services..."
docker compose up -d

# Wait for services to start
sleep 5

# Show status
echo ""
echo "📊 Container Status:"
docker compose ps

echo ""
echo "📋 Recent logs:"
docker compose logs --tail 20
EOF

if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✅ Deployment successful!${NC}"
    echo ""
    echo "🌐 Your API should be available at:"
    echo "   https://$DOMAIN"
    echo "   https://$DOMAIN/graphql"
    echo "   https://$DOMAIN/graphql-playground"
    echo ""
    echo -e "${YELLOW}📋 Useful commands:${NC}"
    echo "   View logs:    ssh -i $SSH_KEY $VPS_USER@$VPS_IP 'cd $REMOTE_DIR && docker compose logs -f'"
    echo "   Check status: ssh -i $SSH_KEY $VPS_USER@$VPS_IP 'cd $REMOTE_DIR && docker compose ps'"
    echo "   Restart:      ssh -i $SSH_KEY $VPS_USER@$VPS_IP 'cd $REMOTE_DIR && docker compose restart'"
    echo "   Stop:         ssh -i $SSH_KEY $VPS_USER@$VPS_IP 'cd $REMOTE_DIR && docker compose down'"
else
    echo -e "${RED}❌ Deployment failed!${NC}"
    exit 1
fi

