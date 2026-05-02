/**
 * Tests for Product API Service Functions
 */

import { describe, it, expect, vi, beforeEach } from 'vitest'
import apiClient from './api'
import {
  getProducts,
  getProduct,
  createProduct,
  updateProduct,
  deleteProduct,
} from './products'
import { Product } from '../types'

// Mock the apiClient
vi.mock('./api', () => ({
  default: {
    get: vi.fn(),
    post: vi.fn(),
    put: vi.fn(),
    delete: vi.fn(),
  },
}))

describe('Product API Service Functions', () => {
  const mockProduct: Product = {
    id: 'test-id-1',
    name: 'Test Product',
    description: 'A test product',
    price: 99.99,
    stock: 100,
    category: 'electronics',
    created_at: '2024-01-01T00:00:00Z',
    updated_at: '2024-01-01T00:00:00Z',
  }

  beforeEach(() => {
    // Reset all mocks before each test
    vi.clearAllMocks()
  })

  describe('getProducts', () => {
    it('should fetch paginated products without params', async () => {
      const mockResponse = {
        data: [mockProduct],
        pagination: {
          total: 1,
          page: 1,
          per_page: 10,
          total_pages: 1,
        },
      }

      vi.mocked(apiClient.get).mockResolvedValueOnce({ data: mockResponse })

      const result = await getProducts()

      expect(apiClient.get).toHaveBeenCalledWith('/api/products', {
        params: undefined,
      })
      expect(result).toEqual(mockResponse)
    })

    it('should fetch products with query parameters', async () => {
      const mockResponse = {
        data: [mockProduct],
        pagination: {
          total: 1,
          page: 1,
          per_page: 20,
          total_pages: 1,
        },
      }

      const params = {
        page: 1,
        per_page: 20,
        category: 'electronics',
        search: 'test',
        sort_by: 'name' as const,
        sort_order: 'asc' as const,
      }

      vi.mocked(apiClient.get).mockResolvedValueOnce({ data: mockResponse })

      const result = await getProducts(params)

      expect(apiClient.get).toHaveBeenCalledWith('/api/products', {
        params,
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('getProduct', () => {
    it('should fetch a single product by ID', async () => {
      vi.mocked(apiClient.get).mockResolvedValueOnce({ data: mockProduct })

      const result = await getProduct('test-id-1')

      expect(apiClient.get).toHaveBeenCalledWith('/api/products/test-id-1')
      expect(result).toEqual(mockProduct)
    })
  })

  describe('createProduct', () => {
    it('should create a new product', async () => {
      const createData = {
        name: 'New Product',
        description: 'A new product',
        price: 49.99,
        stock: 50,
        category: 'books',
      }

      const createdProduct = {
        ...createData,
        id: 'new-id',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-01T00:00:00Z',
      }

      vi.mocked(apiClient.post).mockResolvedValueOnce({ data: createdProduct })

      const result = await createProduct(createData)

      expect(apiClient.post).toHaveBeenCalledWith('/api/products', createData)
      expect(result).toEqual(createdProduct)
    })
  })

  describe('updateProduct', () => {
    it('should update an existing product with partial data', async () => {
      const updateData = {
        price: 79.99,
        stock: 75,
      }

      const updatedProduct = {
        ...mockProduct,
        ...updateData,
      }

      vi.mocked(apiClient.put).mockResolvedValueOnce({ data: updatedProduct })

      const result = await updateProduct('test-id-1', updateData)

      expect(apiClient.put).toHaveBeenCalledWith('/api/products/test-id-1', updateData)
      expect(result).toEqual(updatedProduct)
    })
  })

  describe('deleteProduct', () => {
    it('should delete a product by ID', async () => {
      vi.mocked(apiClient.delete).mockResolvedValueOnce({ data: undefined })

      await deleteProduct('test-id-1')

      expect(apiClient.delete).toHaveBeenCalledWith('/api/products/test-id-1')
    })

    it('should return void when deletion succeeds', async () => {
      vi.mocked(apiClient.delete).mockResolvedValueOnce({ data: undefined })

      const result = await deleteProduct('test-id-1')

      expect(result).toBeUndefined()
    })
  })
})
