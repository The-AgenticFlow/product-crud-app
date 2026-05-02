import { z } from 'zod'

export const productSchema = z.object({
  name: z.string()
    .min(1, 'Name is required')
    .max(255, 'Name must be 255 characters or less'),
  description: z.string()
    .max(1000, 'Description must be 1000 characters or less')
    .optional()
    .or(z.literal('')),
  price: z.number()
    .min(0, 'Price must be 0 or greater'),
  stock: z.number()
    .int('Stock must be an integer')
    .min(0, 'Stock must be 0 or greater'),
  category: z.string()
    .optional()
    .or(z.literal('')),
  imageUrl: z.string()
    .url('Must be a valid URL')
    .optional()
    .or(z.literal(''))
})

export type ProductFormData = z.infer<typeof productSchema>
