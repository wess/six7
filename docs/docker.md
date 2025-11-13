# docker guide

## quick start

### using docker compose

```bash
docker-compose up
```

### using docker run

```bash
docker pull ghcr.io/YOUR_USERNAME/six7:main
docker run -p 4040:4040 \
  -v $(pwd)/data:/data \
  ghcr.io/YOUR_USERNAME/six7:main
```

## building image

### build locally

```bash
docker build -t six7 .
```

### multi-platform build

```bash
docker buildx build --platform linux/amd64,linux/arm64 -t six7 .
```

## running container

### basic run

```bash
docker run -p 4040:4040 six7
```

### with volume mount

```bash
docker run -p 4040:4040 \
  -v $(pwd)/data:/data \
  six7
```

### with custom config

```bash
docker run -p 4040:4040 \
  -v $(pwd)/data:/data \
  -v $(pwd)/six7.yaml:/app/six7.yaml \
  six7
```

### with environment variables

```bash
docker run -p 4040:4040 \
  -e RUST_LOG=debug \
  six7
```

## docker compose

### basic compose file

```yaml
version: '3.8'

services:
  six7:
    image: ghcr.io/YOUR_USERNAME/six7:main
    ports:
      - "4040:4040"
    volumes:
      - ./data:/data
      - ./six7.yaml:/app/six7.yaml
    environment:
      - RUST_LOG=info
    restart: unless-stopped
```

### with networks

```yaml
version: '3.8'

services:
  six7:
    image: ghcr.io/YOUR_USERNAME/six7:main
    ports:
      - "4040:4040"
    volumes:
      - ./data:/data
    networks:
      - app-network
    restart: unless-stopped

networks:
  app-network:
    driver: bridge
```

### development setup

```yaml
version: '3.8'

services:
  six7:
    build: .
    ports:
      - "4040:4040"
    volumes:
      - ./data:/data
      - ./six7.yaml:/app/six7.yaml
    environment:
      - RUST_LOG=debug
```

## configuration

### volumes

- `/data` - storage directory
- `/app/six7.yaml` - config file

### ports

- `4040` - http server

### environment variables

- `RUST_LOG` - log level (error, warn, info, debug, trace)

## image details

### size

- build image: ~1.5gb
- runtime image: ~100mb

### architecture

- multi-stage build
- rust:1.83-slim builder
- debian:bookworm-slim runtime

### tags

- `main` - latest main branch
- `v*` - version tags
- `sha-*` - commit sha tags

## github container registry

### pulling image

```bash
docker pull ghcr.io/YOUR_USERNAME/six7:main
```

### authentication

```bash
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin
```

### available tags

```bash
docker images ghcr.io/YOUR_USERNAME/six7 --format "table {{.Repository}}:{{.Tag}}\t{{.Size}}"
```

## healthcheck

```dockerfile
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:4040/ || exit 1
```

## security

- runs as root (no user specified)
- no secrets in image
- no sensitive data in logs
- cors enabled (development only)

## troubleshooting

### check logs

```bash
docker logs <container-id>
```

### check running containers

```bash
docker ps
```

### inspect container

```bash
docker inspect <container-id>
```

### exec into container

```bash
docker exec -it <container-id> /bin/bash
```

### check storage

```bash
docker exec <container-id> ls -la /data
```

## production considerations

- use specific version tags (not `main`)
- mount persistent volumes
- configure resource limits
- set up log rotation
- use docker secrets for sensitive config
- run behind reverse proxy
- disable cors in production
