import { describe, it, expect, vi, beforeEach } from 'vitest'

// Mock axios
const mockAxiosInstance = {
  interceptors: {
    request: {
      use: vi.fn(),
    },
    response: {
      use: vi.fn(),
    },
  },
  get: vi.fn(),
  post: vi.fn(),
  put: vi.fn(),
  delete: vi.fn(),
}

vi.mock('axios', () => ({
  default: {
    create: vi.fn(() => mockAxiosInstance),
  },
}))

describe('API Client Configuration', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should create axios instance with correct base URL', async () => {
    // Import the module which will trigger axios.create
    const axios = await import('axios')
    await import('./api')

    expect(axios.default.create).toHaveBeenCalled()

    // Get the config passed to axios.create
    const createCall = vi.mocked(axios.default.create).mock.calls[0]
    const config = createCall?.[0]

    expect(config).toMatchObject({
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
      timeout: 10000,
    })

    // Should use environment variable or fallback
    expect(config?.baseURL).toBeDefined()
  })

  it('should setup request interceptor', async () => {
    // Clear module cache and reimport
    vi.resetModules()
    await import('./api')

    // The request interceptor should be registered
    expect(mockAxiosInstance.interceptors.request.use).toHaveBeenCalled()
  })

  it('should setup response interceptor', async () => {
    // Clear module cache and reimport
    vi.resetModules()
    await import('./api')

    // The response interceptor should be registered
    expect(mockAxiosInstance.interceptors.response.use).toHaveBeenCalled()
  })
})
