const express = require('express');
const router = express.Router();

// Mock analytics database
const analytics = {};

// Record user activity
router.post('/track', (req, res) => {
  try {
    const { userId, courseId, action, timestamp } = req.body;
    const key = `${userId}-${courseId}`;

    if (!analytics[key]) {
      analytics[key] = {
        userId,
        courseId,
        activities: [],
        lastActive: timestamp
      };
    }

    analytics[key].activities.push({ action, timestamp });
    analytics[key].lastActive = timestamp;

    res.json({ message: 'Activity tracked' });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Get user progress
router.get('/progress/:userId/:courseId', (req, res) => {
  try {
    const key = `${req.params.userId}-${req.params.courseId}`;
    const data = analytics[key];

    if (!data) {
      return res.json({ userId: req.params.userId, courseId: req.params.courseId, progress: 0 });
    }

    res.json({
      userId: req.params.userId,
      courseId: req.params.courseId,
      totalActivities: data.activities.length,
      lastActive: data.lastActive
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Get course analytics
router.get('/course/:courseId', (req, res) => {
  try {
    const courseAnalytics = Object.values(analytics).filter(
      a => a.courseId == req.params.courseId
    );

    res.json({
      courseId: req.params.courseId,
      totalUsers: courseAnalytics.length,
      totalActivities: courseAnalytics.reduce((sum, a) => sum + a.activities.length, 0)
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

module.exports = router;
