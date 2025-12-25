# API Documentation

## Overview

This is a comprehensive REST API documentation for the Learning Management System (LMS) Microservices. All microservices (Node.js, Rust, Go, Java) implement the same API contract.

## Base URLs

| Language | Base URL | Port |
|----------|----------|------|
| Node.js  | http://localhost:3000 | 3000 |
| Rust     | http://localhost:3001 | 3001 |
| Go       | http://localhost:3002 | 3002 |
| Java     | http://localhost:3003 | 3003 |

## Authentication

All endpoints except `/health`, `/api/auth/register`, and `/api/auth/login` require a valid JWT token.

**Header Format:**
```
Authorization: Bearer <JWT_TOKEN>
```

---

## Endpoints

### Health Check

#### Check Service Status
```
GET /health
```

**Response (200):**
```json
{
  "status": "LMS Microservice - {language} is running"
}
```

---

## Authentication Module (`/api/auth`)

### Register User
```
POST /api/auth/register
```

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "password123",
  "name": "John Doe",
  "role": "student"
}
```

**Response (201):**
```json
{
  "message": "User registered successfully"
}
```

**Error (400):**
```json
{
  "error": "User already exists"
}
```

---

### Login
```
POST /api/auth/login
```

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response (200):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "email": "user@example.com",
    "name": "John Doe",
    "role": "student"
  }
}
```

**Error (401):**
```json
{
  "error": "Invalid credentials"
}
```

---

### Logout
```
POST /api/auth/logout
```

**Headers:**
```
Authorization: Bearer <JWT_TOKEN>
```

**Response (200):**
```json
{
  "message": "Logged out successfully"
}
```

---

## Content Management System (CMS) Module (`/api/cms`)

### Create Course
```
POST /api/cms/courses
```

**Request Body:**
```json
{
  "title": "Web Development 101",
  "description": "Learn web development fundamentals",
  "instructor": "Jane Doe",
  "duration": 12
}
```

**Response (201):**
```json
{
  "id": 1,
  "title": "Web Development 101",
  "description": "Learn web development fundamentals",
  "instructor": "Jane Doe",
  "duration": 12,
  "createdAt": "2024-01-15T10:30:00Z",
  "modules": []
}
```

---

### Get All Courses
```
GET /api/cms/courses
```

**Response (200):**
```json
[
  {
    "id": 1,
    "title": "Web Development 101",
    "description": "Learn web development fundamentals",
    "instructor": "Jane Doe",
    "duration": 12
  }
]
```

---

### Get Course by ID
```
GET /api/cms/courses/{id}
```

**Response (200):**
```json
{
  "id": 1,
  "title": "Web Development 101",
  "description": "Learn web development fundamentals",
  "instructor": "Jane Doe",
  "duration": 12
}
```

**Error (404):**
```json
{
  "error": "Course not found"
}
```

---

### Update Course
```
PUT /api/cms/courses/{id}
```

**Request Body:**
```json
{
  "title": "Web Development 201",
  "description": "Advanced web development",
  "instructor": "Jane Doe",
  "duration": 16
}
```

**Response (200):**
```json
{
  "id": 1,
  "title": "Web Development 201",
  "description": "Advanced web development",
  "instructor": "Jane Doe",
  "duration": 16
}
```

---

### Delete Course
```
DELETE /api/cms/courses/{id}
```

**Response (200):**
```json
{
  "message": "Course deleted successfully"
}
```

---

## Syllabus Module (`/api/syllabus`)

### Create Syllabus
```
POST /api/syllabus/create
```

**Request Body:**
```json
{
  "courseId": 1,
  "topics": ["HTML", "CSS", "JavaScript"],
  "duration": 12,
  "objectives": ["Learn web basics", "Build websites"]
}
```

**Response (201):**
```json
{
  "id": 1,
  "courseId": 1,
  "topics": ["HTML", "CSS", "JavaScript"],
  "duration": 12,
  "objectives": ["Learn web basics", "Build websites"],
  "createdAt": "2024-01-15T10:30:00Z",
  "chapters": []
}
```

---

### Get Syllabus for Course
```
GET /api/syllabus/course/{courseId}
```

**Response (200):**
```json
[
  {
    "id": 1,
    "courseId": 1,
    "topics": ["HTML", "CSS", "JavaScript"],
    "duration": 12,
    "objectives": ["Learn web basics", "Build websites"]
  }
]
```

---

### Update Syllabus
```
PUT /api/syllabus/{id}
```

**Request Body:**
```json
{
  "topics": ["HTML5", "CSS3", "JavaScript ES6"],
  "duration": 14
}
```

**Response (200):**
```json
{
  "id": 1,
  "courseId": 1,
  "topics": ["HTML5", "CSS3", "JavaScript ES6"],
  "duration": 14
}
```

---

## Analytics Module (`/api/analytics`)

### Track User Activity
```
POST /api/analytics/track
```

**Request Body:**
```json
{
  "userId": "user123",
  "courseId": 1,
  "action": "video_watched",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

**Response (200):**
```json
{
  "message": "Activity tracked"
}
```

---

### Get User Progress
```
GET /api/analytics/progress/{userId}/{courseId}
```

**Response (200):**
```json
{
  "userId": "user123",
  "courseId": 1,
  "totalActivities": 15,
  "lastActive": "2024-01-15T10:30:00Z"
}
```

---

### Get Course Analytics
```
GET /api/analytics/course/{courseId}
```

**Response (200):**
```json
{
  "courseId": 1,
  "totalUsers": 42,
  "totalActivities": 523
}
```

---

## Previous Year Questions (PYQ) Module (`/api/pyq`)

### Create PYQ
```
POST /api/pyq/create
```

**Request Body:**
```json
{
  "courseId": 1,
  "year": 2023,
  "questions": ["Question 1", "Question 2"],
  "duration": 120,
  "totalMarks": 100
}
```

**Response (201):**
```json
{
  "id": 1,
  "courseId": 1,
  "year": 2023,
  "questions": ["Question 1", "Question 2"],
  "duration": 120,
  "totalMarks": 100,
  "createdAt": "2024-01-15T10:30:00Z"
}
```

---

### Get PYQs for Course
```
GET /api/pyq/course/{courseId}
```

**Response (200):**
```json
[
  {
    "id": 1,
    "courseId": 1,
    "year": 2023,
    "questions": ["Question 1", "Question 2"],
    "duration": 120,
    "totalMarks": 100
  },
  {
    "id": 2,
    "courseId": 1,
    "year": 2022,
    "questions": ["Question 1", "Question 2"],
    "duration": 120,
    "totalMarks": 100
  }
]
```

---

### Get PYQ by Year
```
GET /api/pyq/course/{courseId}/year/{year}
```

**Response (200):**
```json
{
  "id": 1,
  "courseId": 1,
  "year": 2023,
  "questions": ["Question 1", "Question 2"],
  "duration": 120,
  "totalMarks": 100
}
```

**Error (404):**
```json
{
  "error": "PYQ not found"
}
```

---

## Error Responses

### Standard Error Format

All endpoints return errors in the following format:

```json
{
  "error": "Error message",
  "message": "Additional details (optional)"
}
```

### Common HTTP Status Codes

| Code | Meaning |
|------|---------|
| 200 | OK - Request successful |
| 201 | Created - Resource created |
| 400 | Bad Request - Invalid input |
| 401 | Unauthorized - Missing/invalid token |
| 404 | Not Found - Resource not found |
| 500 | Internal Server Error |

---

## Response Codes by Endpoint

| Endpoint | 200 | 201 | 400 | 401 | 404 | 500 |
|----------|-----|-----|-----|-----|-----|-----|
| POST /register | | ✓ | ✓ | | | ✓ |
| POST /login | ✓ | | ✓ | ✓ | | ✓ |
| POST /logout | ✓ | | | ✓ | | ✓ |
| POST /courses | | ✓ | ✓ | ✓ | | ✓ |
| GET /courses | ✓ | | | ✓ | | ✓ |
| GET /courses/{id} | ✓ | | | ✓ | ✓ | ✓ |
| PUT /courses/{id} | ✓ | | ✓ | ✓ | ✓ | ✓ |
| DELETE /courses/{id} | ✓ | | | ✓ | ✓ | ✓ |
| POST /syllabus/create | | ✓ | ✓ | ✓ | | ✓ |
| GET /syllabus/course/{id} | ✓ | | | ✓ | | ✓ |
| PUT /syllabus/{id} | ✓ | | ✓ | ✓ | ✓ | ✓ |
| POST /analytics/track | ✓ | | ✓ | ✓ | | ✓ |
| GET /analytics/progress/{userId}/{courseId} | ✓ | | | ✓ | | ✓ |
| GET /analytics/course/{courseId} | ✓ | | | ✓ | | ✓ |
| POST /pyq/create | | ✓ | ✓ | ✓ | | ✓ |
| GET /pyq/course/{courseId} | ✓ | | | ✓ | | ✓ |
| GET /pyq/course/{courseId}/year/{year} | ✓ | | | ✓ | ✓ | ✓ |

---

## Rate Limiting

Currently, there is no rate limiting implemented. For production deployment, consider implementing:
- Request throttling
- Per-user rate limits
- IP-based rate limiting

---

## CORS Support

All services support Cross-Origin Resource Sharing (CORS). You can make requests from any origin.

---

## Testing

Use the provided cURL examples or import the API documentation into tools like:
- Postman
- Insomnia
- Thunder Client
- VS Code REST Client

---

## Versioning

Current API Version: **v1**

Future versions will be accessible at `/api/v2/`, etc.
