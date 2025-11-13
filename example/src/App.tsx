import { useState } from 'react'
import './App.css'
import { Upload } from './upload'
import { FileList } from './filelist'

interface UploadedFile {
  name: string
  size: number
  uploadedAt: Date
}

function App() {
  const [files, setFiles] = useState<UploadedFile[]>([])

  const handleUploadComplete = (file: UploadedFile) => {
    setFiles(prev => [...prev, file])
  }

  const handleDelete = (filename: string) => {
    setFiles(prev => prev.filter(f => f.name !== filename))
  }

  return (
    <div style={{ padding: '20px', maxWidth: '1200px', margin: '0 auto' }}>
      <h1>six7 - s3 mock service</h1>
      <p style={{ color: '#888' }}>lightweight s3-compatible storage for local development</p>

      <Upload onUploadComplete={handleUploadComplete} />
      <FileList files={files} onDelete={handleDelete} />
    </div>
  )
}

export default App
