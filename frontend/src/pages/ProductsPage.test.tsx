import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
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

  it('renders delete button for each product', async () => {
    const mockProducts = [
      {
        id: '1',
        name: 'Test Product',
        description: 'A test product',
        price: 29.99,
        stock: 10,
        category: 'Electronics'
      }
    ]

    const { default: ProductsPage } = await import('./ProductsPage')
    render(
      <BrowserRouter>
        <ProductsPage initialProducts={mockProducts} />
      </BrowserRouter>
    )

    const deleteButton = screen.getByRole('button', { name: /delete test product/i })
    expect(deleteButton).toBeInTheDocument()
  })

  it('shows confirmation dialog when delete button is clicked', async () => {
    const mockProducts = [
      {
        id: '1',
        name: 'Test Product',
        description: 'A test product',
        price: 29.99,
        stock: 10,
        category: 'Electronics'
      }
    ]

    const { default: ProductsPage } = await import('./ProductsPage')
    render(
      <BrowserRouter>
        <ProductsPage initialProducts={mockProducts} />
      </BrowserRouter>
    )

    const deleteButton = screen.getByRole('button', { name: /delete test product/i })
    fireEvent.click(deleteButton)

    await waitFor(() => {
      expect(screen.getByText('Delete Product')).toBeInTheDocument()
      expect(screen.getByText(/are you sure you want to delete "test product"/i)).toBeInTheDocument()
    })
  })

  it('triggers delete action when confirmed', async () => {
    const mockProducts = [
      {
        id: '1',
        name: 'Test Product',
        description: 'A test product',
        price: 29.99,
        stock: 10,
        category: 'Electronics'
      }
    ]

    const consoleSpy = vi.spyOn(console, 'log')

    const { default: ProductsPage } = await import('./ProductsPage')
    render(
      <BrowserRouter>
        <ProductsPage initialProducts={mockProducts} />
      </BrowserRouter>
    )

    const deleteButton = screen.getByRole('button', { name: /delete test product/i })
    fireEvent.click(deleteButton)

    await waitFor(() => {
      expect(screen.getByText('Delete Product')).toBeInTheDocument()
    })

    // Use exact match to target the confirm button in the dialog
    const confirmButtons = screen.getAllByRole('button', { name: /delete/i })
    const dialogConfirmButton = confirmButtons.find(btn => btn.textContent === 'Delete')
    fireEvent.click(dialogConfirmButton!)

    await waitFor(() => {
      expect(consoleSpy).toHaveBeenCalledWith('Deleting product:', '1')
    })
  })

  it('closes dialog without deletion when canceled', async () => {
    const mockProducts = [
      {
        id: '1',
        name: 'Test Product',
        description: 'A test product',
        price: 29.99,
        stock: 10,
        category: 'Electronics'
      }
    ]

    const consoleSpy = vi.spyOn(console, 'log')

    const { default: ProductsPage } = await import('./ProductsPage')
    render(
      <BrowserRouter>
        <ProductsPage initialProducts={mockProducts} />
      </BrowserRouter>
    )

    const deleteButton = screen.getByRole('button', { name: /delete test product/i })
    fireEvent.click(deleteButton)

    await waitFor(() => {
      expect(screen.getByText('Delete Product')).toBeInTheDocument()
    })

    const cancelButton = screen.getByRole('button', { name: /cancel/i })
    fireEvent.click(cancelButton)

    await waitFor(() => {
      expect(screen.queryByText('Delete Product')).not.toBeInTheDocument()
    })

    expect(consoleSpy).not.toHaveBeenCalled()
  })
})
