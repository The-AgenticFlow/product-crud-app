import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import { MemoryRouter } from 'react-router-dom'
import App from './App'

describe('App', () => {
  it('renders without crashing', () => {
    expect(typeof App).toBe('function')
  })

  it('renders layout with navigation on root route', () => {
    render(
      <MemoryRouter initialEntries={['/']}>
        <App />
      </MemoryRouter>
    )

    expect(screen.getByText('Product CRUD App')).toBeInTheDocument()
    expect(screen.getByRole('link', { name: /home/i })).toBeInTheDocument()
    expect(screen.getByRole('link', { name: /products/i })).toBeInTheDocument()
  })

  it('renders HomePage content on root route', () => {
    render(
      <MemoryRouter initialEntries={['/']}>
        <App />
      </MemoryRouter>
    )

    expect(screen.getByText('Welcome to Product CRUD App')).toBeInTheDocument()
    expect(screen.getByText('View Products')).toBeInTheDocument()
    expect(screen.getByText('Add Products')).toBeInTheDocument()
    expect(screen.getByText('Manage Inventory')).toBeInTheDocument()
  })

  it('renders ProductsPage content on /products route', () => {
    render(
      <MemoryRouter initialEntries={['/products']}>
        <App />
      </MemoryRouter>
    )

    expect(screen.getByRole('heading', { name: /products/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /add product/i })).toBeInTheDocument()
    expect(screen.getByText('No products yet')).toBeInTheDocument()
  })

  it('layout is present on products route', () => {
    render(
      <MemoryRouter initialEntries={['/products']}>
        <App />
      </MemoryRouter>
    )

    expect(screen.getByText('Product CRUD App')).toBeInTheDocument()
    expect(screen.getByRole('link', { name: /home/i })).toBeInTheDocument()
    expect(screen.getByRole('link', { name: /products/i })).toBeInTheDocument()
  })
})
