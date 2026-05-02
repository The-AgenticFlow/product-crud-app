import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import ProductInfo from './ProductInfo'
import { Product } from '../../types'

describe('ProductInfo', () => {
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

  it('displays product name', () => {
    render(<ProductInfo product={mockProduct} />)

    expect(screen.getByText('Test Product')).toBeInTheDocument()
  })

  it('displays product description', () => {
    render(<ProductInfo product={mockProduct} />)

    expect(screen.getByText('A test product description')).toBeInTheDocument()
  })

  it('displays price formatted as currency', () => {
    render(<ProductInfo product={mockProduct} />)

    expect(screen.getByText('$99.99')).toBeInTheDocument()
  })

  it('formats price with correct decimal places', () => {
    const productWithDecimalPrice: Product = { ...mockProduct, price: 123.456 }
    render(<ProductInfo product={productWithDecimalPrice} />)

    // Should round to 2 decimal places
    expect(screen.getByText('$123.46')).toBeInTheDocument()
  })

  it('displays category as badge', () => {
    render(<ProductInfo product={mockProduct} />)

    expect(screen.getByText('Electronics')).toBeInTheDocument()
    const badge = screen.getByText('Electronics').closest('div')
    expect(badge).toHaveClass('bg-blue-100', 'text-blue-800')
  })

  it('does not display category badge when category is undefined', () => {
    const productWithoutCategory: Product = { ...mockProduct, category: undefined }
    render(<ProductInfo product={productWithoutCategory} />)

    expect(screen.queryByText('Electronics')).not.toBeInTheDocument()
  })

  it('displays stock indicator', () => {
    render(<ProductInfo product={mockProduct} />)

    expect(screen.getByText('In Stock: 10')).toBeInTheDocument()
  })

  it('displays low stock warning when stock is low', () => {
    const productWithLowStock: Product = { ...mockProduct, stock: 5 }
    render(<ProductInfo product={productWithLowStock} />)

    expect(screen.getByText('Low Stock: 5')).toBeInTheDocument()
  })

  it('displays out of stock when stock is zero', () => {
    const productOutOfStock: Product = { ...mockProduct, stock: 0 }
    render(<ProductInfo product={productOutOfStock} />)

    expect(screen.getByText('Out of Stock')).toBeInTheDocument()
  })

  it('displays created timestamp', () => {
    render(<ProductInfo product={mockProduct} />)

    expect(screen.getByText(/Created:/)).toBeInTheDocument()
  })

  it('displays updated timestamp', () => {
    render(<ProductInfo product={mockProduct} />)

    expect(screen.getByText(/Updated:/)).toBeInTheDocument()
  })

  it('displays image by default', () => {
    render(<ProductInfo product={mockProduct} />)

    const img = screen.getByRole('img')
    expect(img).toBeInTheDocument()
    expect(img).toHaveAttribute('src', 'https://example.com/image.jpg')
  })

  it('hides image when showImage is false', () => {
    render(<ProductInfo product={mockProduct} showImage={false} />)

    expect(screen.queryByRole('img')).not.toBeInTheDocument()
  })

  it('displays fallback when no image URL', () => {
    const productWithoutImage: Product = { ...mockProduct, imageUrl: undefined }
    render(<ProductInfo product={productWithoutImage} />)

    expect(screen.getByText('No image available')).toBeInTheDocument()
  })

  it('formats timestamps in human-readable format', () => {
    render(<ProductInfo product={mockProduct} />)

    // Check that the dates are formatted (actual format depends on locale)
    const createdText = screen.getByText(/Created:/).textContent
    const updatedText = screen.getByText(/Updated:/).textContent

    expect(createdText).toContain('January')
    expect(updatedText).toContain('January')
  })

  it('applies custom image className', () => {
    const { container } = render(
      <ProductInfo product={mockProduct} imageClassName="custom-image-class" />
    )

    const customClass = container.querySelector('.custom-image-class')
    expect(customClass).toBeInTheDocument()
  })
})
