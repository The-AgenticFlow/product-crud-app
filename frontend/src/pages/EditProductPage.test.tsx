import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen } from '@testing-library/react'
import { MemoryRouter, Route, Routes } from 'react-router-dom'
import EditProductPage from './EditProductPage'

// Mock navigate
const mockNavigate = vi.fn()
vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual('react-router-dom')
  return {
    ...actual,
    useNavigate: () => mockNavigate,
  }
})

// Mock useProduct hook
const mockProduct = {
  id: '1',
  name: 'Test Product',
  description: 'Test Description',
  price: 99.99,
  stock: 10,
  category: 'electronics',
  imageUrl: 'https://example.com/image.jpg',
  createdAt: '2024-01-01T00:00:00.000Z',
  updatedAt: '2024-01-01T00:00:00.000Z',
}

vi.mock('../hooks/useProducts', () => ({
  useProduct: (id: string) => {
    if (id === '1') {
      return {
        data: mockProduct,
        isLoading: false,
        error: null,
      }
    }
    if (id === 'error') {
      return {
        data: null,
        isLoading: false,
        error: new Error('Product not found'),
      }
    }
    return {
      data: null,
      isLoading: true,
      error: null,
    }
  },
}))

// Mock ProductForm component
vi.mock('../components/ProductForm', () => ({
  default: ({
    mode,
    productId,
    initialData,
    onSuccess,
  }: {
    mode: string
    productId?: string
    initialData?: typeof mockProduct
    onSuccess?: () => void
  }) => (
    <div data-testid="product-form">
      <span>Mode: {mode}</span>
      <span>Product ID: {productId}</span>
      <span>Initial Data: {initialData?.name}</span>
      <button onClick={onSuccess} data-testid="success-button">
        Success
      </button>
    </div>
  ),
}))

describe('EditProductPage', () => {
  beforeEach(() => {
    mockNavigate.mockClear()
  })

  it('renders loading state when product is loading', () => {
    render(
      <MemoryRouter initialEntries={['/products/2/edit']}>
        <Routes>
          <Route path="/products/:id/edit" element={<EditProductPage />} />
        </Routes>
      </MemoryRouter>
    )

    expect(screen.getByText('Loading product...')).toBeInTheDocument()
  })

  it('renders error state when product fails to load', () => {
    render(
      <MemoryRouter initialEntries={['/products/error/edit']}>
        <Routes>
          <Route path="/products/:id/edit" element={<EditProductPage />} />
        </Routes>
      </MemoryRouter>
    )

    expect(screen.getByText('Error loading product.')).toBeInTheDocument()
    expect(screen.getByText('Back to Products')).toBeInTheDocument()
  })

  it('renders edit product heading when product is loaded', () => {
    render(
      <MemoryRouter initialEntries={['/products/1/edit']}>
        <Routes>
          <Route path="/products/:id/edit" element={<EditProductPage />} />
        </Routes>
      </MemoryRouter>
    )

    expect(screen.getByText('Edit Product')).toBeInTheDocument()
  })

  it('renders ProductForm with edit mode and initial data', () => {
    render(
      <MemoryRouter initialEntries={['/products/1/edit']}>
        <Routes>
          <Route path="/products/:id/edit" element={<EditProductPage />} />
        </Routes>
      </MemoryRouter>
    )

    const formElement = screen.getByTestId('product-form')
    expect(formElement).toBeInTheDocument()
    expect(screen.getByText('Mode: edit')).toBeInTheDocument()
    expect(screen.getByText('Product ID: 1')).toBeInTheDocument()
    expect(screen.getByText('Initial Data: Test Product')).toBeInTheDocument()
  })

  it('renders descriptive text with product name', () => {
    render(
      <MemoryRouter initialEntries={['/products/1/edit']}>
        <Routes>
          <Route path="/products/:id/edit" element={<EditProductPage />} />
        </Routes>
      </MemoryRouter>
    )

    expect(
      screen.getByText('Update the details for Test Product.')
    ).toBeInTheDocument()
  })

  it('navigates to /products on success', () => {
    render(
      <MemoryRouter initialEntries={['/products/1/edit']}>
        <Routes>
          <Route path="/products/:id/edit" element={<EditProductPage />} />
        </Routes>
      </MemoryRouter>
    )

    const successButton = screen.getByTestId('success-button')
    successButton.click()

    expect(mockNavigate).toHaveBeenCalledWith('/products')
  })

  it('navigates back to products when clicking Back button on error', () => {
    render(
      <MemoryRouter initialEntries={['/products/error/edit']}>
        <Routes>
          <Route path="/products/:id/edit" element={<EditProductPage />} />
        </Routes>
      </MemoryRouter>
    )

    const backButton = screen.getByText('Back to Products')
    backButton.click()

    expect(mockNavigate).toHaveBeenCalledWith('/products')
  })
})
