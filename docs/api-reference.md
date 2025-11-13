# api reference

six7 implements s3-compatible api endpoints.

## bucket operations

### create bucket

```
PUT /{bucket}
```

creates a new bucket.

**example:**
```bash
curl -X PUT http://localhost:4040/my-bucket
```

**response:**
- `200 OK` - bucket created
- `500 Internal Server Error` - creation failed

### check bucket exists

```
HEAD /{bucket}
```

checks if bucket exists.

**example:**
```bash
curl -I http://localhost:4040/my-bucket
```

**response:**
- `200 OK` - bucket exists
- `404 Not Found` - bucket doesn't exist

### list objects

```
GET /{bucket}?prefix=&delimiter=&max-keys=
```

lists objects in bucket.

**query parameters:**
- `prefix` - filter by prefix
- `delimiter` - group by delimiter
- `max-keys` - limit results (default: 1000)
- `continuation-token` - pagination token

**example:**
```bash
# list all objects
curl http://localhost:4040/my-bucket

# list with prefix
curl http://localhost:4040/my-bucket?prefix=photos/

# list with delimiter (folders)
curl http://localhost:4040/my-bucket?delimiter=/
```

**response:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Name>my-bucket</Name>
  <Prefix></Prefix>
  <MaxKeys>1000</MaxKeys>
  <IsTruncated>false</IsTruncated>
  <Contents>
    <Key>file.txt</Key>
    <LastModified>2024-01-01T00:00:00Z</LastModified>
    <ETag>"d41d8cd98f00b204e9800998ecf8427e"</ETag>
    <Size>1024</Size>
    <StorageClass>STANDARD</StorageClass>
  </Contents>
</ListBucketResult>
```

## object operations

### upload object

```
PUT /{bucket}/{key}
```

uploads an object.

**headers:**
- `Content-Type` - mime type (optional)

**example:**
```bash
curl -X PUT http://localhost:4040/my-bucket/file.txt \
  -H "Content-Type: text/plain" \
  --data-binary @file.txt
```

**response:**
- `200 OK` - object uploaded
  ```
  ETag: "d41d8cd98f00b204e9800998ecf8427e"
  ```
- `500 Internal Server Error` - upload failed

### download object

```
GET /{bucket}/{key}
```

downloads an object.

**example:**
```bash
curl http://localhost:4040/my-bucket/file.txt -o downloaded.txt
```

**response:**
- `200 OK` - object data
  ```
  Content-Type: application/octet-stream
  ```
- `404 Not Found` - object doesn't exist

### get object metadata

```
HEAD /{bucket}/{key}
```

gets object metadata without downloading.

**example:**
```bash
curl -I http://localhost:4040/my-bucket/file.txt
```

**response:**
- `200 OK` - metadata returned
  ```
  Content-Type: text/plain
  Content-Length: 1024
  ETag: "d41d8cd98f00b204e9800998ecf8427e"
  Last-Modified: Mon, 01 Jan 2024 00:00:00 GMT
  ```
- `404 Not Found` - object doesn't exist

### delete object

```
DELETE /{bucket}/{key}
```

deletes an object.

**example:**
```bash
curl -X DELETE http://localhost:4040/my-bucket/file.txt
```

**response:**
- `204 No Content` - object deleted
- `500 Internal Server Error` - deletion failed

## cors

all endpoints support cors with:
- origins: `*`
- methods: `*`
- headers: `*`

## authentication

basic format validation only - accepts any credentials.

## errors

standard http status codes:
- `200 OK` - success
- `204 No Content` - success, no body
- `404 Not Found` - resource not found
- `500 Internal Server Error` - server error

## limitations

not implemented:
- multipart uploads
- versioning
- bucket policies
- access control lists
- encryption
- lifecycle policies
- replication
- logging
- metrics
