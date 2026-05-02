import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'

describe('ProductsPage', () => {
  it('renders products heading', async () => {
    const { default: ProductsPage } = await import('./ProductsPage')
    render(<ProductsPage />)

    expect(screen.getByText('Products')).toBeInTheDocument()
  })

  it('renders add product button', async () => {
    const { default: ProductsPage } = await import('./ProductsPage')
    render(<ProductsPage />)

    expect(screen.getByRole('button', { name: /add product/i })).toBeInTheDocument()
  })

  it('renders empty state when no products', async () => {
    const { default: ProductsPage } = await import('./ProductsPage')
    render(<ProductsPage />)

    expect(screen.getByText('No products yet')).toBeInTheDocument()
    expect(screen.getByText('Click "Add Product" to create your first product')).toBeInTheDocument()
  })
})
