import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, waitFor } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactNode } from 'react'
import { useProducts, useDeleteProduct } from './useProducts'
import { productService } from '../services/productService'

vi.mock('../services/productService', () => ({
  productService: {
    listProducts: vi.fn(),
    deleteProduct: vi.fn(),
  },
}))

const createWrapper = () => {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
      },
    },
  })

  return ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  )
}

describe('useProducts', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should fetch products with no query parameters', async () => {
    const mockResponse = {
      data: [
        {
          id: '1',
          name: 'Product 1',
          price: 100,
          stock: 10,
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
        },
      ],
      meta: {
        page: 1,
        limit: 10,
        totalItems: 1,
        totalPages: 1,
        hasNextPage: false,
        hasPrevPage: false,
      },
    }

    vi.mocked(productService.listProducts).mockResolvedValueOnce(mockResponse)

    const { result } = renderHook(() => useProducts(), {
      wrapper: createWrapper(),
    })

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true)
    })

    expect(result.current.data).toEqual(mockResponse)
    expect(productService.listProducts).toHaveBeenCalledWith(undefined)
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

    vi.mocked(productService.listProducts).mockResolvedValueOnce(mockResponse)

    const query = {
      page: 2,
      limit: 5,
      search: 'test',
      category: 'Electronics',
      sort_by: 'price' as const,
      sort_order: 'desc' as const,
    }

    const { result } = renderHook(() => useProducts(query), {
      wrapper: createWrapper(),
    })

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true)
    })

    expect(result.current.data).toEqual(mockResponse)
    expect(productService.listProducts).toHaveBeenCalledWith(query)
  })
})

describe('useDeleteProduct', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should delete a product', async () => {
    vi.mocked(productService.deleteProduct).mockResolvedValueOnce(undefined)

    const { result } = renderHook(() => useDeleteProduct(), {
      wrapper: createWrapper(),
    })

    await waitFor(() => {
      expect(result.current.mutateAsync).toBeDefined()
    })

    await result.current.mutateAsync('1')

    expect(productService.deleteProduct).toHaveBeenCalledWith('1')
  })
})
