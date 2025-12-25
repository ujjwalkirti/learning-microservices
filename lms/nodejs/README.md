# LMS Microservice - Node.js

A Learning Management System microservice built with Express.js.

## Features

- ✅ Authentication (Register, Login, Logout)
- ✅ Course Management (CMS)
- ✅ Syllabus Management
- ✅ User Analytics & Progress Tracking
- ✅ Previous Year Questions (PYQs)

## Prerequisites

- Node.js 16+
- npm or yarn
- MongoDB (optional, uses in-memory storage by default)

## Installation

```bash
npm install
```

## Configuration

Create a `.env` file in the root directory:

```env
PORT=3000
DATABASE_URL=mongodb://localhost:27017/lms
JWT_SECRET=your_jwt_secret_key_here
NODE_ENV=development
```

## Running the Service

### Development Mode
```bash
npm run dev
```

### Production Mode
```bash
npm start
```

The service will start on `http://localhost:3000`

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
curl -X POST http://localhost:3000/api/auth/register \
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
curl -X POST http://localhost:3000/api/cms/courses \
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
curl -X POST http://localhost:3000/api/analytics/track \
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
src/
├── server.js           # Express app setup
├── routes/
│   ├── auth.js        # Authentication routes
│   ├── cms.js         # Course management routes
│   ├── syllabus.js    # Syllabus routes
│   ├── analytics.js   # Analytics routes
│   └── pyq.js         # PYQ routes
└── middleware/        # Custom middleware (optional)
```

## Testing

```bash
npm test
```

## Deployment

### Docker

```bash
# Build image
docker build -t lms-nodejs .

# Run container
docker run -p 3000:3000 \
  -e PORT=3000 \
  -e JWT_SECRET=your_secret \
  lms-nodejs
```

### Environment Variables

- `PORT`: Service port (default: 3000)
- `DATABASE_URL`: MongoDB connection string
- `JWT_SECRET`: Secret key for JWT signing
- `NODE_ENV`: Environment (development/production)

## Performance Considerations

- Uses in-memory storage for demo purposes
- For production, implement database persistence
- Add caching layer for frequently accessed data
- Implement rate limiting for API endpoints

## Dependencies

- **express**: Web framework
- **dotenv**: Environment variables
- **jsonwebtoken**: JWT authentication
- **bcryptjs**: Password hashing
- **cors**: Cross-origin requests
- **body-parser**: Request parsing

## License

MIT
