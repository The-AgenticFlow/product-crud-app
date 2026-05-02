/**
 * Tests for API types
 * Verifies that type definitions compile correctly and are exported
 */

import { describe, it, expect } from 'vitest'
import type {
  PaginatedResponse,
  ApiErrorResponse,
  GetProductsParams,
  ProductCreateRequest,
  ProductUpdateRequest,
} from './types'
import type { Product } from '../types'

describe('API Types', () => {
  describe('PaginatedResponse', () => {
    it('should accept valid paginated response structure', () => {
      const response: PaginatedResponse<Product> = {
        data: [
          {
            id: '1',
            name: 'Test Product',
            description: 'Test Description',
            price: 99.99,
            stock: 10,
            category: 'Electronics',
            created_at: '2024-01-01T00:00:00Z',
            updated_at: '2024-01-01T00:00:00Z',
          },
        ],
        pagination: {
          total: 1,
          page: 1,
          per_page: 10,
          total_pages: 1,
        },
      }

      expect(response.data).toHaveLength(1)
      expect(response.pagination.total).toBe(1)
    })
  })

  describe('ApiErrorResponse', () => {
    it('should accept valid error response structure', () => {
      const error: ApiErrorResponse = {
        error: 'ValidationError',
        message: 'Invalid input data',
        statusCode: 400,
      }

      expect(error.statusCode).toBe(400)
      expect(error.error).toBe('ValidationError')
    })

    it('should support optional details field', () => {
      const error: ApiErrorResponse = {
        error: 'ValidationError',
        message: 'Invalid input',
        statusCode: 400,
        details: {
          field: 'price',
          reason: 'must be positive',
        },
      }

      expect(error.details).toBeDefined()
    })
  })

  describe('GetProductsParams', () => {
    it('should accept valid query parameters', () => {
      const params: GetProductsParams = {
        page: 1,
        per_page: 20,
        category: 'Electronics',
        search: 'laptop',
        sort_by: 'price',
        sort_order: 'asc',
      }

      expect(params.page).toBe(1)
      expect(params.category).toBe('Electronics')
    })

    it('should allow optional parameters', () => {
      const params: GetProductsParams = {}

      expect(params.page).toBeUndefined()
    })
  })

  describe('ProductCreateRequest', () => {
    it('should accept valid create request', () => {
      const request: ProductCreateRequest = {
        name: 'New Product',
        description: 'Description',
        price: 49.99,
        stock: 100,
        category: 'Books',
      }

      expect(request.name).toBe('New Product')
      expect(request.price).toBe(49.99)
    })
  })

  describe('ProductUpdateRequest', () => {
    it('should accept partial updates', () => {
      const request: ProductUpdateRequest = {
        price: 59.99,
        stock: 50,
      }

      expect(request.price).toBe(59.99)
      expect(request.name).toBeUndefined()
    })
  })

  describe('Product interface', () => {
    it('should include timestamp fields', () => {
      const product: Product = {
        id: '1',
        name: 'Product',
        description: 'Description',
        price: 29.99,
        stock: 25,
        category: 'Games',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-02T00:00:00Z',
      }

      expect(product.created_at).toBeDefined()
      expect(product.updated_at).toBeDefined()
    })
  })
})
