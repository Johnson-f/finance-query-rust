# External Redis Configuration

This application uses **external cloud Redis** (e.g., Sevalla Redis) instead of a local Redis container.

## Configuration

Set the `REDIS_URL` environment variable to connect to your external Redis service.

### In `.env` file:

```bash
REDIS_URL=redis://your-redis-host:6379
```

### For Redis with Authentication:

```bash
REDIS_URL=redis://username:password@your-redis-host:6379
```

### For Redis with TLS/SSL:

```bash
REDIS_URL=rediss://your-redis-host:6380
```

## Sevalla Redis Example

If you're using Sevalla Redis service, your connection string might look like:

```bash
REDIS_URL=redis://your-instance.sevalla-redis.com:6379
```

Or with authentication:

```bash
REDIS_URL=redis://default:your-password@your-instance.sevalla-redis.com:6379
```

## Testing Redis Connection

### From your server:

```bash
# Test connection
redis-cli -u $REDIS_URL ping
# Should return: PONG
```

### From Docker container:

```bash
docker exec finance-query-rust sh -c 'echo "PING" | nc -w 1 your-redis-host 6379'
```

## Running Without Redis

If `REDIS_URL` is not set, the application will:
- ✅ Still run and serve API requests
- ❌ Disable caching (all requests will hit external APIs)
- ❌ Disable rate limiting (no per-IP rate limits)

The application gracefully degrades when Redis is unavailable.

## Troubleshooting

### Connection Refused

1. **Check Redis URL**: Verify the host and port are correct
2. **Check Network**: Ensure your server can reach the Redis host
3. **Check Firewall**: Ensure port 6379 (or your Redis port) is open
4. **Check Credentials**: Verify username/password if using authentication

### Timeout Issues

1. **Check Network Latency**: Test connection speed
2. **Check Redis Load**: Verify Redis service is not overloaded
3. **Increase Timeout**: May need to configure Redis client timeout in code

### Authentication Errors

1. **Verify Credentials**: Double-check username and password
2. **Check Redis ACL**: Ensure user has proper permissions
3. **Test Manually**: Use `redis-cli` to test authentication

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `REDIS_URL` | Full Redis connection string | `redis://host:6379` |
| `RATE_LIMIT_PER_DAY` | Rate limit per IP (requires Redis) | `10000` |

## Production Best Practices

1. **Use Connection Pooling**: The Redis client uses connection pooling automatically
2. **Monitor Redis**: Set up monitoring for your Redis instance
3. **Backup Strategy**: Ensure your Redis provider has backups
4. **High Availability**: Consider Redis cluster or sentinel for production
5. **Security**: Use TLS/SSL and authentication in production

