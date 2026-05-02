import { describe, it, expect, vi, beforeEach } from 'vitest'
import { productService } from './productService'
import api from './api'

vi.mock('./api', () => ({
  default: {
    get: vi.fn(),
    post: vi.fn(),
    put: vi.fn(),
    delete: vi.fn(),
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

  describe('listProducts', () => {
    it('should fetch paginated products with default parameters', async () => {
      const mockResponse = {
        data: [
          {
            id: '1',
            name: 'Product 1',
            price: 100,
            stock: 10,
          },
          {
            id: '2',
            name: 'Product 2',
            price: 200,
            stock: 20,
          },
        ],
        meta: {
          page: 1,
          limit: 10,
          totalItems: 2,
          totalPages: 1,
          hasNextPage: false,
          hasPrevPage: false,
        },
      }

      vi.mocked(api.get).mockResolvedValueOnce({ data: mockResponse })

      const result = await productService.listProducts()

      expect(api.get).toHaveBeenCalledWith('/products?')
      expect(result).toEqual(mockResponse)
    })

    it('should fetch products with query parameters', async () => {
      const mockResponse = {
        data: [],
        meta: {
          page: 2,
          limit: 5,
          totalItems: 10,
          totalPages: 2,
          hasNextPage: false,
          hasPrevPage: true,
        },
      }

      vi.mocked(api.get).mockResolvedValueOnce({ data: mockResponse })

      const result = await productService.listProducts({
        page: 2,
        limit: 5,
        search: 'test',
        category: 'Electronics',
        sort_by: 'price',
        sort_order: 'desc',
      })

      expect(api.get).toHaveBeenCalled()
      const calledUrl = vi.mocked(api.get).mock.calls[0][0]
      expect(calledUrl).toContain('page=2')
      expect(calledUrl).toContain('limit=5')
      expect(calledUrl).toContain('search=test')
      expect(calledUrl).toContain('category=Electronics')
      expect(calledUrl).toContain('sort_by=price')
      expect(calledUrl).toContain('sort_order=desc')
      expect(result).toEqual(mockResponse)
    })
  })

  describe('deleteProduct', () => {
    it('should delete a product via DELETE request', async () => {
      vi.mocked(api.delete).mockResolvedValueOnce({})

      await productService.deleteProduct('1')

      expect(api.delete).toHaveBeenCalledWith('/products/1')
    })
  })
})
