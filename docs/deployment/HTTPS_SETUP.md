# HTTPS Setup Guide

This guide explains how to expose your finance-query-rust API over HTTPS so it can be accessed securely from the internet.

## Prerequisites

1. **Domain Name**: You need a domain name pointing to your server's IP address
2. **Open Ports**: Ports 80 (HTTP) and 443 (HTTPS) must be open in your firewall
3. **Docker & Docker Compose**: Installed on your server

## Quick Start (Recommended: nginx-proxy)

The easiest way to set up HTTPS is using the `nginx-proxy` with automatic Let's Encrypt certificates.

### Step 1: Set Environment Variables

Create a `.env` file in the project root:

```bash
DOMAIN=api.yourdomain.com
EMAIL=your-email@example.com
REDIS_URL=redis://your-redis-host:6379
```

**Note**: Set `REDIS_URL` to your external cloud Redis service (e.g., Sevalla Redis). If not set, the app will run without Redis (caching and rate limiting will be disabled).

### Step 2: Start the Services

```bash
docker-compose up -d
```

That's it! The nginx-proxy will automatically:
- Obtain SSL certificates from Let's Encrypt
- Configure HTTPS
- Renew certificates automatically
- Handle all SSL/TLS termination

### Step 3: Verify

```bash
curl https://api.yourdomain.com/health
```

## Alternative: Manual nginx Setup

If you prefer more control over the nginx configuration, you can use the `setup-ssl.sh` script with a custom nginx setup. However, the nginx-proxy approach (above) is recommended as it handles everything automatically.

For manual setup, you would need to:
1. Configure nginx manually
2. Set up Let's Encrypt certificates
3. Configure certificate renewal

The nginx-proxy setup is simpler and recommended for most use cases.

## DNS Configuration

Before obtaining SSL certificates, ensure your domain points to your server:

```bash
# Check DNS
dig yourdomain.com
# or
nslookup yourdomain.com
```

The A record should point to your server's public IP address.

## Firewall Configuration

### Ubuntu/Debian (UFW)

```bash
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw reload
```

### CentOS/RHEL (firewalld)

```bash
sudo firewall-cmd --permanent --add-service=http
sudo firewall-cmd --permanent --add-service=https
sudo firewall-cmd --reload
```

### Cloud Providers

- **AWS**: Configure Security Groups to allow ports 80 and 443
- **Google Cloud**: Configure Firewall Rules
- **Azure**: Configure Network Security Groups
- **DigitalOcean**: Configure Cloud Firewalls

## Testing Your Setup

### Test HTTP (should redirect to HTTPS)

```bash
curl -I http://yourdomain.com/health
# Should return 301 redirect
```

### Test HTTPS

```bash
curl https://yourdomain.com/health
# Should return JSON with status: "healthy"
```

### Test API Endpoint

```bash
curl https://yourdomain.com/v1/quotes?symbols=AAPL
```

## Troubleshooting

### Certificate Not Obtained

1. **Check DNS**: Ensure domain points to your server
   ```bash
   dig yourdomain.com
   ```

2. **Check Ports**: Ensure ports 80 and 443 are open
   ```bash
   sudo netstat -tulpn | grep -E ':(80|443)'
   ```

3. **Check Logs**: View nginx and certbot logs
   ```bash
   docker logs finance-query-nginx
   docker logs finance-query-certbot
   ```

### Certificate Renewal Issues

Certificates are automatically renewed by the nginx-proxy setup. If you need to manually renew:

```bash
# Force certificate renewal
docker exec finance-query-letsencrypt /app/force_renew
```

### WebSocket Issues

WebSockets should work automatically with the nginx configuration. If you experience issues:

1. Check nginx logs: `docker logs finance-query-nginx`
2. Verify WebSocket upgrade headers are being passed
3. Test WebSocket connection:
   ```bash
   wscat -c wss://yourdomain.com/v1/ws/quotes
   ```

## Security Best Practices

1. **Keep Certificates Updated**: Automatic renewal is configured
2. **Use Strong SSL Configuration**: The nginx config uses modern TLS settings
3. **Monitor Logs**: Regularly check for security issues
4. **Rate Limiting**: Already configured in the application
5. **Firewall**: Only expose necessary ports

## Production Deployment

For production, consider:

1. **Load Balancing**: Use a load balancer in front of nginx
2. **CDN**: Use Cloudflare or similar for DDoS protection
3. **Monitoring**: Set up monitoring and alerting
4. **Backup**: Regularly backup SSL certificates
5. **Logging**: Centralized logging for security analysis

## Alternative: Cloudflare Tunnel (No Port Opening Required)

If you can't open ports 80/443, use Cloudflare Tunnel:

```bash
# Install cloudflared
docker run cloudflare/cloudflared:latest tunnel --no-autoupdate run --token YOUR_TUNNEL_TOKEN
```

This creates a secure tunnel without exposing ports.

## Alternative: ngrok (Quick Testing)

For quick testing without a domain:

```bash
# Install ngrok
brew install ngrok  # macOS
# or download from https://ngrok.com

# Start tunnel
ngrok http 8080
```

This gives you a temporary HTTPS URL like `https://abc123.ngrok.io`

## Support

If you encounter issues:

1. Check the logs: `docker-compose logs`
2. Verify DNS: `dig yourdomain.com`
3. Test connectivity: `curl -v https://yourdomain.com/health`
4. Review firewall rules

