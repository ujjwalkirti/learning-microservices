const express = require('express');
const router = express.Router();

// Mock syllabus database
const syllabi = {};
let syllabusIdCounter = 1;

// Create syllabus
router.post('/create', (req, res) => {
  try {
    const { courseId, topics, duration, objectives } = req.body;
    const syllabusId = syllabusIdCounter++;

    syllabi[syllabusId] = {
      id: syllabusId,
      courseId,
      topics,
      duration,
      objectives,
      createdAt: new Date(),
      chapters: []
    };

    res.status(201).json(syllabi[syllabusId]);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Get syllabus by course
router.get('/course/:courseId', (req, res) => {
  try {
    const courseSyllabi = Object.values(syllabi).filter(
      s => s.courseId == req.params.courseId
    );
    res.json(courseSyllabi);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Get all syllabi
router.get('/', (req, res) => {
  try {
    res.json(Object.values(syllabi));
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Update syllabus
router.put('/:id', (req, res) => {
  try {
    const syllabus = syllabi[req.params.id];
    if (!syllabus) {
      return res.status(404).json({ error: 'Syllabus not found' });
    }

    Object.assign(syllabus, req.body);
    res.json(syllabus);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

module.exports = router;
