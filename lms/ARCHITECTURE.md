# LMS Microservices Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                          Client Applications                      │
│                   (Web, Mobile, Desktop, Third-party)             │
└──────────────┬──────────────┬──────────────┬──────────────────────┘
               │              │              │
       ┌───────▼──────┬───────▼──────┬───────▼──────┬───────────┐
       │   Node.js    │    Rust      │      Go      │    Java   │
       │   (3000)     │    (3001)    │   (3002)     │  (3003)   │
       │  Express.js  │  Actix-web   │     Gin      │ Spring Boot
       └───────┬──────┴───────┬──────┴───────┬──────┴───────┬────┘
               │              │              │              │
               │          ┌───┴──────────────┴──────────┐   │
               │          │                            │   │
               ▼          ▼                            ▼   ▼
           ┌─────────┐  ┌──────────────────┐      ┌─────────┐
           │ MongoDB │  │   PostgreSQL     │      │ MongoDB │
           └─────────┘  └──────────────────┘      └─────────┘
               (Auth)    (CMS, Syllabus,          (Optional)
                         Analytics, PYQ)
```

## Microservices Breakdown

### 1. Node.js Service (Port 3000)
**Framework:** Express.js
**Database:** MongoDB
**Use Case:** General-purpose, JSON-friendly, rapid development

**Modules:**
- Authentication with JWT
- Course CMS
- Syllabus Management
- Analytics & Progress Tracking
- Previous Year Questions

**Strengths:**
- Non-blocking I/O
- Event-driven architecture
- Rich npm ecosystem
- Good for real-time applications

---

### 2. Rust Service (Port 3001)
**Framework:** Actix-web
**Database:** PostgreSQL
**Use Case:** High-performance, memory-safe, concurrent workloads

**Modules:**
- Same as Node.js
- Optimized async/await handling

**Strengths:**
- Zero-cost abstractions
- Memory safety without garbage collection
- Excellent for CPU-bound operations
- Best performance characteristics

---

### 3. Go Service (Port 3002)
**Framework:** Gin
**Database:** PostgreSQL
**Use Case:** Fast deployment, lightweight services, cloud-native

**Modules:**
- Same as Node.js
- Built-in concurrency with goroutines

**Strengths:**
- Simple and fast to develop
- Single binary compilation
- Great concurrency model
- Excellent for microservices

---

### 4. Java Service (Port 3003)
**Framework:** Spring Boot
**Database:** PostgreSQL
**Use Case:** Enterprise-grade, mature ecosystem, long-term support

**Modules:**
- Same as Node.js
- Spring Security integration

**Strengths:**
- Mature framework
- Extensive enterprise features
- Strong typing and safety
- Rich library ecosystem

---

## Module Architecture

Each microservice implements 5 core modules:

### Module 1: Authentication (`/api/auth`)
```
┌──────────────────────┐
│   Authentication     │
├──────────────────────┤
│ - Register User      │
│ - Login              │
│ - Logout             │
│ - JWT Token Mgmt     │
│ - Password Hashing   │
└──────────────────────┘
     ↓
  [JWT Token] ──→ Required for all other operations
```

### Module 2: Content Management System (`/api/cms`)
```
┌──────────────────────┐
│   Course CMS         │
├──────────────────────┤
│ - Create Course      │
│ - Read Courses       │
│ - Update Course      │
│ - Delete Course      │
│ - Module Management  │
└──────────────────────┘
```

### Module 3: Syllabus Management (`/api/syllabus`)
```
┌──────────────────────┐
│   Syllabus Mgmt      │
├──────────────────────┤
│ - Create Syllabus    │
│ - Define Topics      │
│ - Set Objectives     │
│ - Chapter Planning   │
│ - Duration Mgmt      │
└──────────────────────┘
```

### Module 4: Analytics & Progress (`/api/analytics`)
```
┌──────────────────────┐
│   Analytics          │
├──────────────────────┤
│ - Track Activities   │
│ - User Progress      │
│ - Course Analytics   │
│ - Engagement Stats   │
│ - Learning Patterns  │
└──────────────────────┘
```

### Module 5: Previous Year Questions (`/api/pyq`)
```
┌──────────────────────┐
│   PYQ Repository     │
├──────────────────────┤
│ - Store Questions    │
│ - Retrieve by Year   │
│ - Question Bank      │
│ - Difficulty Levels  │
│ - Solution Tracking  │
└──────────────────────┘
```

---

## Request Flow Diagram

```
┌─────────────┐
│   Client    │
└──────┬──────┘
       │
       │ HTTP Request
       ▼
┌──────────────────────┐
│  API Gateway Layer   │ (Optional in production)
│  - Load Balancing    │
│  - Request Routing   │
│  - Rate Limiting     │
└──────┬───────────────┘
       │
       │ Route to Service
       ▼
┌──────────────────────────────────┐
│   Microservice (Any Language)    │
├──────────────────────────────────┤
│ 1. Receive Request               │
│ 2. Validate JWT Token (if needed)│
│ 3. Parse Request Body            │
│ 4. Execute Business Logic        │
│ 5. Interact with Database        │
│ 6. Build Response                │
│ 7. Send Response                 │
└──────┬───────────────────────────┘
       │
       │ HTTP Response
       ▼
┌─────────────┐
│   Client    │ ◄── JSON Response
└─────────────┘
```

---

## Database Schema (Conceptual)

### MongoDB (Node.js)
```javascript
// Users Collection
{
  _id: ObjectId,
  email: String,
  passwordHash: String,
  name: String,
  role: String,  // student, instructor, admin
  createdAt: Date
}

// Courses Collection
{
  _id: ObjectId,
  title: String,
  description: String,
  instructor: String,
  duration: Number,
  modules: Array,
  createdAt: Date
}

// Analytics Collection
{
  _id: ObjectId,
  userId: String,
  courseId: ObjectId,
  activity: String,
  timestamp: Date
}
```

### PostgreSQL (Rust, Go, Java)
```sql
-- Users Table
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR UNIQUE NOT NULL,
  password_hash VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  role VARCHAR NOT NULL,
  created_at TIMESTAMP
);

-- Courses Table
CREATE TABLE courses (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description TEXT,
  instructor VARCHAR,
  duration INTEGER,
  created_at TIMESTAMP
);

-- Analytics Table
CREATE TABLE analytics (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id),
  course_id INTEGER REFERENCES courses(id),
  action VARCHAR,
  timestamp TIMESTAMP
);
```

---

## Deployment Architecture

### Single Service Deployment
```
┌─────────────────┐
│   Docker Host   │
├─────────────────┤
│ ┌─────────────┐ │
│ │ Microservice│ │
│ │  Container  │ │
│ └─────┬───────┘ │
│       ↓         │
│ ┌─────────────┐ │
│ │  Database   │ │
│ │  Container  │ │
│ └─────────────┘ │
└─────────────────┘
```

### Full Stack Deployment
```
┌──────────────────────────────────────────────┐
│            Docker Compose Environment        │
├──────────────────────────────────────────────┤
│                                              │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐     │
│  │ Node.js │  │  Rust   │  │   Go    │     │
│  │  3000   │  │  3001   │  │  3002   │     │
│  └────┬────┘  └────┬────┘  └────┬────┘     │
│       │            │            │           │
│  ┌────▼────────────▼────────────▼────┐     │
│  │   PostgreSQL + MongoDB Network    │     │
│  └───────────────────────────────────┘     │
│                                              │
│  ┌──────────────────────────────────┐      │
│  │      Shared Docker Network       │      │
│  │      Service Discovery           │      │
│  └──────────────────────────────────┘      │
│                                              │
└──────────────────────────────────────────────┘
```

---

## Data Flow Example: User Registration to Syllabus Creation

```
1. User Registration
   Client → POST /api/auth/register → Service → DB
   ↓
   Response with success message

2. User Login
   Client → POST /api/auth/login → Service → DB
   ↓
   Response with JWT token

3. Create Course
   Client → POST /api/cms/courses (with token) → Service → DB
   ↓
   Course created with ID = 1

4. Create Syllabus
   Client → POST /api/syllabus/create (with token) → Service → DB
   ↓
   Syllabus created, linked to course ID = 1

5. Track User Activity
   Client → POST /api/analytics/track → Service → DB
   ↓
   Activity logged for analytics

6. Get Progress
   Client → GET /api/analytics/progress/{userId}/1 → Service → DB
   ↓
   Return user progress data
```

---

## Performance Characteristics

| Aspect | Node.js | Rust | Go | Java |
|--------|---------|------|----|----|
| Startup Time | Fast | Very Fast | Very Fast | Slower |
| Memory Usage | Moderate | Very Low | Low | Moderate |
| Throughput | High | Very High | Very High | High |
| Latency | Low | Very Low | Very Low | Low |
| Development Speed | Fastest | Slower | Fast | Moderate |
| Type Safety | No | Strong | Moderate | Strong |
| Concurrency | Event-based | Async/Await | Goroutines | Threads |

---

## Scaling Strategies

### Horizontal Scaling
```
┌──────────────────────────────┐
│      Load Balancer           │
│      (nginx/HAProxy)         │
└──────────────┬───────────────┘
        │
    ┌───┴─────┬─────────┬────────┐
    │         │         │        │
    ▼         ▼         ▼        ▼
┌─────┐   ┌─────┐   ┌─────┐  ┌─────┐
│ :3000│   │ :3000│   │ :3000│  │:3000│
│Node1 │   │Node2 │   │Node3 │  │Node4│
└─────┘   └─────┘   └─────┘  └─────┘
    │         │         │        │
    └─────────┴─────────┴────────┘
              │
              ▼
        ┌──────────────┐
        │  Database    │
        │   (Shared)   │
        └──────────────┘
```

### Vertical Scaling
- Increase CPU/Memory per instance
- Not recommended as primary strategy

---

## Security Considerations

### Authentication
- ✅ JWT token-based authentication
- ✅ Password hashing with bcrypt
- ⚠️ TODO: Implement refresh tokens
- ⚠️ TODO: Add rate limiting

### API Security
- ✅ CORS enabled
- ⚠️ TODO: Implement HTTPS
- ⚠️ TODO: Add API key validation
- ⚠️ TODO: Implement request signing

### Data Security
- ⚠️ TODO: Encrypt sensitive data
- ⚠️ TODO: Implement database encryption
- ⚠️ TODO: Add audit logging

---

## Monitoring & Observability

```
┌────────────────────────────────────┐
│   Monitoring & Observability       │
├────────────────────────────────────┤
│                                    │
│  Logs:     ELK Stack / CloudWatch  │
│  Metrics:  Prometheus / Grafana    │
│  Tracing:  Jaeger / DataDog        │
│  Health:   Periodic health checks  │
│                                    │
└────────────────────────────────────┘
         ↑         ↑         ↑
         │         │         │
   ┌─────┴─────────┴─────────┴─────┐
   │   All Microservices Report    │
   └───────────────────────────────┘
```

---

## Future Enhancements

1. **API Gateway** - Unified entry point with routing
2. **Service Mesh** - Inter-service communication management
3. **Event Bus** - Async communication between services
4. **Cache Layer** - Redis for frequently accessed data
5. **Search Engine** - Elasticsearch for course search
6. **Message Queue** - RabbitMQ/Kafka for async tasks
7. **Container Orchestration** - Kubernetes for production
8. **CI/CD Pipeline** - Automated testing and deployment

---

## Conclusion

This microservices architecture provides:
- **Flexibility**: Choose the best tool for each service
- **Scalability**: Scale individual services independently
- **Resilience**: Failure in one service doesn't affect others
- **Learning**: Compare language implementations side-by-side
- **Production-Ready**: Can be deployed at scale
