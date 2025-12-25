# LMS Microservice - Go

A lightweight and fast Learning Management System microservice built with Gin.

## Features

- ✅ Authentication (Register, Login, Logout)
- ✅ Course Management (CMS)
- ✅ Syllabus Management
- ✅ User Analytics & Progress Tracking
- ✅ Previous Year Questions (PYQs)
- ✅ Fast HTTP server with Gin
- ✅ Concurrent request handling

## Prerequisites

- Go 1.21+
- PostgreSQL

## Installation

```bash
go mod download
```

## Configuration

Create a `.env` file in the root directory:

```env
PORT=3002
DATABASE_URL=postgres://user:password@localhost:5432/lms
JWT_SECRET=your_jwt_secret_key_here
GIN_MODE=debug
```

## Running the Service

### Development Mode
```bash
go run main.go
```

### Build Binary
```bash
go build -o lms-service
./lms-service
```

The service will start on `http://localhost:3002`

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
GET /api/syllabus/course/{courseId}
PUT /api/syllabus/{id}
```

### Analytics
```bash
POST /api/analytics/track
GET /api/analytics/progress/{userId}/{courseId}
GET /api/analytics/course/{courseId}
```

### PYQs
```bash
POST /api/pyq/create
GET /api/pyq/course/{courseId}
GET /api/pyq/course/{courseId}/year/{year}
```

## Example Requests

### Register User
```bash
curl -X POST http://localhost:3002/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123",
    "name": "John Doe",
    "role": "student"
  }'
```

### Get Health Status
```bash
curl http://localhost:3002/health
```

## Project Structure

```
.
├── main.go              # Application entry point
├── go.mod               # Module definition
├── .env                 # Environment variables
└── README.md            # This file
```

## Testing

```bash
go test ./...
```

## Deployment

### Docker

```bash
# Build image
docker build -t lms-go .

# Run container
docker run -p 3002:3002 \
  -e PORT=3002 \
  -e JWT_SECRET=your_secret \
  lms-go
```

### Environment Variables

- `PORT`: Service port (default: 3002)
- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: Secret key for JWT signing
- `GIN_MODE`: Gin mode (debug, release)

## Performance

Go provides:
- Fast compilation and execution
- Lightweight goroutines for concurrency
- Built-in HTTP server capabilities
- Static binary compilation

## Dependencies

- **gin-gonic/gin**: Web framework
- **golang-jwt/jwt**: JWT authentication
- **gorm**: ORM for database operations
- **crypto**: Encryption and hashing

## Advantages

- Single binary deployment
- Built-in concurrency with goroutines
- Fast startup time
- Memory efficient
- Great for microservices

## License

MIT
