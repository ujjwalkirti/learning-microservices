# Quick Start Guide

## Prerequisites

- Docker and Docker Compose
- OR: Node.js 16+, Rust 1.70+, Go 1.21+, Java 17+
- PostgreSQL and MongoDB (if not using Docker)

## Option 1: Run with Docker Compose (Recommended)

The easiest way to run all microservices together:

```bash
cd microservices/lms
docker-compose up
```

This will:
- Start PostgreSQL database (for Rust, Go, Java)
- Start MongoDB database (for Node.js)
- Build and start all 4 microservices
- Create necessary networks and volumes

Wait for all services to be healthy (usually 30-60 seconds).

### Verify Services

```bash
# Check Node.js service
curl http://localhost:3000/health

# Check Rust service
curl http://localhost:3001/health

# Check Go service
curl http://localhost:3002/health

# Check Java service
curl http://localhost:3003/health
```

All should respond with status message.

### Stop Services

```bash
docker-compose down
```

---

## Option 2: Run Services Individually

### Prerequisites Setup

```bash
# Start PostgreSQL
docker run --name lms-postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres

# Start MongoDB
docker run --name lms-mongo -p 27017:27017 -d mongo
```

### Node.js Service

```bash
cd nodejs
npm install
npm start
# Service runs on http://localhost:3000
```

### Rust Service

```bash
cd rust
cargo run
# Service runs on http://localhost:3001
```

### Go Service

```bash
cd go
go mod download
go run main.go
# Service runs on http://localhost:3002
```

### Java Service

```bash
cd java
mvn clean install
mvn spring-boot:run
# Service runs on http://localhost:3003
```

---

## Testing the API

### 1. Register a User

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@example.com",
    "password": "password123",
    "name": "John Student",
    "role": "student"
  }'
```

Expected response:
```json
{
  "message": "User registered successfully"
}
```

### 2. Login

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@example.com",
    "password": "password123"
  }'
```

Expected response:
```json
{
  "token": "jwt_token_here",
  "user": {
    "email": "student@example.com",
    "name": "John Student",
    "role": "student"
  }
}
```

Save the token for subsequent requests.

### 3. Create a Course

```bash
curl -X POST http://localhost:3000/api/cms/courses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer jwt_token_here" \
  -d '{
    "title": "Web Development Fundamentals",
    "description": "Learn HTML, CSS, and JavaScript",
    "instructor": "Jane Doe",
    "duration": 12
  }'
```

### 4. Get All Courses

```bash
curl -X GET http://localhost:3000/api/cms/courses \
  -H "Authorization: Bearer jwt_token_here"
```

### 5. Create a Syllabus

```bash
curl -X POST http://localhost:3000/api/syllabus/create \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer jwt_token_here" \
  -d '{
    "courseId": 1,
    "topics": ["HTML", "CSS", "JavaScript", "React"],
    "duration": 12,
    "objectives": [
      "Learn web development fundamentals",
      "Build responsive websites",
      "Master React framework"
    ]
  }'
```

### 6. Track User Activity

```bash
curl -X POST http://localhost:3000/api/analytics/track \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer jwt_token_here" \
  -d '{
    "userId": "student@example.com",
    "courseId": 1,
    "action": "video_watched",
    "timestamp": "2024-01-15T10:30:00Z"
  }'
```

### 7. Get User Progress

```bash
curl -X GET http://localhost:3000/api/analytics/progress/student%40example.com/1 \
  -H "Authorization: Bearer jwt_token_here"
```

### 8. Create Previous Year Questions

```bash
curl -X POST http://localhost:3000/api/pyq/create \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer jwt_token_here" \
  -d '{
    "courseId": 1,
    "year": 2023,
    "questions": [
      "What is HTML?",
      "Explain CSS flexbox",
      "What are JavaScript promises?"
    ],
    "duration": 120,
    "totalMarks": 100
  }'
```

---

## Testing with Same Request on Multiple Services

You can make the same request to different services to compare responses:

```bash
# Test on Node.js (3000)
curl http://localhost:3000/health

# Test on Rust (3001)
curl http://localhost:3001/health

# Test on Go (3002)
curl http://localhost:3002/health

# Test on Java (3003)
curl http://localhost:3003/health
```

All should respond with success messages.

---

## API Documentation

For complete API documentation, see [API_DOCUMENTATION.md](./API_DOCUMENTATION.md)

---

## Environment Variables

### Node.js
- `PORT=3000`
- `DATABASE_URL=mongodb://localhost:27017/lms`
- `JWT_SECRET=your_jwt_secret_key_here`
- `NODE_ENV=development`

### Rust
- `PORT=3001`
- `DATABASE_URL=postgres://user:password@localhost:5432/lms`
- `JWT_SECRET=your_jwt_secret_key_here`
- `RUST_LOG=info`

### Go
- `PORT=3002`
- `DATABASE_URL=postgres://user:password@localhost:5432/lms`
- `JWT_SECRET=your_jwt_secret_key_here`
- `GIN_MODE=debug`

### Java
- `SERVER_PORT=3003`
- `SPRING_DATASOURCE_URL=jdbc:postgresql://localhost:5432/lms`
- `SPRING_DATASOURCE_USERNAME=postgres`
- `SPRING_DATASOURCE_PASSWORD=password`
- `JWT_SECRET=your_jwt_secret_key_here`

---

## Troubleshooting

### Port Already in Use

If a port is already in use, either:
1. Kill the process using that port
2. Change the port in environment variables/config

### Database Connection Issues

Ensure PostgreSQL and MongoDB are running:

```bash
# Check PostgreSQL
docker ps | grep postgres

# Check MongoDB
docker ps | grep mongo
```

### Service Won't Start

Check service logs:

```bash
# Docker Compose
docker-compose logs service-name

# Individual services
# Check .env files for correct configuration
# Check database connectivity
```

### JWT Token Issues

- Make sure token is passed in `Authorization: Bearer <token>` header
- Check token hasn't expired
- Verify JWT_SECRET is the same on all services

---

## Production Deployment

For production:

1. **Use environment-specific configs** (dev, staging, prod)
2. **Implement proper database migrations**
3. **Add authentication middleware** to protected routes
4. **Use HTTPS** for all API calls
5. **Implement rate limiting** to prevent abuse
6. **Add logging and monitoring**
7. **Use reverse proxy** (nginx) for load balancing
8. **Containerize** and use Kubernetes for orchestration

---

## Next Steps

1. Read the [API_DOCUMENTATION.md](./API_DOCUMENTATION.md) for detailed endpoint documentation
2. Check individual service READMEs for language-specific information
3. Review the [ARCHITECTURE.md](./ARCHITECTURE.md) for system design details
4. Explore the source code in each language directory

---

## Support

For issues or questions:
1. Check the service logs
2. Review API documentation
3. Verify database connectivity
4. Check environment variables
5. Review error responses from API

---

## License

MIT
