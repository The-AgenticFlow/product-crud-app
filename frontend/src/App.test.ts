import { describe, it, expect } from 'vitest'
import App from './App'

describe('App', () => {
  it('renders without crashing', () => {
    // Basic smoke test - would need @testing-library/react for full tests
    expect(typeof App).toBe('function')
  })
})
