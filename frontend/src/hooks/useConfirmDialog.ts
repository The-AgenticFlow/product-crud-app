import { useState, useCallback, useRef } from 'react'

export interface ConfirmDialogConfig {
  title: string
  message: string
  confirmText?: string
  cancelText?: string
  variant?: 'default' | 'danger'
}

interface ConfirmDialogState extends ConfirmDialogConfig {
  isOpen: boolean
}

export function useConfirmDialog() {
  const [dialogState, setDialogState] = useState<ConfirmDialogState>({
    isOpen: false,
    title: '',
    message: '',
  })

  const resolveRef = useRef<((value: boolean) => void) | null>(null)

  const confirm = useCallback(
    async (config: ConfirmDialogConfig): Promise<boolean> => {
      return new Promise((resolve) => {
        resolveRef.current = resolve
        setDialogState({
          ...config,
          isOpen: true,
        })
      })
    },
    []
  )

  const handleConfirm = useCallback(() => {
    resolveRef.current?.(true)
    setDialogState((prev) => ({ ...prev, isOpen: false }))
  }, [])

  const handleCancel = useCallback(() => {
    resolveRef.current?.(false)
    setDialogState((prev) => ({ ...prev, isOpen: false }))
  }, [])

  return {
    confirm,
    dialogProps: {
      isOpen: dialogState.isOpen,
      title: dialogState.title,
      message: dialogState.message,
      confirmText: dialogState.confirmText,
      cancelText: dialogState.cancelText,
      variant: dialogState.variant,
      onConfirm: handleConfirm,
      onCancel: handleCancel,
      onClose: handleCancel,
    },
  }
}
