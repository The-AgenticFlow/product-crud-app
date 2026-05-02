import { describe, it, expect, vi } from 'vitest'
import { render, screen } from '@testing-library/react'
import { BrowserRouter } from 'react-router-dom'
import CreateProductPage from './CreateProductPage'

// Mock navigate
const mockNavigate = vi.fn()
vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual('react-router-dom')
  return {
    ...actual,
    useNavigate: () => mockNavigate,
  }
})

// Mock ProductForm component
vi.mock('../components/ProductForm', () => ({
  default: ({ mode, onSuccess }: { mode: string; onSuccess?: () => void }) => (
    <div data-testid="product-form">
      <span>Mode: {mode}</span>
      <button onClick={onSuccess} data-testid="success-button">
        Success
      </button>
    </div>
  ),
}))

describe('CreateProductPage', () => {
  it('renders create product heading', () => {
    render(
      <BrowserRouter>
        <CreateProductPage />
      </BrowserRouter>
    )

    expect(screen.getByText('Create Product')).toBeInTheDocument()
  })

  it('renders descriptive text', () => {
    render(
      <BrowserRouter>
        <CreateProductPage />
      </BrowserRouter>
    )

    expect(
      screen.getByText('Fill in the details below to add a new product to your inventory.')
    ).toBeInTheDocument()
  })

  it('renders ProductForm with create mode', () => {
    render(
      <BrowserRouter>
        <CreateProductPage />
      </BrowserRouter>
    )

    const formElement = screen.getByTestId('product-form')
    expect(formElement).toBeInTheDocument()
    expect(screen.getByText('Mode: create')).toBeInTheDocument()
  })

  it('navigates to /products on success', () => {
    render(
      <BrowserRouter>
        <CreateProductPage />
      </BrowserRouter>
    )

    const successButton = screen.getByTestId('success-button')
    successButton.click()

    expect(mockNavigate).toHaveBeenCalledWith('/products')
  })
})
