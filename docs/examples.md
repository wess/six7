# examples

## aws sdk examples

### javascript/typescript

#### basic upload and download

```typescript
import { S3Client, PutObjectCommand, GetObjectCommand } from '@aws-sdk/client-s3'

const client = new S3Client({
  endpoint: 'http://localhost:4040',
  region: 'us-east-1',
  credentials: {
    accessKeyId: 'test',
    secretAccessKey: 'test'
  },
  forcePathStyle: true
})

// upload file
async function upload() {
  const command = new PutObjectCommand({
    Bucket: 'test-bucket',
    Key: 'hello.txt',
    Body: 'hello world',
    ContentType: 'text/plain'
  })

  const response = await client.send(command)
  console.log('uploaded:', response.ETag)
}

// download file
async function download() {
  const command = new GetObjectCommand({
    Bucket: 'test-bucket',
    Key: 'hello.txt'
  })

  const response = await client.send(command)
  const body = await response.Body.transformToString()
  console.log('content:', body)
}
```

#### list objects

```typescript
import { S3Client, ListObjectsV2Command } from '@aws-sdk/client-s3'

async function listObjects() {
  const command = new ListObjectsV2Command({
    Bucket: 'test-bucket',
    Prefix: 'photos/',
    Delimiter: '/'
  })

  const response = await client.send(command)

  console.log('objects:')
  response.Contents?.forEach(obj => {
    console.log(`  ${obj.Key} (${obj.Size} bytes)`)
  })

  console.log('folders:')
  response.CommonPrefixes?.forEach(prefix => {
    console.log(`  ${prefix.Prefix}`)
  })
}
```

#### upload from buffer

```typescript
import { createReadStream } from 'fs'
import { S3Client, PutObjectCommand } from '@aws-sdk/client-s3'

async function uploadFile(filePath: string, key: string) {
  const fileStream = createReadStream(filePath)

  const command = new PutObjectCommand({
    Bucket: 'test-bucket',
    Key: key,
    Body: fileStream,
    ContentType: 'image/jpeg'
  })

  await client.send(command)
}
```

### python

#### basic operations

```python
import boto3

s3 = boto3.client(
    's3',
    endpoint_url='http://localhost:4040',
    aws_access_key_id='test',
    aws_secret_access_key='test',
    region_name='us-east-1'
)

# upload file
s3.put_object(
    Bucket='test-bucket',
    Key='hello.txt',
    Body=b'hello world',
    ContentType='text/plain'
)

# download file
response = s3.get_object(Bucket='test-bucket', Key='hello.txt')
content = response['Body'].read()
print(content.decode())

# list objects
response = s3.list_objects_v2(Bucket='test-bucket')
for obj in response.get('Contents', []):
    print(f"{obj['Key']} - {obj['Size']} bytes")

# delete object
s3.delete_object(Bucket='test-bucket', Key='hello.txt')
```

#### upload from file

```python
# upload file
with open('photo.jpg', 'rb') as f:
    s3.put_object(
        Bucket='test-bucket',
        Key='photos/photo.jpg',
        Body=f,
        ContentType='image/jpeg'
    )

# download to file
with open('downloaded.jpg', 'wb') as f:
    response = s3.get_object(Bucket='test-bucket', Key='photos/photo.jpg')
    f.write(response['Body'].read())
```

### rust

#### using aws-sdk-s3

```rust
use aws_sdk_s3::{Client, Config, config::{Credentials, Region}};
use aws_sdk_s3::primitives::ByteStream;

#[tokio::main]
async fn main() {
    let config = Config::builder()
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

    // upload
    client
        .put_object()
        .bucket("test-bucket")
        .key("hello.txt")
        .body(ByteStream::from_static(b"hello world"))
        .content_type("text/plain")
        .send()
        .await
        .unwrap();

    // download
    let response = client
        .get_object()
        .bucket("test-bucket")
        .key("hello.txt")
        .send()
        .await
        .unwrap();

    let body = response.body.collect().await.unwrap();
    println!("{}", String::from_utf8_lossy(&body.into_bytes()));

    // list
    let response = client
        .list_objects_v2()
        .bucket("test-bucket")
        .send()
        .await
        .unwrap();

    for object in response.contents().unwrap_or_default() {
        println!("{} - {} bytes", object.key().unwrap(), object.size());
    }
}
```

## curl examples

### basic operations

```bash
ENDPOINT="http://localhost:4040"
BUCKET="test-bucket"

# create bucket
curl -X PUT $ENDPOINT/$BUCKET

# upload text file
echo "hello world" > test.txt
curl -X PUT $ENDPOINT/$BUCKET/test.txt \
  -H "Content-Type: text/plain" \
  --data-binary @test.txt

# upload binary file
curl -X PUT $ENDPOINT/$BUCKET/photo.jpg \
  -H "Content-Type: image/jpeg" \
  --data-binary @photo.jpg

# download file
curl $ENDPOINT/$BUCKET/test.txt

# download to file
curl $ENDPOINT/$BUCKET/photo.jpg -o downloaded.jpg

# get object metadata
curl -I $ENDPOINT/$BUCKET/test.txt

# list objects
curl $ENDPOINT/$BUCKET

# list with prefix
curl "$ENDPOINT/$BUCKET?prefix=photos/"

# delete object
curl -X DELETE $ENDPOINT/$BUCKET/test.txt
```

### batch operations

```bash
# upload multiple files
for file in *.txt; do
  curl -X PUT $ENDPOINT/$BUCKET/$file \
    --data-binary @$file
done

# download multiple files
for key in file1.txt file2.txt file3.txt; do
  curl $ENDPOINT/$BUCKET/$key -o $key
done
```

## integration examples

### next.js image upload

```typescript
// app/api/upload/route.ts
import { S3Client, PutObjectCommand } from '@aws-sdk/client-s3'
import { NextRequest, NextResponse } from 'next/server'

const s3 = new S3Client({
  endpoint: process.env.S3_ENDPOINT,
  region: 'us-east-1',
  credentials: {
    accessKeyId: 'test',
    secretAccessKey: 'test'
  },
  forcePathStyle: true
})

export async function POST(request: NextRequest) {
  const formData = await request.formData()
  const file = formData.get('file') as File

  const buffer = await file.arrayBuffer()

  await s3.send(new PutObjectCommand({
    Bucket: 'uploads',
    Key: file.name,
    Body: new Uint8Array(buffer),
    ContentType: file.type
  }))

  return NextResponse.json({ success: true })
}
```

### express.js file server

```javascript
const express = require('express')
const multer = require('multer')
const { S3Client, PutObjectCommand, GetObjectCommand } = require('@aws-sdk/client-s3')

const app = express()
const upload = multer({ storage: multer.memoryStorage() })

const s3 = new S3Client({
  endpoint: 'http://localhost:4040',
  region: 'us-east-1',
  credentials: {
    accessKeyId: 'test',
    secretAccessKey: 'test'
  },
  forcePathStyle: true
})

// upload endpoint
app.post('/upload', upload.single('file'), async (req, res) => {
  await s3.send(new PutObjectCommand({
    Bucket: 'uploads',
    Key: req.file.originalname,
    Body: req.file.buffer,
    ContentType: req.file.mimetype
  }))

  res.json({ success: true })
})

// download endpoint
app.get('/download/:key', async (req, res) => {
  const response = await s3.send(new GetObjectCommand({
    Bucket: 'uploads',
    Key: req.params.key
  }))

  response.Body.pipe(res)
})

app.listen(3000)
```

### django file upload

```python
import boto3
from django.views import View
from django.http import JsonResponse

s3 = boto3.client(
    's3',
    endpoint_url='http://localhost:4040',
    aws_access_key_id='test',
    aws_secret_access_key='test'
)

class UploadView(View):
    def post(self, request):
        file = request.FILES['file']

        s3.put_object(
            Bucket='uploads',
            Key=file.name,
            Body=file.read(),
            ContentType=file.content_type
        )

        return JsonResponse({'success': True})
```

## testing examples

### jest integration test

```typescript
import { S3Client, PutObjectCommand, GetObjectCommand } from '@aws-sdk/client-s3'

describe('s3 integration', () => {
  const client = new S3Client({
    endpoint: 'http://localhost:4040',
    region: 'us-east-1',
    credentials: {
      accessKeyId: 'test',
      secretAccessKey: 'test'
    },
    forcePathStyle: true
  })

  test('upload and download', async () => {
    // upload
    await client.send(new PutObjectCommand({
      Bucket: 'test-bucket',
      Key: 'test.txt',
      Body: 'test content'
    }))

    // download
    const response = await client.send(new GetObjectCommand({
      Bucket: 'test-bucket',
      Key: 'test.txt'
    }))

    const content = await response.Body.transformToString()
    expect(content).toBe('test content')
  })
})
```

### pytest integration test

```python
import boto3
import pytest

@pytest.fixture
def s3_client():
    return boto3.client(
        's3',
        endpoint_url='http://localhost:4040',
        aws_access_key_id='test',
        aws_secret_access_key='test'
    )

def test_upload_download(s3_client):
    # upload
    s3_client.put_object(
        Bucket='test-bucket',
        Key='test.txt',
        Body=b'test content'
    )

    # download
    response = s3_client.get_object(
        Bucket='test-bucket',
        Key='test.txt'
    )

    content = response['Body'].read()
    assert content == b'test content'
```
