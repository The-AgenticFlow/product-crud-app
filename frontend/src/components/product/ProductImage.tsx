import { Image as ImageIcon } from 'lucide-react'

export interface ProductImageProps {
  imageUrl?: string
  name: string
  className?: string
}

export default function ProductImage({
  imageUrl,
  name,
  className = '',
}: ProductImageProps) {
  if (!imageUrl) {
    return (
      <div
        className={`flex items-center justify-center bg-gray-100 rounded-lg ${className}`}
        style={{ minHeight: '300px' }}
      >
        <div className="text-center text-gray-400">
          <ImageIcon className="h-16 w-16 mx-auto mb-2" />
          <p className="text-sm">No image available</p>
        </div>
      </div>
    )
  }

  return (
    <div className={`bg-gray-100 rounded-lg overflow-hidden ${className}`}>
      <img
        src={imageUrl}
        alt={name}
        className="w-full h-auto object-cover"
        style={{ minHeight: '300px', maxHeight: '500px' }}
        onError={(e) => {
          // Fallback to placeholder on error
          e.currentTarget.style.display = 'none'
          const parent = e.currentTarget.parentElement
          if (parent) {
            parent.innerHTML = `
              <div class="flex items-center justify-center bg-gray-100 rounded-lg" style="min-height: 300px;">
                <div class="text-center text-gray-400">
                  <svg class="h-16 w-16 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                  </svg>
                  <p class="text-sm">Failed to load image</p>
                </div>
              </div>
            `
          }
        }}
      />
    </div>
  )
}
