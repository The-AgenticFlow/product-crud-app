import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen } from '@testing-library/react'
import { MemoryRouter, Routes, Route } from 'react-router-dom'
import { QueryClient, QueryClientProvider, UseQueryResult } from '@tanstack/react-query'
import ProductDetailPage from './ProductDetailPage'
import * as useProductsHook from '../hooks/useProducts'
import { Product } from '../types'

// Mock the hooks
vi.mock('../hooks/useProducts')

// Helper function to create complete mock query results
const createMockQueryResult = (
  overrides: Partial<UseQueryResult<Product, Error>> = {}
): UseQueryResult<Product, Error> => {
  return {
    data: undefined,
    error: null,
    isError: false,
    isPending: false,
    isLoading: false,
    isFetching: false,
    isSuccess: true,
    dataUpdatedAt: 0,
    errorUpdatedAt: 0,
    failureCount: 0,
    failureReason: null,
    errorUpdateCount: 0,
    isRefetching: false,
    isLoadingError: false,
    isRefetchError: false,
    promise: Promise.resolve(null as unknown as Product),
    refetch: vi.fn(),
    status: 'success',
    fetchStatus: 'idle',
    ...overrides,
  } as UseQueryResult<Product, Error>
}

const createWrapper = (initialRoute = '/products/1') => {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: { retry: false },
    },
  })
  return ({ children }: { children: React.ReactNode }) => (
    <QueryClientProvider client={queryClient}>
      <MemoryRouter initialEntries={[initialRoute]}>
        <Routes>
          <Route path="/products/:id" element={children} />
        </Routes>
      </MemoryRouter>
    </QueryClientProvider>
  )
}

describe('ProductDetailPage', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders loading state', () => {
    vi.mocked(useProductsHook.useProduct).mockReturnValue(
      createMockQueryResult({ isLoading: true, isPending: true })
    )

    render(<ProductDetailPage />, { wrapper: createWrapper() })

    // Should show a spinner (loading state)
    const spinner = document.querySelector('.animate-spin')
    expect(spinner).toBeInTheDocument()
  })

  it('renders error state', () => {
    vi.mocked(useProductsHook.useProduct).mockReturnValue(
      createMockQueryResult({
        isError: true,
        error: new Error('Failed to load product'),
      })
    )

    render(<ProductDetailPage />, { wrapper: createWrapper() })

    expect(screen.getByText('Error Loading Product')).toBeInTheDocument()
    expect(screen.getByText('Failed to load product')).toBeInTheDocument()
  })

  it('renders not found state for 404 errors', () => {
    vi.mocked(useProductsHook.useProduct).mockReturnValue(
      createMockQueryResult({
        isError: true,
        error: new Error('404 Not Found'),
      })
    )

    render(<ProductDetailPage />, { wrapper: createWrapper() })

    expect(screen.getByText('Product Not Found')).toBeInTheDocument()
    expect(screen.getByText(/doesn't exist or has been deleted/i)).toBeInTheDocument()
  })

  it('renders product details when loaded', async () => {
    const mockProduct: Product = {
      id: '1',
      name: 'Test Product',
      description: 'A test product description',
      price: 99.99,
      stock: 10,
      category: 'Electronics',
      imageUrl: 'https://example.com/image.jpg',
      created_at: '2026-01-01T00:00:00Z',
      updated_at: '2026-01-02T00:00:00Z',
    }

    vi.mocked(useProductsHook.useProduct).mockReturnValue(
      createMockQueryResult({
        data: mockProduct,
        isSuccess: true,
      })
    )

    render(<ProductDetailPage />, { wrapper: createWrapper() })

    // Check product details are displayed
    expect(screen.getByText('Test Product')).toBeInTheDocument()
    expect(screen.getByText('A test product description')).toBeInTheDocument()
    expect(screen.getByText('$99.99')).toBeInTheDocument()

    // Check back button
    expect(screen.getByText('Back to Products')).toBeInTheDocument()
  })

  it('renders back to products link', async () => {
    const mockProduct: Product = {
      id: '1',
      name: 'Test Product',
      description: 'A test product description',
      price: 99.99,
      stock: 10,
      category: 'Electronics',
      imageUrl: 'https://example.com/image.jpg',
      created_at: '2026-01-01T00:00:00Z',
      updated_at: '2026-01-02T00:00:00Z',
    }

    vi.mocked(useProductsHook.useProduct).mockReturnValue(
      createMockQueryResult({
        data: mockProduct,
        isSuccess: true,
      })
    )

    render(<ProductDetailPage />, { wrapper: createWrapper() })

    const backLink = screen.getByRole('link', { name: /back to products/i })
    expect(backLink).toBeInTheDocument()
    expect(backLink).toHaveAttribute('href', '/products')
  })

  it('extracts product id from route params', () => {
    vi.mocked(useProductsHook.useProduct).mockReturnValue(
      createMockQueryResult({ isLoading: true })
    )

    render(<ProductDetailPage />, { wrapper: createWrapper('/products/abc123') })

    expect(useProductsHook.useProduct).toHaveBeenCalledWith('abc123')
  })
})
