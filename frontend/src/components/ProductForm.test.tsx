import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { MemoryRouter } from 'react-router-dom'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import ProductForm from './ProductForm'
import * as useProductsHook from '../hooks/useProducts'

// Mock the hooks
vi.mock('../hooks/useProducts')

const createWrapper = () => {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: { retry: false },
      mutations: { retry: false },
    },
  })
  return ({ children }: { children: React.ReactNode }) => (
    <QueryClientProvider client={queryClient}>
      <MemoryRouter>
        {children}
      </MemoryRouter>
    </QueryClientProvider>
  )
}

describe('ProductForm', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders form in create mode', () => {
    const mockCreateProduct = vi.fn()
    vi.mocked(useProductsHook.useCreateProduct).mockReturnValue({
      mutateAsync: mockCreateProduct,
      isPending: false,
    } as any)
    vi.mocked(useProductsHook.useUpdateProduct).mockReturnValue({
      mutateAsync: vi.fn(),
      isPending: false,
    } as any)

    render(<ProductForm mode="create" />, { wrapper: createWrapper() })

    expect(screen.getByLabelText(/name/i)).toBeInTheDocument()
    expect(screen.getByLabelText(/description/i)).toBeInTheDocument()
    expect(screen.getByLabelText(/price/i)).toBeInTheDocument()
    expect(screen.getByLabelText(/stock/i)).toBeInTheDocument()
    expect(screen.getByLabelText(/category/i)).toBeInTheDocument()
    expect(screen.getByLabelText(/image url/i)).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /create product/i })).toBeInTheDocument()
  })

  it('renders form in edit mode with initial data', () => {
    const mockUpdateProduct = vi.fn()
    vi.mocked(useProductsHook.useCreateProduct).mockReturnValue({
      mutateAsync: vi.fn(),
      isPending: false,
    } as any)
    vi.mocked(useProductsHook.useUpdateProduct).mockReturnValue({
      mutateAsync: mockUpdateProduct,
      isPending: false,
    } as any)

    const initialData = {
      id: '1',
      name: 'Test Product',
      description: 'Test Description',
      price: 99.99,
      stock: 10,
      category: 'electronics',
      imageUrl: 'https://example.com/image.jpg',
    }

    render(<ProductForm mode="edit" productId="1" initialData={initialData} />, { wrapper: createWrapper() })

    expect(screen.getByDisplayValue('Test Product')).toBeInTheDocument()
    expect(screen.getByDisplayValue('Test Description')).toBeInTheDocument()
    expect(screen.getByDisplayValue('99.99')).toBeInTheDocument()
    expect(screen.getByDisplayValue('10')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /update product/i })).toBeInTheDocument()
  })

  it('displays validation errors for required fields', async () => {
    const mockCreateProduct = vi.fn()
    vi.mocked(useProductsHook.useCreateProduct).mockReturnValue({
      mutateAsync: mockCreateProduct,
      isPending: false,
    } as any)
    vi.mocked(useProductsHook.useUpdateProduct).mockReturnValue({
      mutateAsync: vi.fn(),
      isPending: false,
    } as any)

    render(<ProductForm mode="create" />, { wrapper: createWrapper() })

    const submitButton = screen.getByRole('button', { name: /create product/i })
    fireEvent.click(submitButton)

    await waitFor(() => {
      expect(screen.getByText(/name is required/i)).toBeInTheDocument()
    })

    expect(mockCreateProduct).not.toHaveBeenCalled()
  })

  it('displays validation error for name too long', async () => {
    const mockCreateProduct = vi.fn()
    vi.mocked(useProductsHook.useCreateProduct).mockReturnValue({
      mutateAsync: mockCreateProduct,
      isPending: false,
    } as any)
    vi.mocked(useProductsHook.useUpdateProduct).mockReturnValue({
      mutateAsync: vi.fn(),
      isPending: false,
    } as any)

    render(<ProductForm mode="create" />, { wrapper: createWrapper() })

    const nameInput = screen.getByLabelText(/name/i)
    const longName = 'a'.repeat(256) // 256 characters, exceeds max of 255
    fireEvent.change(nameInput, { target: { value: longName } })

    const priceInput = screen.getByLabelText(/price/i)
    fireEvent.change(priceInput, { target: { value: '10' } })

    const stockInput = screen.getByLabelText(/stock/i)
    fireEvent.change(stockInput, { target: { value: '5' } })

    const submitButton = screen.getByRole('button', { name: /create product/i })
    fireEvent.click(submitButton)

    await waitFor(() => {
      expect(screen.getByText(/name must be 255 characters or less/i)).toBeInTheDocument()
    })
  })

  it('displays validation error for invalid URL', async () => {
    const mockCreateProduct = vi.fn()
    vi.mocked(useProductsHook.useCreateProduct).mockReturnValue({
      mutateAsync: mockCreateProduct,
      isPending: false,
    } as any)
    vi.mocked(useProductsHook.useUpdateProduct).mockReturnValue({
      mutateAsync: vi.fn(),
      isPending: false,
    } as any)

    render(<ProductForm mode="create" />, { wrapper: createWrapper() })

    // Fill required fields first
    const nameInput = screen.getByLabelText(/name/i)
    fireEvent.change(nameInput, { target: { value: 'Test Product' } })

    const priceInput = screen.getByLabelText(/price/i)
    fireEvent.change(priceInput, { target: { value: '10' } })

    const stockInput = screen.getByLabelText(/stock/i)
    fireEvent.change(stockInput, { target: { value: '5' } })

    const imageUrlInput = screen.getByLabelText(/image url/i)
    fireEvent.change(imageUrlInput, { target: { value: 'invalid-url' } })

    const submitButton = screen.getByRole('button', { name: /create product/i })
    fireEvent.click(submitButton)

    await waitFor(() => {
      expect(screen.getByText(/must be a valid url/i)).toBeInTheDocument()
    })
  })

  it('calls createProduct mutation on form submission', async () => {
    const mockCreateProduct = vi.fn().mockResolvedValue({ id: '1' })
    vi.mocked(useProductsHook.useCreateProduct).mockReturnValue({
      mutateAsync: mockCreateProduct,
      isPending: false,
    } as any)
    vi.mocked(useProductsHook.useUpdateProduct).mockReturnValue({
      mutateAsync: vi.fn(),
      isPending: false,
    } as any)

    render(<ProductForm mode="create" />, { wrapper: createWrapper() })

    const nameInput = screen.getByLabelText(/name/i)
    fireEvent.change(nameInput, { target: { value: 'New Product' } })

    const priceInput = screen.getByLabelText(/price/i)
    fireEvent.change(priceInput, { target: { value: '50' } })

    const stockInput = screen.getByLabelText(/stock/i)
    fireEvent.change(stockInput, { target: { value: '20' } })

    const submitButton = screen.getByRole('button', { name: /create product/i })
    fireEvent.click(submitButton)

    await waitFor(() => {
      expect(mockCreateProduct).toHaveBeenCalledWith({
        name: 'New Product',
        description: '',
        price: 50,
        stock: 20,
        category: '',
        imageUrl: '',
      })
    })
  })

  it('calls updateProduct mutation on form submission in edit mode', async () => {
    const mockUpdateProduct = vi.fn().mockResolvedValue({ id: '1' })
    vi.mocked(useProductsHook.useCreateProduct).mockReturnValue({
      mutateAsync: vi.fn(),
      isPending: false,
    } as any)
    vi.mocked(useProductsHook.useUpdateProduct).mockReturnValue({
      mutateAsync: mockUpdateProduct,
      isPending: false,
    } as any)

    const initialData = {
      id: '1',
      name: 'Test Product',
      description: 'Test Description',
      price: 99.99,
      stock: 10,
      category: 'electronics',
      imageUrl: 'https://example.com/image.jpg',
    }

    render(<ProductForm mode="edit" productId="1" initialData={initialData} />, { wrapper: createWrapper() })

    const nameInput = screen.getByDisplayValue('Test Product')
    fireEvent.change(nameInput, { target: { value: 'Updated Product' } })

    const submitButton = screen.getByRole('button', { name: /update product/i })
    fireEvent.click(submitButton)

    await waitFor(() => {
      expect(mockUpdateProduct).toHaveBeenCalledWith({
        name: 'Updated Product',
        description: 'Test Description',
        price: 99.99,
        stock: 10,
        category: 'electronics',
        imageUrl: 'https://example.com/image.jpg',
      })
    })
  })

  it('disables submit button during submission', () => {
    const mockCreateProduct = vi.fn()
    vi.mocked(useProductsHook.useCreateProduct).mockReturnValue({
      mutateAsync: mockCreateProduct,
      isPending: true,
    } as any)
    vi.mocked(useProductsHook.useUpdateProduct).mockReturnValue({
      mutateAsync: vi.fn(),
      isPending: false,
    } as any)

    render(<ProductForm mode="create" />, { wrapper: createWrapper() })

    const submitButton = screen.getByRole('button', { name: /saving/i })
    expect(submitButton).toBeDisabled()
  })

  it('renders cancel button', () => {
    const mockCreateProduct = vi.fn()
    vi.mocked(useProductsHook.useCreateProduct).mockReturnValue({
      mutateAsync: mockCreateProduct,
      isPending: false,
    } as any)
    vi.mocked(useProductsHook.useUpdateProduct).mockReturnValue({
      mutateAsync: vi.fn(),
      isPending: false,
    } as any)

    render(<ProductForm mode="create" />, { wrapper: createWrapper() })

    const cancelButton = screen.getByRole('button', { name: /cancel/i })
    expect(cancelButton).toBeInTheDocument()
  })
})
