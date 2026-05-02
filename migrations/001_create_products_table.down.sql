-- Drop trigger and function
DROP TRIGGER IF EXISTS update_products_updated_at ON products;
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop index
DROP INDEX IF EXISTS idx_products_category;

-- Drop products table
DROP TABLE IF EXISTS products;
