# Quick Start: Expose API over HTTPS

Get your API accessible over HTTPS in 3 steps!

## Prerequisites

- Domain name pointing to your server
- Ports 80 and 443 open in firewall
- Docker & Docker Compose installed

## Step 1: Set Your Domain and Redis

Create a `.env` file:

```bash
cat > .env << EOF
DOMAIN=api.yourdomain.com
EMAIL=your-email@example.com
REDIS_URL=redis://your-redis-host:6379
EOF
```

Replace:
- `api.yourdomain.com` with your actual domain
- `your-email@example.com` with your email
- `redis://your-redis-host:6379` with your external Redis connection string (e.g., Sevalla Redis)

**Note**: If you don't set `REDIS_URL`, the app will work but without caching and rate limiting.

## Step 2: Start Services

```bash
docker-compose up -d
```

This will:
- ✅ Start your API
- ✅ Connect to your external Redis (Sevalla Redis)
- ✅ Start nginx reverse proxy
- ✅ Automatically get SSL certificate from Let's Encrypt
- ✅ Configure HTTPS

## Step 3: Test It!

```bash
curl https://api.yourdomain.com/health
```

You should see:
```json
{"status":"healthy","timestamp":"..."}
```

## That's It! 🎉

Your API is now accessible at `https://api.yourdomain.com`

### Example API Calls

```bash
# Get stock quotes
curl https://api.yourdomain.com/v1/quotes?symbols=AAPL,MSFT

# Search for symbols
curl https://api.yourdomain.com/v1/search?q=apple

# Get market movers
curl https://api.yourdomain.com/v1/gainers
```

## Troubleshooting

**Certificate not working?**
- Check DNS: `dig api.yourdomain.com` (should point to your server IP)
- Check ports: `sudo ufw allow 80/tcp && sudo ufw allow 443/tcp`
- Check logs: `docker-compose -f docker-compose.simple.yml logs`

**Need more help?**
See [HTTPS_SETUP.md](./HTTPS_SETUP.md) for detailed instructions.

## Alternative: Quick Testing (No Domain Required)

For quick testing without a domain, use ngrok:

```bash
# Start your app locally
docker-compose up -d

# In another terminal, install and run ngrok
ngrok http 8080
```

This gives you a temporary HTTPS URL like `https://abc123.ngrok.io`

