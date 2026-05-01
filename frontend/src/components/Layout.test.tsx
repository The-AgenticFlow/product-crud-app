import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import { MemoryRouter } from 'react-router-dom'
import Layout from './Layout'

describe('Layout', () => {
  it('renders header with app title', () => {
    render(
      <MemoryRouter>
        <Layout />
      </MemoryRouter>
    )

    expect(screen.getByText('Product CRUD App')).toBeInTheDocument()
  })

  it('renders navigation links', () => {
    render(
      <MemoryRouter>
        <Layout />
      </MemoryRouter>
    )

    expect(screen.getByRole('link', { name: /home/i })).toBeInTheDocument()
    expect(screen.getByRole('link', { name: /products/i })).toBeInTheDocument()
  })

  it('Home link points to root route', () => {
    render(
      <MemoryRouter>
        <Layout />
      </MemoryRouter>
    )

    const homeLink = screen.getByRole('link', { name: /home/i })
    expect(homeLink).toHaveAttribute('href', '/')
  })

  it('Products link points to products route', () => {
    render(
      <MemoryRouter>
        <Layout />
      </MemoryRouter>
    )

    const productsLink = screen.getByRole('link', { name: /products/i })
    expect(productsLink).toHaveAttribute('href', '/products')
  })
})
