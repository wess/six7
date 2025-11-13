# six7

lightweight, s3-compatible mock service for local development.

## features

- 100% s3-compatible api (put, get, list, delete, head operations)
- local file storage
- configurable via yaml
- cors enabled for browser access
- lightweight and fast
- docker-ready

## quick start

### run the server

```bash
# build and run
cargo run

# or build and run separately
cargo build --release
./target/release/six7
```

server runs on `http://localhost:4040` by default.

### configuration

edit `six7.yaml` to configure:

```yaml
server:
  host: "127.0.0.1"
  port: 4040

storage:
  path: "./data"  # local storage directory

buckets:
  - name: "test-bucket"
```

### run the example app

```bash
cd example
bun install
bun run dev
```

open `http://localhost:5173` to see the example app.

## usage with aws sdk

### javascript/typescript

```typescript
import { S3Client } from '@aws-sdk/client-s3'

const s3Client = new S3Client({
  region: 'us-east-1',
  endpoint: 'http://localhost:4040',
  credentials: {
    accessKeyId: 'test',
    secretAccessKey: 'test',
  },
  forcePathStyle: true,
})
```

### python

```python
import boto3

s3 = boto3.client(
    's3',
    endpoint_url='http://localhost:4040',
    aws_access_key_id='test',
    aws_secret_access_key='test',
    region_name='us-east-1'
)
```

### rust

```rust
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::Client;

let config = aws_sdk_s3::Config::builder()
    .endpoint_url("http://localhost:4040")
    .credentials_provider(Credentials::new(
        "test",
        "test",
        None,
        None,
        "static"
    ))
    .region(Region::new("us-east-1"))
    .force_path_style(true)
    .build();

let client = Client::from_conf(config);
```

## supported operations

- `PUT /:bucket` - create bucket
- `HEAD /:bucket` - check bucket exists
- `GET /:bucket` - list objects
- `PUT /:bucket/:key` - upload object
- `GET /:bucket/:key` - download object
- `HEAD /:bucket/:key` - get object metadata
- `DELETE /:bucket/:key` - delete object

## testing

```bash
# run all tests
cargo test

# run specific test module
cargo test config
cargo test storage
cargo test auth
cargo test integration
```

## docker

### using docker compose

```bash
docker-compose up
```

### pull from github container registry

```bash
docker pull ghcr.io/YOUR_USERNAME/six7:main
docker run -p 4040:4040 -v $(pwd)/data:/data ghcr.io/YOUR_USERNAME/six7:main
```

### build locally

```bash
docker build -t six7 .
docker run -p 4040:4040 -v $(pwd)/data:/data six7
```

## documentation

- [getting started](docs/getting-started.md)
- [configuration](docs/configuration.md)
- [api reference](docs/api-reference.md)
- [docker guide](docs/docker.md)
- [examples](docs/examples.md)

## why six7?

- no jvm overhead like minio
- simpler than localstack
- s3-only focus
- rust performance
- trivial setup

## storage

files stored in configured path with structure:

```
./data/
  bucket-name/
    file1.jpg
    folder/
      file2.png
```

no database needed - just filesystem.

## limitations

- basic auth validation (checks format, not full signature verification)
- no multipart uploads (yet)
- no versioning
- no acls/policies
- no encryption

perfect for dev/testing, not production.
