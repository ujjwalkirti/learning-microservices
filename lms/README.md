# LMS Microservices

A comprehensive Learning Management System (LMS) built with microservices architecture in 4 different languages: Node.js, Rust, Go, and Java.

## Architecture Overview

Each microservice provides the following functionality:
- **Authentication**: User registration, login, logout with JWT
- **CMS**: Course content management
- **Syllabus**: Course syllabus management
- **Analytics**: User activity tracking and progress monitoring
- **PYQs**: Previous Year Questions management

## Folder Structure

```
microservices/
├── lms/
│   ├── nodejs/       # Node.js Express API (Port: 3000)
│   ├── rust/         # Rust Actix API (Port: 3001)
│   ├── go/           # Go Gin API (Port: 3002)
│   └── java/         # Java Spring Boot API (Port: 3003)
```

## Microservices

### 1. Node.js Microservice
- **Framework**: Express.js
- **Port**: 3000
- **Database**: MongoDB
- See [nodejs/README.md](./nodejs/README.md) for details

### 2. Rust Microservice
- **Framework**: Actix-web
- **Port**: 3001
- **Database**: PostgreSQL
- See [rust/README.md](./rust/README.md) for details

### 3. Go Microservice
- **Framework**: Gin
- **Port**: 3002
- **Database**: PostgreSQL
- See [go/README.md](./go/README.md) for details

### 4. Java Microservice
- **Framework**: Spring Boot
- **Port**: 3003
- **Database**: PostgreSQL
- See [java/README.md](./java/README.md) for details

## API Endpoints (Common across all microservices)

### Authentication (`/api/auth`)
- `POST /register` - Register new user
- `POST /login` - Login user
- `POST /logout` - Logout user

### CMS (`/api/cms`)
- `POST /courses` - Create course
- `GET /courses` - Get all courses
- `GET /courses/{id}` - Get course by ID
- `PUT /courses/{id}` - Update course
- `DELETE /courses/{id}` - Delete course

### Syllabus (`/api/syllabus`)
- `POST /create` - Create syllabus
- `GET /course/{courseId}` - Get syllabus for course
- `PUT /{id}` - Update syllabus

### Analytics (`/api/analytics`)
- `POST /track` - Track user activity
- `GET /progress/{userId}/{courseId}` - Get user progress
- `GET /course/{courseId}` - Get course analytics

### PYQs (`/api/pyq`)
- `POST /create` - Create PYQ
- `GET /course/{courseId}` - Get PYQs for course
- `GET /course/{courseId}/year/{year}` - Get PYQ for specific year

## Health Check
- `GET /health` - Check service status

## Getting Started

Each microservice can be run independently. Navigate to the respective language folder for setup instructions.

```bash
# Node.js
cd nodejs
npm install
npm start

# Rust
cd rust
cargo run

# Go
cd go
go run main.go

# Java
cd java
mvn clean install
mvn spring-boot:run
```

## Technology Stack

| Component | Node.js | Rust | Go | Java |
|-----------|---------|------|----|----|
| Framework | Express | Actix-web | Gin | Spring Boot |
| Runtime | Node.js | Tokio | Go Runtime | JVM |
| Database | MongoDB | PostgreSQL | PostgreSQL | PostgreSQL |
| Auth | JWT | JWT | JWT | JWT/Spring Security |
| ORM | Mongoose | - | GORM | JPA/Hibernate |

## Development

### Prerequisites
- Docker (for database containers)
- Each language's development tools installed

### Database Setup

```bash
# PostgreSQL (for Rust, Go, Java)
docker run --name lms-postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres

# MongoDB (for Node.js)
docker run --name lms-mongo -p 27017:27017 -d mongo
```

## Contributing

Each microservice is independent and can be developed, tested, and deployed separately.

## License

MIT
