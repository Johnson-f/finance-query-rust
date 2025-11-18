# Health Endpoints

## GET /v1/health

### Overview

**Purpose:** Health check of the API  
**Response Format:** JSON object with service status

### Authentication

None required

### Responses

- **200 OK**
  - **Content-Type:** `application/json`
  - **Example (200):**
    ```json
    {
      "status": "healthy",
      "timestamp": "2025-05-13T19:35:38.383240",
      "services": {
          "status": "all_operational"
      }
    }
    ```

## GET /v1/ping

### Overview

**Purpose:** Simple connectivity check  
**Response Format:** Basic JSON health status with timestamp

### Authentication

None required

### Responses

- **200 OK**
  - **Content-Type:** `application/json`
  - **Example (200):**
    ```json
    {
      "status": "healthy",
      "timestamp": "2023-10-01T12:34:56.789Z"
    }
    ```

## Schema References

### HealthStatus Schema

| Field     | Type   | Description                       | Required |
|-----------|--------|-----------------------------------|:--------:|
| status    | string | Overall health status             |    ✓     |
| timestamp | string | ISO timestamp                     |    ✓     |
| services  | object | Service operational status        |    ✓     |

### PingStatus Schema

| Field     | Type   | Description                       | Required |
|-----------|--------|-----------------------------------|:--------:|
| status    | string | Health status                     |    ✓     |
| timestamp | string | ISO timestamp                     |    ✓     |

