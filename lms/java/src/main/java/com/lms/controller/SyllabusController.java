package com.lms.controller;

import java.util.ArrayList;
import java.util.Map;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/syllabus")
public class SyllabusController {

    @PostMapping("/create")
    public ResponseEntity<?> createSyllabus(@RequestBody SyllabusRequest request) {
        return ResponseEntity.status(201).body(Map.of(
            "id", 1,
            "course_id", request.getCourseId(),
            "topics", request.getTopics()
        ));
    }

    @GetMapping("/course/{courseId}")
    public ResponseEntity<?> getSyllabus(@PathVariable Integer courseId) {
        return ResponseEntity.ok(Map.of(
            "course_id", courseId,
            "syllabi", new ArrayList<>()
        ));
    }

    @PutMapping("/{id}")
    public ResponseEntity<?> updateSyllabus(@PathVariable Integer id, @RequestBody SyllabusRequest request) {
        return ResponseEntity.ok(Map.of(
            "id", id,
            "message", "Syllabus updated"
        ));
    }
}

class SyllabusRequest {
    private Integer courseId;
    private java.util.List<String> topics;
    private Integer duration;
    private java.util.List<String> objectives;

    // Getters and Setters
    public Integer getCourseId() { return courseId; }
    public void setCourseId(Integer courseId) { this.courseId = courseId; }
    public java.util.List<String> getTopics() { return topics; }
    public void setTopics(java.util.List<String> topics) { this.topics = topics; }
    public Integer getDuration() { return duration; }
    public void setDuration(Integer duration) { this.duration = duration; }
    public java.util.List<String> getObjectives() { return objectives; }
    public void setObjectives(java.util.List<String> objectives) { this.objectives = objectives; }
}
