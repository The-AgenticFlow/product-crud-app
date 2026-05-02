import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, act, waitFor } from '@testing-library/react'
import { useConfirmDialog } from './useConfirmDialog'

describe('useConfirmDialog', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('returns a confirm function and dialogProps', () => {
    const { result } = renderHook(() => useConfirmDialog())

    expect(result.current.confirm).toBeInstanceOf(Function)
    expect(result.current.dialogProps).toBeDefined()
    expect(result.current.dialogProps.isOpen).toBe(false)
  })

  it('opens dialog when confirm is called', async () => {
    const { result } = renderHook(() => useConfirmDialog())

    expect(result.current.dialogProps.isOpen).toBe(false)

    act(() => {
      result.current.confirm({
        title: 'Test Title',
        message: 'Test Message',
      })
    })

    expect(result.current.dialogProps.isOpen).toBe(true)
    expect(result.current.dialogProps.title).toBe('Test Title')
    expect(result.current.dialogProps.message).toBe('Test Message')
  })

  it('uses default values for optional props', async () => {
    const { result } = renderHook(() => useConfirmDialog())

    act(() => {
      result.current.confirm({
        title: 'Test',
        message: 'Test',
      })
    })

    expect(result.current.dialogProps.confirmText).toBeUndefined()
    expect(result.current.dialogProps.cancelText).toBeUndefined()
    expect(result.current.dialogProps.variant).toBeUndefined()
  })

  it('accepts custom configuration', async () => {
    const { result } = renderHook(() => useConfirmDialog())

    act(() => {
      result.current.confirm({
        title: 'Delete Item',
        message: 'Are you sure?',
        confirmText: 'Delete',
        cancelText: 'Go Back',
        variant: 'danger',
      })
    })

    expect(result.current.dialogProps.title).toBe('Delete Item')
    expect(result.current.dialogProps.message).toBe('Are you sure?')
    expect(result.current.dialogProps.confirmText).toBe('Delete')
    expect(result.current.dialogProps.cancelText).toBe('Go Back')
    expect(result.current.dialogProps.variant).toBe('danger')
  })

  it('resolves with true when confirmed', async () => {
    const { result } = renderHook(() => useConfirmDialog())

    let confirmPromise: Promise<boolean> | undefined

    act(() => {
      confirmPromise = result.current.confirm({
        title: 'Test',
        message: 'Test',
      })
    })

    act(() => {
      result.current.dialogProps.onConfirm()
    })

    await waitFor(() => {
      expect(confirmPromise).resolves.toBe(true)
    })
    expect(result.current.dialogProps.isOpen).toBe(false)
  })

  it('resolves with false when canceled', async () => {
    const { result } = renderHook(() => useConfirmDialog())

    let confirmPromise: Promise<boolean> | undefined

    act(() => {
      confirmPromise = result.current.confirm({
        title: 'Test',
        message: 'Test',
      })
    })

    act(() => {
      result.current.dialogProps.onCancel()
    })

    await waitFor(() => {
      expect(confirmPromise).resolves.toBe(false)
    })
    expect(result.current.dialogProps.isOpen).toBe(false)
  })

  it('resolves with false when closed', async () => {
    const { result } = renderHook(() => useConfirmDialog())

    let confirmPromise: Promise<boolean> | undefined

    act(() => {
      confirmPromise = result.current.confirm({
        title: 'Test',
        message: 'Test',
      })
    })

    act(() => {
      result.current.dialogProps.onClose()
    })

    await waitFor(() => {
      expect(confirmPromise).resolves.toBe(false)
    })
    expect(result.current.dialogProps.isOpen).toBe(false)
  })

  it('can be reused multiple times', async () => {
    const { result } = renderHook(() => useConfirmDialog())

    // First confirmation
    let confirmPromise1: Promise<boolean> | undefined
    act(() => {
      confirmPromise1 = result.current.confirm({
        title: 'First',
        message: 'First Message',
      })
    })

    act(() => {
      result.current.dialogProps.onConfirm()
    })

    await waitFor(() => {
      expect(confirmPromise1).resolves.toBe(true)
    })
    expect(result.current.dialogProps.isOpen).toBe(false)

    // Second confirmation
    let confirmPromise2: Promise<boolean> | undefined
    act(() => {
      confirmPromise2 = result.current.confirm({
        title: 'Second',
        message: 'Second Message',
      })
    })

    expect(result.current.dialogProps.title).toBe('Second')
    expect(result.current.dialogProps.isOpen).toBe(true)

    act(() => {
      result.current.dialogProps.onCancel()
    })

    await waitFor(() => {
      expect(confirmPromise2).resolves.toBe(false)
    })
    expect(result.current.dialogProps.isOpen).toBe(false)
  })
})
