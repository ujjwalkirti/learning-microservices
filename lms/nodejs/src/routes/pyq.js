const express = require('express');
const router = express.Router();

// Mock PYQ database
const pyqs = {};
let pyqIdCounter = 1;

// Create PYQ
router.post('/create', (req, res) => {
  try {
    const { courseId, year, questions, duration, totalMarks } = req.body;
    const pyqId = pyqIdCounter++;

    pyqs[pyqId] = {
      id: pyqId,
      courseId,
      year,
      questions,
      duration,
      totalMarks,
      createdAt: new Date()
    };

    res.status(201).json(pyqs[pyqId]);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Get PYQs by course
router.get('/course/:courseId', (req, res) => {
  try {
    const coursePyqs = Object.values(pyqs).filter(
      p => p.courseId == req.params.courseId
    );
    res.json(coursePyqs);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Get PYQ by year
router.get('/course/:courseId/year/:year', (req, res) => {
  try {
    const pyq = Object.values(pyqs).find(
      p => p.courseId == req.params.courseId && p.year == req.params.year
    );

    if (!pyq) {
      return res.status(404).json({ error: 'PYQ not found' });
    }

    res.json(pyq);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Get all PYQs
router.get('/', (req, res) => {
  try {
    res.json(Object.values(pyqs));
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

module.exports = router;
