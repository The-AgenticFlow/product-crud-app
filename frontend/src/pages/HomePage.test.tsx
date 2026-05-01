import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'

describe('HomePage', () => {
  it('renders welcome heading', async () => {
    const { default: HomePage } = await import('./HomePage')
    render(<HomePage />)

    expect(screen.getByText('Welcome to Product CRUD App')).toBeInTheDocument()
  })

  it('renders app description', async () => {
    const { default: HomePage } = await import('./HomePage')
    render(<HomePage />)

    expect(screen.getByText('A simple application to manage your products')).toBeInTheDocument()
  })

  it('renders feature cards', async () => {
    const { default: HomePage } = await import('./HomePage')
    render(<HomePage />)

    expect(screen.getByText('View Products')).toBeInTheDocument()
    expect(screen.getByText('Add Products')).toBeInTheDocument()
    expect(screen.getByText('Manage Inventory')).toBeInTheDocument()
  })

  it('renders feature descriptions', async () => {
    const { default: HomePage } = await import('./HomePage')
    render(<HomePage />)

    expect(screen.getByText('Browse and search through your product catalog')).toBeInTheDocument()
    expect(screen.getByText('Create new products with detailed information')).toBeInTheDocument()
    expect(screen.getByText('Update stock levels and product details')).toBeInTheDocument()
  })
})
