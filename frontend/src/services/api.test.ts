import { describe, it, expect } from 'vitest'
import api from './api'

describe('API Services', () => {
  it('should have correct base URL configured', () => {
    expect(api.defaults.baseURL).toBe('http://localhost:3000/api')
  })
})
