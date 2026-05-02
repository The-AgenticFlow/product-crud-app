import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import { BrowserRouter } from 'react-router-dom'

describe('ProductsPage', () => {
  it('renders products heading', async () => {
    const { default: ProductsPage } = await import('./ProductsPage')
    render(
      <BrowserRouter>
        <ProductsPage />
      </BrowserRouter>
    )

    expect(screen.getByText('Products')).toBeInTheDocument()
  })

  it('renders add product link', async () => {
    const { default: ProductsPage } = await import('./ProductsPage')
    render(
      <BrowserRouter>
        <ProductsPage />
      </BrowserRouter>
    )

    const addProductLink = screen.getByRole('link', { name: /add product/i })
    expect(addProductLink).toBeInTheDocument()
    expect(addProductLink).toHaveAttribute('href', '/products/new')
  })

  it('renders empty state when no products', async () => {
    const { default: ProductsPage } = await import('./ProductsPage')
    render(
      <BrowserRouter>
        <ProductsPage />
      </BrowserRouter>
    )

    expect(screen.getByText('No products yet')).toBeInTheDocument()
    expect(screen.getByText('Click "Add Product" to create your first product')).toBeInTheDocument()
  })
})
