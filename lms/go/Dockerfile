FROM golang:1.21-alpine AS builder

WORKDIR /app

COPY go.mod go.sum ./

RUN go mod download

COPY . .

RUN go build -o lms-service .

FROM alpine:latest

WORKDIR /root/

COPY --from=builder /app/lms-service .

EXPOSE 3002

CMD ["./lms-service"]
