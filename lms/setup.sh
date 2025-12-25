#!/bin/bash

# LMS Microservices - Installation and Setup Script
# This script helps you get started with the LMS microservices

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                 LMS Microservices - Setup Script                  ║${NC)"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to print status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
    fi
}

echo -e "${YELLOW}Checking Prerequisites...${NC}"
echo ""

# Check Docker
if command_exists docker; then
    print_status 0 "Docker is installed"
    docker_version=$(docker --version)
    echo -e "  ${BLUE}→${NC} $docker_version"
else
    print_status 1 "Docker is not installed"
    echo -e "  ${BLUE}→${NC} Visit: https://www.docker.com/products/docker-desktop"
fi

# Check Docker Compose
if command_exists docker-compose; then
    print_status 0 "Docker Compose is installed"
    compose_version=$(docker-compose --version)
    echo -e "  ${BLUE}→${NC} $compose_version"
else
    print_status 1 "Docker Compose is not installed"
fi

echo ""
echo -e "${YELLOW}Optional Runtimes (for running services individually):${NC}"
echo ""

# Check Node.js
if command_exists node; then
    print_status 0 "Node.js is installed"
    node_version=$(node --version)
    echo -e "  ${BLUE}→${NC} $node_version"
else
    echo -e "${RED}✗${NC} Node.js is not installed (optional)"
fi

# Check Rust
if command_exists rustc; then
    print_status 0 "Rust is installed"
    rust_version=$(rustc --version)
    echo -e "  ${BLUE}→${NC} $rust_version"
else
    echo -e "${RED}✗${NC} Rust is not installed (optional)"
fi

# Check Go
if command_exists go; then
    print_status 0 "Go is installed"
    go_version=$(go version)
    echo -e "  ${BLUE}→${NC} $go_version"
else
    echo -e "${RED}✗${NC} Go is not installed (optional)"
fi

# Check Java
if command_exists java; then
    print_status 0 "Java is installed"
    java_version=$(java -version 2>&1 | head -1)
    echo -e "  ${BLUE}→${NC} $java_version"
else
    echo -e "${RED}✗${NC} Java is not installed (optional)"
fi

echo ""
echo -e "${YELLOW}Setup Options:${NC}"
echo ""
echo -e "${BLUE}1.${NC} Start all services with Docker Compose (Recommended)"
echo -e "   ${GREEN}Command:${NC} docker-compose up"
echo ""
echo -e "${BLUE}2.${NC} Start Node.js service individually"
echo -e "   ${GREEN}Commands:${NC}"
echo -e "     cd nodejs"
echo -e "     npm install"
echo -e "     npm start"
echo ""
echo -e "${BLUE}3.${NC} Start Rust service individually"
echo -e "   ${GREEN}Commands:${NC}"
echo -e "     cd rust"
echo -e "     cargo run"
echo ""
echo -e "${BLUE}4.${NC} Start Go service individually"
echo -e "   ${GREEN}Commands:${NC}"
echo -e "     cd go"
echo -e "     go run main.go"
echo ""
echo -e "${BLUE}5.${NC} Start Java service individually"
echo -e "   ${GREEN}Commands:${NC}"
echo -e "     cd java"
echo -e "     mvn spring-boot:run"
echo ""

echo -e "${YELLOW}Quick Test Commands:${NC}"
echo ""
echo -e "${GREEN}Health Check:${NC}"
echo -e "  curl http://localhost:3000/health   # Node.js"
echo -e "  curl http://localhost:3001/health   # Rust"
echo -e "  curl http://localhost:3002/health   # Go"
echo -e "  curl http://localhost:3003/health   # Java"
echo ""

echo -e "${YELLOW}Documentation:${NC}"
echo ""
echo -e "${BLUE}Start Here:${NC}"
echo -e "  ${GREEN}→${NC} README.md"
echo ""
echo -e "${BLUE}Quick Setup:${NC}"
echo -e "  ${GREEN}→${NC} QUICKSTART.md"
echo ""
echo -e "${BLUE}API Reference:${NC}"
echo -e "  ${GREEN}→${NC} API_DOCUMENTATION.md"
echo ""
echo -e "${BLUE}Architecture:${NC}"
echo -e "  ${GREEN}→${NC} ARCHITECTURE.md"
echo ""
echo -e "${BLUE}Project Summary:${NC}"
echo -e "  ${GREEN}→${NC} PROJECT_SUMMARY.md"
echo ""

echo -e "${BLUE}═══════════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "$(tput bold)Recommended Next Steps:$(tput sgr0)"
echo ""
echo -e "  ${BLUE}1.${NC} Read the README.md file"
echo -e "  ${BLUE}2.${NC} Run 'docker-compose up' to start all services"
echo -e "  ${BLUE}3.${NC} Test the health endpoints"
echo -e "  ${BLUE}4.${NC} Review API_DOCUMENTATION.md for endpoints"
echo -e "  ${BLUE}5.${NC} Make your first API call"
echo ""
echo -e "${GREEN}Happy coding! 🚀${NC}"
echo ""
