import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import ProductImage from './ProductImage'

describe('ProductImage', () => {
  it('renders image when URL is provided', () => {
    render(<ProductImage imageUrl="https://example.com/product.jpg" name="Test Product" />)

    const img = screen.getByRole('img')
    expect(img).toBeInTheDocument()
    expect(img).toHaveAttribute('src', 'https://example.com/product.jpg')
    expect(img).toHaveAttribute('alt', 'Test Product')
  })

  it('renders fallback when no URL is provided', () => {
    render(<ProductImage name="Test Product" />)

    expect(screen.getByText('No image available')).toBeInTheDocument()
    expect(screen.queryByRole('img')).not.toBeInTheDocument()
  })

  it('renders fallback when URL is empty string', () => {
    render(<ProductImage imageUrl="" name="Test Product" />)

    expect(screen.getByText('No image available')).toBeInTheDocument()
    expect(screen.queryByRole('img')).not.toBeInTheDocument()
  })

  it('renders fallback when URL is undefined', () => {
    render(<ProductImage imageUrl={undefined} name="Test Product" />)

    expect(screen.getByText('No image available')).toBeInTheDocument()
    expect(screen.queryByRole('img')).not.toBeInTheDocument()
  })

  it('applies custom className', () => {
    const { container } = render(
      <ProductImage imageUrl="https://example.com/product.jpg" name="Test Product" className="custom-class" />
    )

    const wrapper = container.querySelector('.custom-class')
    expect(wrapper).toBeInTheDocument()
  })

  it('applies default minimum height to fallback', () => {
    const { container } = render(<ProductImage name="Test Product" />)

    const fallback = container.querySelector('[style*="min-height: 300px"]')
    expect(fallback).toBeInTheDocument()
  })

  it('applies default minimum height to image wrapper', () => {
    const { container } = render(
      <ProductImage imageUrl="https://example.com/product.jpg" name="Test Product" />
    )

    const wrapper = container.querySelector('.bg-gray-100')
    expect(wrapper).toBeInTheDocument()
  })

  it('renders image icon in fallback', () => {
    render(<ProductImage name="Test Product" />)

    const container = screen.getByText('No image available').parentElement
    const icon = container?.querySelector('svg')
    expect(icon).toBeInTheDocument()
  })
})
