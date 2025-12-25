require('dotenv').config();
const express = require('express');
const cors = require('cors');
const bodyParser = require('body-parser');

const app = express();

// Middleware
app.use(cors());
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

// Import routes
const authRoutes = require('./routes/auth');
const cmsRoutes = require('./routes/cms');
const syllabusRoutes = require('./routes/syllabus');
const analyticsRoutes = require('./routes/analytics');
const pyqRoutes = require('./routes/pyq');

// Health check
app.get('/health', (req, res) => {
  res.json({ status: 'LMS Microservice - Node.js is running' });
});

// Routes
app.use('/api/auth', authRoutes);
app.use('/api/cms', cmsRoutes);
app.use('/api/syllabus', syllabusRoutes);
app.use('/api/analytics', analyticsRoutes);
app.use('/api/pyq', pyqRoutes);

// Error handling middleware
app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(500).json({ error: 'Internal Server Error', message: err.message });
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`LMS Microservice listening on port ${PORT}`);
});
