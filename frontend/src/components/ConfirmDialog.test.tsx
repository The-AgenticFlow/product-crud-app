import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import ConfirmDialog from './ConfirmDialog'

describe('ConfirmDialog', () => {
  const defaultProps = {
    isOpen: true,
    title: 'Test Dialog',
    message: 'This is a test message',
    onConfirm: vi.fn(),
    onCancel: vi.fn(),
    onClose: vi.fn(),
  }

  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders with correct content', () => {
    render(<ConfirmDialog {...defaultProps} />)

    expect(screen.getByText('Test Dialog')).toBeInTheDocument()
    expect(screen.getByText('This is a test message')).toBeInTheDocument()
    expect(screen.getByRole('dialog')).toBeInTheDocument()
  })

  it('renders with default button text', () => {
    render(<ConfirmDialog {...defaultProps} />)

    expect(screen.getByRole('button', { name: 'Confirm' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Cancel' })).toBeInTheDocument()
  })

  it('renders with custom button text', () => {
    render(
      <ConfirmDialog
        {...defaultProps}
        confirmText="Delete"
        cancelText="Go Back"
      />
    )

    expect(screen.getByRole('button', { name: 'Delete' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Go Back' })).toBeInTheDocument()
  })

  it('has correct ARIA attributes', () => {
    render(<ConfirmDialog {...defaultProps} />)

    const dialog = screen.getByRole('dialog')
    expect(dialog).toHaveAttribute('aria-modal', 'true')
    expect(dialog).toHaveAttribute('aria-labelledby', 'dialog-title')
    expect(dialog).toHaveAttribute('aria-describedby', 'dialog-description')
  })

  it('calls onConfirm when confirm button is clicked', async () => {
    const onConfirm = vi.fn()
    render(<ConfirmDialog {...defaultProps} onConfirm={onConfirm} />)

    const confirmButton = screen.getByRole('button', { name: 'Confirm' })
    fireEvent.click(confirmButton)

    expect(onConfirm).toHaveBeenCalledTimes(1)
  })

  it('calls onCancel when cancel button is clicked', async () => {
    const onCancel = vi.fn()
    render(<ConfirmDialog {...defaultProps} onCancel={onCancel} />)

    const cancelButton = screen.getByRole('button', { name: 'Cancel' })
    fireEvent.click(cancelButton)

    expect(onCancel).toHaveBeenCalledTimes(1)
  })

  it('calls onCancel when close button is clicked', async () => {
    const onCancel = vi.fn()
    render(<ConfirmDialog {...defaultProps} onCancel={onCancel} />)

    const closeButton = screen.getByLabelText('Close dialog')
    fireEvent.click(closeButton)

    expect(onCancel).toHaveBeenCalledTimes(1)
  })

  it('calls onCancel when Escape key is pressed', async () => {
    const onCancel = vi.fn()
    render(<ConfirmDialog {...defaultProps} onCancel={onCancel} />)

    fireEvent.keyDown(document, { key: 'Escape' })

    expect(onCancel).toHaveBeenCalledTimes(1)
  })

  it('calls onConfirm when Enter key is pressed', async () => {
    const onConfirm = vi.fn()
    render(<ConfirmDialog {...defaultProps} onConfirm={onConfirm} />)

    fireEvent.keyDown(document, { key: 'Enter' })

    expect(onConfirm).toHaveBeenCalledTimes(1)
  })

  it('does not render when isOpen is false', () => {
    render(<ConfirmDialog {...defaultProps} isOpen={false} />)

    expect(screen.queryByRole('dialog')).not.toBeInTheDocument()
  })

  it('shows danger variant styling', () => {
    render(<ConfirmDialog {...defaultProps} variant="danger" />)

    const confirmButton = screen.getByRole('button', { name: 'Confirm' })
    expect(confirmButton).toHaveClass('bg-red-600')

    // Check for warning icon container
    const iconContainer = document.querySelector('.bg-red-100.rounded-full')
    expect(iconContainer).toBeInTheDocument()
  })

  it('shows default variant styling', () => {
    render(<ConfirmDialog {...defaultProps} variant="default" />)

    const confirmButton = screen.getByRole('button', { name: 'Confirm' })
    expect(confirmButton).toHaveClass('bg-blue-600')
  })

  it('closes when clicking outside the dialog', async () => {
    const onCancel = vi.fn()
    render(<ConfirmDialog {...defaultProps} onCancel={onCancel} />)

    // Click the overlay backdrop
    const overlay = document.querySelector('.fixed.inset-0.z-50')
    fireEvent.click(overlay!)

    expect(onCancel).toHaveBeenCalledTimes(1)
  })

  it('does not close when clicking inside the dialog', async () => {
    const onCancel = vi.fn()
    render(<ConfirmDialog {...defaultProps} onCancel={onCancel} />)

    const dialog = screen.getByRole('dialog')
    fireEvent.click(dialog)

    expect(onCancel).not.toHaveBeenCalled()
  })

  it('focuses the confirm button when opened', async () => {
    render(<ConfirmDialog {...defaultProps} />)

    const confirmButton = screen.getByRole('button', { name: 'Confirm' })

    // Focus should move to confirm button after dialog opens
    await waitFor(() => {
      expect(confirmButton).toHaveFocus()
    })
  })

  it('returns focus to trigger element on close', async () => {
    const triggerButton = document.createElement('button')
    triggerButton.textContent = 'Open Dialog'
    document.body.appendChild(triggerButton)
    triggerButton.focus()

    const { rerender } = render(
      <ConfirmDialog {...defaultProps} isOpen={true} />
    )

    // Close the dialog
    rerender(<ConfirmDialog {...defaultProps} isOpen={false} />)

    await waitFor(() => {
      expect(triggerButton).toHaveFocus()
    })

    document.body.removeChild(triggerButton)
  })
})
