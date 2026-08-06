# LogLens Deployment Guide

## Docker Compose (SQLite Default)

```yaml
version: '3.8'

services:
  loglens:
    build: .
    ports:
      - "8080:8080"
    environment:
      - LOGLENS_DATABASE_URL=sqlite:///data/loglens.db
      - LOGLENS_SESSION_SECRET=a_very_long_secure_random_secret_string_here
    volumes:
      - loglens_data:/data
    restart: unless-stopped

volumes:
  loglens_data:
```

## Reverse Proxy Examples

### Nginx

```nginx
server {
    listen 80;
    server_name loglens.example.com;

    client_max_body_size 1G;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Server-Sent Events support
        proxy_set_header Connection '';
        proxy_http_version 1.1;
        proxy_buffering off;
        proxy_cache off;
    }
}
```
