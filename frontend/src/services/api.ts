import axios, { AxiosError, AxiosInstance, InternalAxiosRequestConfig } from 'axios'

// Create axios instance with default configuration
const apiClient: AxiosInstance = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000',
  headers: {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
  },
  timeout: 10000, // 10 seconds timeout
})

// Request interceptor for logging (optional but useful for debugging)
apiClient.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    // Log request details in development
    if (import.meta.env.DEV) {
      console.log(`[API Request] ${config.method?.toUpperCase()} ${config.url}`, config.data)
    }
    return config
  },
  (error: AxiosError) => {
    // Handle request errors
    if (import.meta.env.DEV) {
      console.error('[API Request Error]', error)
    }
    return Promise.reject(error)
  }
)

// Response interceptor for error handling
apiClient.interceptors.response.use(
  (response) => {
    // Log response in development
    if (import.meta.env.DEV) {
      console.log(`[API Response] ${response.config.method?.toUpperCase()} ${response.config.url}`, response.data)
    }
    return response
  },
  (error: AxiosError) => {
    // Handle different error scenarios
    if (error.response) {
      // Server responded with error status (4xx, 5xx)
      const { status, data } = error.response

      if (import.meta.env.DEV) {
        console.error(`[API Error] Status: ${status}`, data)
      }

      // You can add specific error handling here
      // For example, redirect to login on 401
      if (status === 401) {
        // Handle unauthorized
        console.warn('Unauthorized - authentication required')
      } else if (status === 403) {
        // Handle forbidden
        console.warn('Forbidden - insufficient permissions')
      } else if (status === 404) {
        // Handle not found
        console.warn('Resource not found')
      } else if (status >= 500) {
        // Handle server errors
        console.error('Server error occurred')
      }
    } else if (error.request) {
      // Request was made but no response received (network error)
      if (import.meta.env.DEV) {
        console.error('[API Network Error]', error.message)
      }
    } else {
      // Something else happened while setting up the request
      if (import.meta.env.DEV) {
        console.error('[API Setup Error]', error.message)
      }
    }

    return Promise.reject(error)
  }
)

export default apiClient
