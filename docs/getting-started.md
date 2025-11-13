# getting started

## installation

### from source

```bash
git clone https://github.com/YOUR_USERNAME/six7.git
cd six7
cargo build --release
```

### using docker

```bash
docker pull ghcr.io/YOUR_USERNAME/six7:main
```

## first run

### create configuration

create `six7.yaml` in your project directory:

```yaml
server:
  host: 127.0.0.1
  port: 4040

storage:
  path: ./data

buckets:
  - name: test-bucket
```

### start the server

```bash
# using cargo
cargo run

# using binary
./target/release/six7

# using docker
docker-compose up
```

the server will start on `http://localhost:4040`

## verify it works

### using curl

```bash
# create bucket
curl -X PUT http://localhost:4040/test-bucket

# upload file
echo "hello world" > test.txt
curl -X PUT http://localhost:4040/test-bucket/test.txt \
  --data-binary @test.txt

# download file
curl http://localhost:4040/test-bucket/test.txt

# list objects
curl http://localhost:4040/test-bucket

# delete object
curl -X DELETE http://localhost:4040/test-bucket/test.txt
```

### using aws cli

```bash
# configure endpoint
export AWS_ENDPOINT_URL=http://localhost:4040
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test

# list buckets
aws s3 ls --endpoint-url $AWS_ENDPOINT_URL

# upload file
aws s3 cp test.txt s3://test-bucket/

# list objects
aws s3 ls s3://test-bucket/

# download file
aws s3 cp s3://test-bucket/test.txt downloaded.txt
```

## next steps

- [configuration guide](configuration.md)
- [api reference](api-reference.md)
- [examples](examples.md)
