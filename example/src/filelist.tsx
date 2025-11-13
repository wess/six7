import { useState } from 'react'
import { DeleteObjectCommand } from '@aws-sdk/client-s3'
import { s3Client, BUCKET_NAME } from './s3client'

interface UploadedFile {
  name: string
  size: number
  uploadedAt: Date
}

interface FileListProps {
  files: UploadedFile[]
  onDelete: (filename: string) => void
}

export function FileList({ files, onDelete }: FileListProps) {
  const [error, setError] = useState<string | null>(null)
  const [deleting, setDeleting] = useState<string | null>(null)

  const handleDelete = async (filename: string) => {
    setDeleting(filename)
    setError(null)

    try {
      const command = new DeleteObjectCommand({
        Bucket: BUCKET_NAME,
        Key: filename,
      })

      await s3Client.send(command)
      onDelete(filename)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'failed to delete')
    } finally {
      setDeleting(null)
    }
  }

  const getFileUrl = (key: string) => {
    return `http://localhost:4040/${BUCKET_NAME}/${key}`
  }

  return (
    <div>
      <h2>files ({files.length})</h2>
      {error && <p style={{ color: 'red' }}>{error}</p>}
      {files.length === 0 ? (
        <p>no files uploaded yet</p>
      ) : (
        <table style={{ width: '100%', borderCollapse: 'collapse' }}>
          <thead>
            <tr style={{ borderBottom: '2px solid #ccc' }}>
              <th style={{ textAlign: 'left', padding: '10px' }}>preview</th>
              <th style={{ textAlign: 'left', padding: '10px' }}>name</th>
              <th style={{ textAlign: 'left', padding: '10px' }}>size</th>
              <th style={{ textAlign: 'left', padding: '10px' }}>uploaded</th>
              <th style={{ textAlign: 'left', padding: '10px' }}>actions</th>
            </tr>
          </thead>
          <tbody>
            {files.map((file) => (
              <tr key={file.name} style={{ borderBottom: '1px solid #eee' }}>
                <td style={{ padding: '10px' }}>
                  <img
                    src={getFileUrl(file.name)}
                    alt={file.name}
                    style={{ width: '50px', height: '50px', objectFit: 'cover' }}
                  />
                </td>
                <td style={{ padding: '10px' }}>{file.name}</td>
                <td style={{ padding: '10px' }}>{(file.size / 1024).toFixed(2)} KB</td>
                <td style={{ padding: '10px' }}>{file.uploadedAt.toLocaleString()}</td>
                <td style={{ padding: '10px' }}>
                  <button
                    onClick={() => handleDelete(file.name)}
                    disabled={deleting === file.name}
                    style={{
                      background: deleting === file.name ? '#ccc' : '#ff4444',
                      color: 'white',
                      border: 'none',
                      padding: '5px 10px',
                      cursor: deleting === file.name ? 'not-allowed' : 'pointer',
                      borderRadius: '3px',
                    }}
                  >
                    {deleting === file.name ? 'deleting...' : 'delete'}
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  )
}
