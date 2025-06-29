import { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

function App() {
  const [count, setCount] = useState(0)
  const [rustMessage, setRustMessage] = useState<string>('')
  const [isLoading, setIsLoading] = useState(false)

  const testRustCommunication = async () => {
    setIsLoading(true)
    try {
      const message = await invoke<string>('hello_from_rust')
      setRustMessage(message)
    } catch (error) {
      setRustMessage(`Error: ${error}`)
      console.error('Failed to call Rust function:', error)
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>OrgDesk Development</h1>
      
      {/* Tauri Communication Test */}
      <div className="card">
        <h2>🔗 React ↔ Rust Communication Test</h2>
        <button 
          onClick={testRustCommunication}
          disabled={isLoading}
          style={{ 
            backgroundColor: '#4CAF50', 
            color: 'white',
            padding: '10px 20px',
            fontSize: '16px',
            margin: '10px'
          }}
        >
          {isLoading ? 'Testing...' : 'Test Rust Backend'}
        </button>
        {rustMessage && (
          <div style={{ 
            marginTop: '10px', 
            padding: '10px', 
            backgroundColor: rustMessage.includes('Error') ? '#ffebee' : '#e8f5e8',
            border: `1px solid ${rustMessage.includes('Error') ? '#f44336' : '#4CAF50'}`,
            borderRadius: '4px'
          }}>
            <strong>Backend Response:</strong> {rustMessage}
          </div>
        )}
      </div>

      {/* Original Vite Counter */}
      <div className="card">
        <h3>React Counter Test</h3>
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      
      <p className="read-the-docs">
        🚀 Testing Tauri setup for OrgDesk development
      </p>
    </>
  )
}

export default App
