package com.lms.controller;

import java.util.HashMap;
import java.util.Map;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/cms")
public class CmsController {

    @PostMapping("/courses")
    public ResponseEntity<?> createCourse(@RequestBody CourseRequest request) {
        return ResponseEntity.status(201).body(Map.of(
            "id", 1,
            "title", request.getTitle(),
            "description", request.getDescription()
        ));
    }

    @GetMapping("/courses")
    public ResponseEntity<?> getCourses() {
        return ResponseEntity.ok(new java.util.ArrayList<>());
    }

    @GetMapping("/courses/{id}")
    public ResponseEntity<?> getCourse(@PathVariable Integer id) {
        return ResponseEntity.ok(Map.of(
            "id", id,
            "title", "Sample Course"
        ));
    }

    @PutMapping("/courses/{id}")
    public ResponseEntity<?> updateCourse(@PathVariable Integer id, @RequestBody CourseRequest request) {
        return ResponseEntity.ok(Map.of(
            "id", id,
            "title", request.getTitle()
        ));
    }

    @DeleteMapping("/courses/{id}")
    public ResponseEntity<?> deleteCourse(@PathVariable Integer id) {
        return ResponseEntity.ok(Map.of("message", "Course deleted successfully"));
    }
}

class CourseRequest {
    private String title;
    private String description;
    private String instructor;
    private Integer duration;

    // Getters and Setters
    public String getTitle() { return title; }
    public void setTitle(String title) { this.title = title; }
    public String getDescription() { return description; }
    public void setDescription(String description) { this.description = description; }
    public String getInstructor() { return instructor; }
    public void setInstructor(String instructor) { this.instructor = instructor; }
    public Integer getDuration() { return duration; }
    public void setDuration(Integer duration) { this.duration = duration; }
}
