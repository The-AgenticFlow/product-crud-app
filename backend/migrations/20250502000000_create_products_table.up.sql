-- Enable UUID extension for gen_random_uuid()
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create products table
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL CHECK (price >= 0),
    stock INTEGER NOT NULL CHECK (stock >= 0) DEFAULT 0,
    category VARCHAR(100),
    sku VARCHAR(100) UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create index on category for faster filtering
CREATE INDEX idx_products_category ON products(category);

-- Create index on is_active for faster filtering
CREATE INDEX idx_products_is_active ON products(is_active);

-- Create index on sku for faster lookups
CREATE INDEX idx_products_sku ON products(sku);
