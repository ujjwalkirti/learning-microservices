const express = require('express');
const router = express.Router();

// Mock CMS database
const courses = {};
let courseIdCounter = 1;

// Create course
router.post('/courses', (req, res) => {
  try {
    const { title, description, instructor, duration } = req.body;
    const courseId = courseIdCounter++;

    courses[courseId] = {
      id: courseId,
      title,
      description,
      instructor,
      duration,
      createdAt: new Date(),
      modules: []
    };

    res.status(201).json(courses[courseId]);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Get all courses
router.get('/courses', (req, res) => {
  try {
    res.json(Object.values(courses));
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Get course by ID
router.get('/courses/:id', (req, res) => {
  try {
    const course = courses[req.params.id];
    if (!course) {
      return res.status(404).json({ error: 'Course not found' });
    }
    res.json(course);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Update course
router.put('/courses/:id', (req, res) => {
  try {
    const course = courses[req.params.id];
    if (!course) {
      return res.status(404).json({ error: 'Course not found' });
    }

    Object.assign(course, req.body);
    res.json(course);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Delete course
router.delete('/courses/:id', (req, res) => {
  try {
    delete courses[req.params.id];
    res.json({ message: 'Course deleted successfully' });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

module.exports = router;
