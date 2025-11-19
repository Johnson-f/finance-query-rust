#!/bin/bash

# SSL Certificate Setup Script for finance-query-rust
# This script helps you obtain SSL certificates from Let's Encrypt

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔒 SSL Certificate Setup for finance-query-rust${NC}"
echo "=========================================="

# Check if domain is provided
if [ -z "$1" ]; then
    echo -e "${YELLOW}Usage: ./setup-ssl.sh <your-domain.com> <your-email@example.com>${NC}"
    echo ""
    echo "Example: ./setup-ssl.sh api.example.com admin@example.com"
    exit 1
fi

DOMAIN=$1
EMAIL=${2:-"admin@${DOMAIN}"}

echo -e "${BLUE}Domain:${NC} $DOMAIN"
echo -e "${BLUE}Email:${NC} $EMAIL"
echo ""

# Create necessary directories
echo -e "${BLUE}📁 Creating directories...${NC}"
mkdir -p nginx/certbot/conf
mkdir -p nginx/certbot/www

# Update nginx config with domain
echo -e "${BLUE}📝 Updating nginx configuration...${NC}"
sed -i.bak "s/YOUR_DOMAIN/$DOMAIN/g" nginx/nginx.conf
rm -f nginx/nginx.conf.bak

# Start nginx temporarily for certificate validation
echo -e "${BLUE}🚀 Starting nginx for certificate validation...${NC}"
docker-compose -f docker-compose.prod.yml up -d nginx

# Wait for nginx to be ready
echo -e "${BLUE}⏳ Waiting for nginx to be ready...${NC}"
sleep 5

# Obtain certificate
echo -e "${BLUE}📜 Obtaining SSL certificate from Let's Encrypt...${NC}"
docker run --rm \
    -v "$(pwd)/nginx/certbot/conf:/etc/letsencrypt" \
    -v "$(pwd)/nginx/certbot/www:/var/www/certbot" \
    certbot/certbot \
    certonly \
    --webroot \
    --webroot-path=/var/www/certbot \
    --email "$EMAIL" \
    --agree-tos \
    --no-eff-email \
    -d "$DOMAIN"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ SSL certificate obtained successfully!${NC}"
    echo ""
    echo -e "${BLUE}🔄 Restarting nginx with SSL configuration...${NC}"
    docker-compose -f docker-compose.prod.yml restart nginx
    
    echo ""
    echo -e "${GREEN}✅ Setup complete!${NC}"
    echo ""
    echo -e "${BLUE}Your API is now available at:${NC}"
    echo -e "${GREEN}https://$DOMAIN${NC}"
    echo ""
    echo -e "${BLUE}Test it with:${NC}"
    echo -e "${YELLOW}curl https://$DOMAIN/health${NC}"
else
    echo -e "${RED}❌ Failed to obtain SSL certificate${NC}"
    echo ""
    echo "Common issues:"
    echo "1. Make sure your domain points to this server's IP address"
    echo "2. Ensure ports 80 and 443 are open in your firewall"
    echo "3. Check that nginx is running and accessible"
    exit 1
fi

