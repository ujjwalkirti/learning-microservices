.PHONY: help build up down logs clean test install

# Colors for output
RED=\033[0;31m
GREEN=\033[0;32m
YELLOW=\033[0;33m
BLUE=\033[0;34m
NC=\033[0m # No Color

help:
	@echo "$(BLUE)╔════════════════════════════════════════════════════════════════╗$(NC)"
	@echo "$(BLUE)║     LMS Microservices - Available Commands                      ║$(NC)"
	@echo "$(BLUE)╚════════════════════════════════════════════════════════════════╝$(NC)"
	@echo ""
	@echo "$(GREEN)Docker Compose Commands:$(NC)"
	@echo "  make up              - Start all services with Docker Compose"
	@echo "  make down            - Stop all services"
	@echo "  make logs            - View logs from all services"
	@echo "  make clean           - Remove containers, volumes, and networks"
	@echo "  make rebuild         - Rebuild all Docker images"
	@echo ""
	@echo "$(GREEN)Service Setup Commands:$(NC)"
	@echo "  make install-node    - Install Node.js dependencies"
	@echo "  make install-rust    - Build Rust service"
	@echo "  make install-go      - Download Go dependencies"
	@echo "  make install-java    - Build Java service with Maven"
	@echo ""
	@echo "$(GREEN)Service Run Commands:$(NC)"
	@echo "  make run-node        - Start Node.js service"
	@echo "  make run-rust        - Start Rust service"
	@echo "  make run-go          - Start Go service"
	@echo "  make run-java        - Start Java service"
	@echo ""
	@echo "$(GREEN)Testing Commands:$(NC)"
	@echo "  make health          - Check health of all services"
	@echo "  make test-node       - Run Node.js tests"
	@echo "  make test-rust       - Run Rust tests"
	@echo "  make test-go         - Run Go tests"
	@echo "  make test-java       - Run Java tests"
	@echo ""
	@echo "$(GREEN)Documentation:$(NC)"
	@echo "  make docs            - Display project documentation paths"
	@echo ""

# Docker Compose Commands
up:
	@echo "$(YELLOW)Starting all services with Docker Compose...$(NC)"
	docker-compose up -d
	@echo "$(GREEN)✓ Services started$(NC)"
	@echo "$(BLUE)Access services at:$(NC)"
	@echo "  Node.js: http://localhost:3000"
	@echo "  Rust:    http://localhost:3001"
	@echo "  Go:      http://localhost:3002"
	@echo "  Java:    http://localhost:3003"

down:
	@echo "$(YELLOW)Stopping all services...$(NC)"
	docker-compose down
	@echo "$(GREEN)✓ Services stopped$(NC)"

logs:
	docker-compose logs -f

logs-node:
	docker-compose logs -f lms-nodejs

logs-rust:
	docker-compose logs -f lms-rust

logs-go:
	docker-compose logs -f lms-go

logs-java:
	docker-compose logs -f lms-java

rebuild:
	@echo "$(YELLOW)Rebuilding all Docker images...$(NC)"
	docker-compose build --no-cache
	@echo "$(GREEN)✓ Images rebuilt$(NC)"

clean:
	@echo "$(RED)Cleaning up Docker resources...$(NC)"
	docker-compose down -v
	docker system prune -f
	@echo "$(GREEN)✓ Cleanup complete$(NC)"

# Installation Commands
install-node:
	@echo "$(YELLOW)Installing Node.js dependencies...$(NC)"
	cd nodejs && npm install
	@echo "$(GREEN)✓ Node.js dependencies installed$(NC)"

install-rust:
	@echo "$(YELLOW)Building Rust project...$(NC)"
	cd rust && cargo build
	@echo "$(GREEN)✓ Rust project built$(NC)"

install-go:
	@echo "$(YELLOW)Downloading Go dependencies...$(NC)"
	cd go && go mod download
	@echo "$(GREEN)✓ Go dependencies downloaded$(NC)"

install-java:
	@echo "$(YELLOW)Building Java project with Maven...$(NC)"
	cd java && mvn clean install
	@echo "$(GREEN)✓ Java project built$(NC)"

install-all: install-node install-rust install-go install-java
	@echo "$(GREEN)✓ All services installed$(NC)"

# Run Commands
run-node:
	@echo "$(YELLOW)Starting Node.js service...$(NC)"
	cd nodejs && npm start

run-node-dev:
	@echo "$(YELLOW)Starting Node.js service in dev mode...$(NC)"
	cd nodejs && npm run dev

run-rust:
	@echo "$(YELLOW)Starting Rust service...$(NC)"
	cd rust && cargo run

run-go:
	@echo "$(YELLOW)Starting Go service...$(NC)"
	cd go && go run main.go

run-java:
	@echo "$(YELLOW)Starting Java service...$(NC)"
	cd java && mvn spring-boot:run

# Testing Commands
health:
	@echo "$(BLUE)Checking service health...$(NC)"
	@echo ""
	@echo "Node.js (3000):"
	@curl -s http://localhost:3000/health | grep -q "status" && echo "$(GREEN)✓ Running$(NC)" || echo "$(RED)✗ Not running$(NC)"
	@echo ""
	@echo "Rust (3001):"
	@curl -s http://localhost:3001/health | grep -q "status" && echo "$(GREEN)✓ Running$(NC)" || echo "$(RED)✗ Not running$(NC)"
	@echo ""
	@echo "Go (3002):"
	@curl -s http://localhost:3002/health | grep -q "status" && echo "$(GREEN)✓ Running$(NC)" || echo "$(RED)✗ Not running$(NC)"
	@echo ""
	@echo "Java (3003):"
	@curl -s http://localhost:3003/health | grep -q "status" && echo "$(GREEN)✓ Running$(NC)" || echo "$(RED)✗ Not running$(NC)"

test-node:
	@echo "$(YELLOW)Running Node.js tests...$(NC)"
	cd nodejs && npm test

test-rust:
	@echo "$(YELLOW)Running Rust tests...$(NC)"
	cd rust && cargo test

test-go:
	@echo "$(YELLOW)Running Go tests...$(NC)"
	cd go && go test ./...

test-java:
	@echo "$(YELLOW)Running Java tests...$(NC)"
	cd java && mvn test

test-all: test-node test-rust test-go test-java
	@echo "$(GREEN)✓ All tests completed$(NC)"

# API Test Commands
test-register:
	@echo "$(BLUE)Testing user registration...$(NC)"
	curl -X POST http://localhost:3000/api/auth/register \
	  -H "Content-Type: application/json" \
	  -d '{"email":"test@example.com","password":"test123","name":"Test User","role":"student"}'

test-login:
	@echo "$(BLUE)Testing user login...$(NC)"
	curl -X POST http://localhost:3000/api/auth/login \
	  -H "Content-Type: application/json" \
	  -d '{"email":"test@example.com","password":"test123"}'

test-create-course:
	@echo "$(BLUE)Testing course creation...$(NC)"
	curl -X POST http://localhost:3000/api/cms/courses \
	  -H "Content-Type: application/json" \
	  -d '{"title":"Web Dev 101","description":"Learn web development","instructor":"Jane Doe","duration":12}'

test-get-courses:
	@echo "$(BLUE)Testing get courses...$(NC)"
	curl http://localhost:3000/api/cms/courses

# Documentation
docs:
	@echo "$(BLUE)╔════════════════════════════════════════════════════════════════╗$(NC)"
	@echo "$(BLUE)║     Documentation Files                                         ║$(NC)"
	@echo "$(BLUE)╚════════════════════════════════════════════════════════════════╝$(NC)"
	@echo ""
	@echo "$(GREEN)Main Documentation:$(NC)"
	@echo "  - README.md                  - Project overview"
	@echo "  - PROJECT_SUMMARY.md         - Complete project summary"
	@echo "  - QUICKSTART.md              - Getting started guide"
	@echo "  - API_DOCUMENTATION.md       - Complete API reference"
	@echo "  - ARCHITECTURE.md            - System architecture"
	@echo ""
	@echo "$(GREEN)Service-Specific READMEs:$(NC)"
	@echo "  - nodejs/README.md           - Node.js service guide"
	@echo "  - rust/README.md             - Rust service guide"
	@echo "  - go/README.md               - Go service guide"
	@echo "  - java/README.md             - Java service guide"
	@echo ""
	@echo "$(GREEN)Other Files:$(NC)"
	@echo "  - docker-compose.yml         - Docker Compose configuration"
	@echo "  - Makefile                   - This makefile"
	@echo "  - .gitignore                 - Git ignore rules"
	@echo ""

# Status
status:
	@echo "$(BLUE)Service Status:$(NC)"
	docker ps --format "table {{.Names}}\t{{.Status}}" | grep lms

# Database Commands
db-migrate:
	@echo "$(YELLOW)Running database migrations...$(NC)"
	@echo "Note: Implement migration scripts in each service"

# Development Commands
dev-setup:
	@echo "$(YELLOW)Setting up development environment...$(NC)"
	mkdir -p logs
	@echo "$(GREEN)✓ Development environment ready$(NC)"

# Version Info
version:
	@echo "$(BLUE)Component Versions:$(NC)"
	@echo "Node.js: $$(node --version)" 2>/dev/null || echo "Node.js: Not installed"
	@echo "Rust: $$(rustc --version)" 2>/dev/null || echo "Rust: Not installed"
	@echo "Go: $$(go version)" 2>/dev/null || echo "Go: Not installed"
	@echo "Java: $$(java -version 2>&1)" 2>/dev/null || echo "Java: Not installed"
	@echo "Docker: $$(docker --version)"
	@echo "Docker Compose: $$(docker-compose --version)"

# Default target
.DEFAULT_GOAL := help
