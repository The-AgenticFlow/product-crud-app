import { useEffect, useRef, useCallback } from 'react'
import { createPortal } from 'react-dom'
import { AlertTriangle, X } from 'lucide-react'

export interface ConfirmDialogProps {
  isOpen: boolean
  title: string
  message: string
  confirmText?: string
  cancelText?: string
  variant?: 'default' | 'danger'
  onConfirm: () => void
  onCancel: () => void
  onClose: () => void
}

export default function ConfirmDialog({
  isOpen,
  title,
  message,
  confirmText = 'Confirm',
  cancelText = 'Cancel',
  variant = 'default',
  onConfirm,
  onCancel,
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  onClose: _onClose, // Used by hook but not needed in component
}: ConfirmDialogProps) {
  const overlayRef = useRef<HTMLDivElement>(null)
  const dialogRef = useRef<HTMLDivElement>(null)
  const previousActiveElement = useRef<HTMLElement | null>(null)
  const confirmButtonRef = useRef<HTMLButtonElement>(null)

  // Handle escape key
  useEffect(() => {
    if (!isOpen) return

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        e.preventDefault()
        onCancel()
      } else if (e.key === 'Enter') {
        e.preventDefault()
        onConfirm()
      }
    }

    document.addEventListener('keydown', handleKeyDown)
    return () => document.removeEventListener('keydown', handleKeyDown)
  }, [isOpen, onConfirm, onCancel])

  // Focus trap and focus management
  useEffect(() => {
    if (isOpen) {
      // Store the previously focused element
      previousActiveElement.current = document.activeElement as HTMLElement

      // Focus the confirm button when dialog opens
      const timer = setTimeout(() => {
        confirmButtonRef.current?.focus()
      }, 100)

      // Prevent body scroll
      document.body.style.overflow = 'hidden'

      return () => {
        clearTimeout(timer)
        document.body.style.overflow = ''

        // Return focus to the trigger element
        previousActiveElement.current?.focus()
      }
    }
  }, [isOpen])

  // Handle click outside
  const handleOverlayClick = useCallback(
    (e: React.MouseEvent) => {
      if (e.target === overlayRef.current) {
        onCancel()
      }
    },
    [onCancel]
  )

  // Handle tab trap
  const handleTabKey = useCallback((e: React.KeyboardEvent) => {
    if (e.key !== 'Tab' || !dialogRef.current) return

    const focusableElements = dialogRef.current.querySelectorAll<HTMLElement>(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    )

    const firstElement = focusableElements[0]
    const lastElement = focusableElements[focusableElements.length - 1]

    if (e.shiftKey) {
      if (document.activeElement === firstElement) {
        e.preventDefault()
        lastElement.focus()
      }
    } else {
      if (document.activeElement === lastElement) {
        e.preventDefault()
        firstElement.focus()
      }
    }
  }, [])

  if (!isOpen) return null

  const dialogContent = (
    <div
      ref={overlayRef}
      onClick={handleOverlayClick}
      className="fixed inset-0 z-50 overflow-y-auto"
      aria-labelledby="dialog-title"
      aria-describedby="dialog-description"
    >
      {/* Overlay backdrop */}
      <div
        className="fixed inset-0 bg-black bg-opacity-50 transition-opacity animate-fade-in"
        aria-hidden="true"
      />

      {/* Dialog container */}
      <div className="flex min-h-full items-center justify-center p-4">
        <div
          ref={dialogRef}
          role="dialog"
          aria-modal="true"
          aria-labelledby="dialog-title"
          aria-describedby="dialog-description"
          onKeyDown={handleTabKey}
          className="relative bg-white rounded-lg shadow-xl max-w-md w-full transform transition-all animate-fade-in-scale"
        >
          {/* Close button */}
          <button
            onClick={onCancel}
            className="absolute top-4 right-4 text-gray-400 hover:text-gray-600 transition-colors"
            aria-label="Close dialog"
          >
            <X className="h-5 w-5" />
          </button>

          {/* Dialog content */}
          <div className="p-6">
            {/* Icon for danger variant */}
            {variant === 'danger' && (
              <div className="flex items-center justify-center w-12 h-12 mx-auto mb-4 bg-red-100 rounded-full">
                <AlertTriangle className="h-6 w-6 text-red-600" />
              </div>
            )}

            {/* Title */}
            <h2
              id="dialog-title"
              className="text-xl font-semibold text-gray-900 text-center mb-2"
            >
              {title}
            </h2>

            {/* Message */}
            <p
              id="dialog-description"
              className="text-gray-600 text-center mb-6"
            >
              {message}
            </p>

            {/* Actions */}
            <div className="flex gap-3 justify-center">
              <button
                onClick={onCancel}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2"
              >
                {cancelText}
              </button>
              <button
                ref={confirmButtonRef}
                onClick={onConfirm}
                className={`px-4 py-2 text-sm font-medium text-white rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 ${
                  variant === 'danger'
                    ? 'bg-red-600 hover:bg-red-700 focus:ring-red-500'
                    : 'bg-blue-600 hover:bg-blue-700 focus:ring-blue-500'
                }`}
              >
                {confirmText}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )

  // Render using portal
  return createPortal(dialogContent, document.body)
}
