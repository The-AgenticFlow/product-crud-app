import { describe, it, expect, vi, beforeEach } from 'vitest'
import { productService } from './productService'
import api from './api'

vi.mock('./api', () => ({
  default: {
    get: vi.fn(),
    post: vi.fn(),
    put: vi.fn(),
  },
}))

describe('productService', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  describe('createProduct', () => {
    it('should create a product via POST request', async () => {
      const mockProduct = {
        id: '1',
        name: 'Test Product',
        description: 'Test Description',
        price: 100,
        stock: 10,
        category: 'Electronics',
      }

      vi.mocked(api.post).mockResolvedValueOnce({ data: mockProduct })

      const result = await productService.createProduct({
        name: 'Test Product',
        description: 'Test Description',
        price: 100,
        stock: 10,
        category: 'Electronics',
      })

      expect(api.post).toHaveBeenCalledWith('/products', {
        name: 'Test Product',
        description: 'Test Description',
        price: 100,
        stock: 10,
        category: 'Electronics',
      })
      expect(result).toEqual(mockProduct)
    })
  })

  describe('updateProduct', () => {
    it('should update a product via PUT request', async () => {
      const mockProduct = {
        id: '1',
        name: 'Updated Product',
        description: 'Updated Description',
        price: 150,
        stock: 20,
        category: 'Electronics',
      }

      vi.mocked(api.put).mockResolvedValueOnce({ data: mockProduct })

      const result = await productService.updateProduct('1', {
        name: 'Updated Product',
        price: 150,
      })

      expect(api.put).toHaveBeenCalledWith('/products/1', {
        name: 'Updated Product',
        price: 150,
      })
      expect(result).toEqual(mockProduct)
    })
  })

  describe('getProduct', () => {
    it('should fetch a product by ID via GET request', async () => {
      const mockProduct = {
        id: '1',
        name: 'Test Product',
        description: 'Test Description',
        price: 100,
        stock: 10,
        category: 'Electronics',
      }

      vi.mocked(api.get).mockResolvedValueOnce({ data: mockProduct })

      const result = await productService.getProduct('1')

      expect(api.get).toHaveBeenCalledWith('/products/1')
      expect(result).toEqual(mockProduct)
    })
  })
})
