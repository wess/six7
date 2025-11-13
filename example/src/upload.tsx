import { useState } from 'react'
import { PutObjectCommand } from '@aws-sdk/client-s3'
import { s3Client, BUCKET_NAME } from './s3client'

interface UploadedFile {
  name: string
  size: number
  uploadedAt: Date
}

interface UploadProps {
  onUploadComplete: (file: UploadedFile) => void
}

export function Upload({ onUploadComplete }: UploadProps) {
  const [uploading, setUploading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (!file) return

    setUploading(true)
    setError(null)

    try {
      const arrayBuffer = await file.arrayBuffer()
      const body = new Uint8Array(arrayBuffer)

      const command = new PutObjectCommand({
        Bucket: BUCKET_NAME,
        Key: file.name,
        Body: body,
        ContentType: file.type,
      })

      await s3Client.send(command)
      onUploadComplete({
        name: file.name,
        size: file.size,
        uploadedAt: new Date(),
      })

      if (e.target) {
        e.target.value = ''
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'upload failed')
    } finally {
      setUploading(false)
    }
  }

  return (
    <div style={{ marginBottom: '20px' }}>
      <h2>upload file</h2>
      <input
        type="file"
        onChange={handleFileChange}
        disabled={uploading}
        accept="image/*"
      />
      {uploading && <p>uploading...</p>}
      {error && <p style={{ color: 'red' }}>{error}</p>}
    </div>
  )
}
