# Product CRUD Application

A full-stack CRUD application for managing products, built with **Rust** (backend) and **React** (frontend).

## Tech Stack

### Backend
- **Rust** with Actix-web or Axum
- **SQLx** or **Diesel** for database operations
- **PostgreSQL** as the primary database
- **Serde** for JSON serialization
- **Tracing** for logging

### Frontend
- **React** with TypeScript
- **Vite** as the build tool
- **React Query** for data fetching
- **React Router** for navigation
- **TailwindCSS** for styling
- **Zod** for form validation

## Features

- Create, Read, Update, and Delete products
- Product search and filtering
- Form validation
- Responsive UI
- RESTful API
- Error handling

## Project Structure

```
product-crud-app/
├── backend/          # Rust API server
│   ├── src/
│   ├── Cargo.toml
│   └── migrations/
├── frontend/         # React application
│   ├── src/
│   ├── package.json
│   └── vite.config.ts
├── docker-compose.yml
└── README.md
```

## Getting Started

### Prerequisites

- Rust (latest stable)
- Node.js 18+
- PostgreSQL 15+
- Docker & Docker Compose (optional)

### Backend Setup

```bash
cd backend
cargo run
```

### Frontend Setup

```bash
cd frontend
npm install
npm run dev
```

### Docker Setup

```bash
docker-compose up -d
```

## API Endpoints

| Method | Endpoint          | Description         |
|--------|-------------------|---------------------|
| GET    | /api/products     | List all products   |
| GET    | /api/products/:id | Get a single product|
| POST   | /api/products     | Create a product    |
| PUT    | /api/products/:id | Update a product    |
| DELETE | /api/products/:id | Delete a product    |

## Product Schema

```json
{
  "id": "uuid",
  "name": "string",
  "description": "string",
  "price": "number",
  "stock": "integer",
  "category": "string",
  "created_at": "timestamp",
  "updated_at": "timestamp"
}
```

## Development Workflow

1. Pick an issue from the GitHub Issues tab
2. Create a branch: `git checkout -b feature/issue-number-description`
3. Implement the feature
4. Write tests
5. Submit a pull request

## License

MIT
