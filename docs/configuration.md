# LogLens Configuration Reference

LogLens is configured via environment variables or a configuration file.

## Environment Variables

| Variable | Default | Description |
| -------- | ------- | ----------- |
| `LOGLENS_HOST` | `0.0.0.0` | Bind IP address for server mode |
| `LOGLENS_PORT` | `8080` | Port for HTTP API |
| `LOGLENS_DATABASE_URL` | `sqlite:///data/loglens.db` | SQLx connection URL |
| `LOGLENS_DATA_DIR` | `/data` | Base data directory |
| `LOGLENS_UPLOAD_DIR` | `/data/uploads` | Directory for temporary uploads |
| `LOGLENS_MAX_UPLOAD_BYTES` | `1073741824` (1GB) | Maximum file upload size limit |
| `LOGLENS_SESSION_SECRET` | Required in prod | 32+ byte key for signing session tokens |
| `LOGLENS_RETENTION_DAYS` | `30` | Automated data retention cleanup threshold |
