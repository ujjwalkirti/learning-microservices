package com.lms.controller;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/pyq")
public class PyqController {

    @PostMapping("/create")
    public ResponseEntity<?> createPyq(@RequestBody PyqRequest request) {
        return ResponseEntity.status(201).body(Map.of(
            "id", 1,
            "course_id", request.getCourseId(),
            "year", request.getYear()
        ));
    }

    @GetMapping("/course/{courseId}")
    public ResponseEntity<?> getPyqs(@PathVariable Integer courseId) {
        return ResponseEntity.ok(Map.of(
            "course_id", courseId,
            "pyqs", new ArrayList<>()
        ));
    }

    @GetMapping("/course/{courseId}/year/{year}")
    public ResponseEntity<?> getPyqByYear(@PathVariable Integer courseId, @PathVariable Integer year) {
        return ResponseEntity.ok(Map.of(
            "course_id", courseId,
            "year", year
        ));
    }
}

class PyqRequest {
    private Integer courseId;
    private Integer year;
    private List<String> questions;
    private Integer duration;
    private Integer totalMarks;

    // Getters and Setters
    public Integer getCourseId() { return courseId; }
    public void setCourseId(Integer courseId) { this.courseId = courseId; }
    public Integer getYear() { return year; }
    public void setYear(Integer year) { this.year = year; }
    public List<String> getQuestions() { return questions; }
    public void setQuestions(List<String> questions) { this.questions = questions; }
    public Integer getDuration() { return duration; }
    public void setDuration(Integer duration) { this.duration = duration; }
    public Integer getTotalMarks() { return totalMarks; }
    public void setTotalMarks(Integer totalMarks) { this.totalMarks = totalMarks; }
}
