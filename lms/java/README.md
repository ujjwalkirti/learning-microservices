# LMS Microservice - Java

A robust Learning Management System microservice built with Spring Boot.

## Features

- ✅ Authentication (Register, Login, Logout) with JWT
- ✅ Course Management (CMS)
- ✅ Syllabus Management
- ✅ User Analytics & Progress Tracking
- ✅ Previous Year Questions (PYQs)
- ✅ Spring Security
- ✅ JPA/Hibernate ORM
- ✅ Comprehensive error handling

## Prerequisites

- Java 17+
- Maven 3.8+
- PostgreSQL

## Installation

```bash
mvn clean install
```

## Configuration

The application uses `application.properties` for configuration:

```properties
spring.application.name=lms-microservice-java
spring.profiles.active=dev

# Server
server.port=3003

# Database
spring.datasource.url=jdbc:postgresql://localhost:5432/lms
spring.datasource.username=postgres
spring.datasource.password=password

# JWT
jwt.secret=your_jwt_secret_key_here
jwt.expiration=604800000
```

## Running the Service

### Using Maven
```bash
mvn spring-boot:run
```

### Using JAR
```bash
mvn clean package
java -jar target/lms-microservice-java-1.0.0.jar
```

The service will start on `http://localhost:3003`

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
curl -X POST http://localhost:3003/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123",
    "name": "John Doe",
    "role": "student"
  }'
```

### Create Course
```bash
curl -X POST http://localhost:3003/api/cms/courses \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Web Development",
    "description": "Learn web development",
    "instructor": "Jane Doe",
    "duration": 12
  }'
```

### Track Activity
```bash
curl -X POST http://localhost:3003/api/analytics/track \
  -H "Content-Type: application/json" \
  -d '{
    "userId": "user123",
    "courseId": 1,
    "action": "video_watched",
    "timestamp": "2024-01-15T10:30:00Z"
  }'
```

## Project Structure

```
src/main/
├── java/com/lms/
│   ├── LmsApplication.java          # Main application class
│   └── controller/
│       ├── HealthController.java     # Health endpoint
│       ├── AuthController.java       # Authentication endpoints
│       ├── CmsController.java        # Course management endpoints
│       ├── SyllabusController.java   # Syllabus endpoints
│       ├── AnalyticsController.java  # Analytics endpoints
│       └── PyqController.java        # PYQ endpoints
└── resources/
    └── application.properties        # Configuration
```

## Testing

```bash
mvn test
```

## Deployment

### Docker

```bash
# Build image
docker build -t lms-java .

# Run container
docker run -p 3003:3003 \
  -e SPRING_DATASOURCE_URL=jdbc:postgresql://host.docker.internal:5432/lms \
  -e JWT_SECRET=your_secret \
  lms-java
```

### Docker Compose

```yaml
version: '3.8'
services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_PASSWORD: password
      POSTGRES_DB: lms
    ports:
      - "5432:5432"

  lms-java:
    build: .
    ports:
      - "3003:3003"
    depends_on:
      - postgres
    environment:
      SPRING_DATASOURCE_URL: jdbc:postgresql://postgres:5432/lms
      JWT_SECRET: your_jwt_secret
```

## Key Dependencies

- **Spring Boot 3.1.5**: Web framework
- **Spring Data JPA**: ORM abstraction
- **Spring Security**: Security framework
- **JWT (io.jsonwebtoken)**: JWT authentication
- **PostgreSQL Driver**: Database connectivity
- **Lombok**: Boilerplate reduction

## Application Properties

| Property | Default | Description |
|----------|---------|-------------|
| `server.port` | 3003 | Server port |
| `spring.datasource.url` | - | Database URL |
| `spring.jpa.hibernate.ddl-auto` | update | Schema generation |
| `jwt.secret` | - | JWT signing secret |
| `jwt.expiration` | 604800000 | Token expiration (ms) |

## Advanced Features to Add

- [ ] Request/Response validation
- [ ] Global exception handling
- [ ] Audit logging
- [ ] Caching (Redis)
- [ ] API rate limiting
- [ ] OpenAPI/Swagger documentation
- [ ] Database migration (Flyway)
- [ ] Integration tests

## License

MIT
