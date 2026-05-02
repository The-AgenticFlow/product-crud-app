import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import StockIndicator from './StockIndicator'

describe('StockIndicator', () => {
  it('displays normal stock correctly', () => {
    render(<StockIndicator stock={50} />)

    expect(screen.getByText('In Stock: 50')).toBeInTheDocument()
    expect(screen.getByText('In Stock: 50').closest('div')).toHaveClass('bg-green-100', 'text-green-800')
  })

  it('displays low stock warning when stock is below threshold', () => {
    render(<StockIndicator stock={5} />)

    expect(screen.getByText('Low Stock: 5')).toBeInTheDocument()
    expect(screen.getByText('Low Stock: 5').closest('div')).toHaveClass('bg-yellow-100', 'text-yellow-800')
  })

  it('displays out of stock when stock is zero', () => {
    render(<StockIndicator stock={0} />)

    expect(screen.getByText('Out of Stock')).toBeInTheDocument()
    expect(screen.getByText('Out of Stock').closest('div')).toHaveClass('bg-red-100', 'text-red-800')
  })

  it('uses default low stock threshold of 10', () => {
    // Stock of 9 should be low
    render(<StockIndicator stock={9} />)
    expect(screen.getByText('Low Stock: 9')).toBeInTheDocument()

    // Stock of 10 should be normal
    render(<StockIndicator stock={10} />)
    expect(screen.getByText('In Stock: 10')).toBeInTheDocument()
  })

  it('allows custom low stock threshold', () => {
    // With threshold of 20, stock of 15 should be low
    render(<StockIndicator stock={15} lowStockThreshold={20} />)
    expect(screen.getByText('Low Stock: 15')).toBeInTheDocument()

    // With threshold of 20, stock of 25 should be normal
    render(<StockIndicator stock={25} lowStockThreshold={20} />)
    expect(screen.getByText('In Stock: 25')).toBeInTheDocument()
  })

  it('displays warning icon for low stock', () => {
    render(<StockIndicator stock={5} />)
    const container = screen.getByText('Low Stock: 5').closest('div')
    const icon = container?.querySelector('svg')
    expect(icon).toBeInTheDocument()
  })

  it('displays warning icon for out of stock', () => {
    render(<StockIndicator stock={0} />)
    const container = screen.getByText('Out of Stock').closest('div')
    const icon = container?.querySelector('svg')
    expect(icon).toBeInTheDocument()
  })

  it('displays package icon for normal stock', () => {
    render(<StockIndicator stock={50} />)
    const container = screen.getByText('In Stock: 50').closest('div')
    const icon = container?.querySelector('svg')
    expect(icon).toBeInTheDocument()
  })
})
