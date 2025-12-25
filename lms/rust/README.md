# LMS Microservice - Rust

A high-performance Learning Management System microservice built with Actix-web.

## Features

- ✅ Authentication (Register, Login, Logout)
- ✅ Course Management (CMS)
- ✅ Syllabus Management
- ✅ User Analytics & Progress Tracking
- ✅ Previous Year Questions (PYQs)
- ✅ High performance with Actix-web
- ✅ Async/await support with Tokio

## Prerequisites

- Rust 1.70+
- Cargo
- PostgreSQL

## Installation

```bash
cargo build
```

## Configuration

Create a `.env` file in the root directory:

```env
DATABASE_URL=postgres://user:password@localhost:5432/lms
JWT_SECRET=your_jwt_secret_key_here
RUST_LOG=info
PORT=3001
```

## Running the Service

### Development Mode
```bash
cargo run
```

### Release Mode
```bash
cargo run --release
```

The service will start on `http://0.0.0.0:3001`

## API Endpoints

### Health Check
```bash
GET /health
```

### Authentication
```bash
POST /api/auth/register
POST /api/auth/login
POST /api/auth/logout
```

### Course Management
```bash
POST /api/cms/courses
GET /api/cms/courses
GET /api/cms/courses/{id}
PUT /api/cms/courses/{id}
DELETE /api/cms/courses/{id}
```

### Syllabus
```bash
POST /api/syllabus/create
GET /api/syllabus/course/{course_id}
PUT /api/syllabus/{id}
```

### Analytics
```bash
POST /api/analytics/track
GET /api/analytics/progress/{user_id}/{course_id}
GET /api/analytics/course/{course_id}
```

### PYQs
```bash
POST /api/pyq/create
GET /api/pyq/course/{course_id}
GET /api/pyq/course/{course_id}/year/{year}
```

## Project Structure

```
src/
├── main.rs                    # Application entry point
├── handlers/
│   ├── mod.rs               # Module definitions
│   ├── auth.rs              # Authentication handlers
│   ├── cms.rs               # Course management handlers
│   ├── syllabus.rs          # Syllabus handlers
│   ├── analytics.rs         # Analytics handlers
│   └── pyq.rs               # PYQ handlers
└── models/                  # Data models (optional)
```

## Testing

```bash
cargo test
```

## Deployment

### Docker

```bash
# Build image
docker build -t lms-rust .

# Run container
docker run -p 3001:3001 \
  -e DATABASE_URL=postgres://localhost/lms \
  -e JWT_SECRET=your_secret \
  lms-rust
```

### Environment Variables

- `PORT`: Service port (default: 3001)
- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: Secret key for JWT signing
- `RUST_LOG`: Log level (trace, debug, info, warn, error)

## Performance

Rust/Actix-web provides:
- Zero-cost abstractions
- Memory safety without garbage collection
- High concurrency with async/await
- Fast startup times

## Dependencies

- **actix-web**: Web framework
- **tokio**: Async runtime
- **serde**: Serialization/deserialization
- **jsonwebtoken**: JWT authentication
- **bcrypt**: Password hashing
- **dotenv**: Environment variables

## Build & Deploy Notes

- Rust creates optimized binaries
- Single executable deployment
- No runtime dependencies (except PostgreSQL)
- Excellent for containerization

## License

MIT
