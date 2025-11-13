# configuration

six7 is configured via a yaml file (`six7.yaml`).

## configuration file

### location

the server looks for `six7.yaml` in:
1. current working directory
2. specified path via `--config` flag

```bash
six7 --config /path/to/six7.yaml
```

### full example

```yaml
server:
  host: 127.0.0.1
  port: 4040

storage:
  path: ./data

buckets:
  - name: test-bucket
  - name: uploads
  - name: images
```

## configuration options

### server

```yaml
server:
  host: 127.0.0.1  # bind address
  port: 4040       # listen port
```

- **host**: ip address to bind to
  - `127.0.0.1` - localhost only
  - `0.0.0.0` - all interfaces
- **port**: tcp port (default: 4040)

### storage

```yaml
storage:
  path: ./data  # storage directory
```

- **path**: directory for storing objects
  - relative or absolute path
  - created if doesn't exist
  - buckets stored as subdirectories

### buckets

```yaml
buckets:
  - name: bucket1
  - name: bucket2
  - name: bucket3
```

- **name**: bucket identifier
  - must be dns-compliant
  - lowercase letters, numbers, hyphens
  - 3-63 characters

buckets are pre-created on server start.

## environment variables

none currently supported - all config via yaml.

## directory structure

```
./data/
  bucket1/
    file1.txt
    folder/
      file2.txt
  bucket2/
    image.jpg
```

objects stored as regular files in bucket subdirectories.

## examples

### development

```yaml
server:
  host: 127.0.0.1
  port: 4040

storage:
  path: ./dev-data

buckets:
  - name: test
```

### docker

```yaml
server:
  host: 0.0.0.0  # bind all interfaces in container
  port: 4040

storage:
  path: /data  # mounted volume

buckets:
  - name: uploads
  - name: images
```

### ci/testing

```yaml
server:
  host: 127.0.0.1
  port: 4040

storage:
  path: /tmp/six7-test

buckets:
  - name: test-bucket
```

## validation

config validated on startup. server exits if:
- config file not found
- invalid yaml syntax
- missing required fields
- invalid values
