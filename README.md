# Simple Http Server

Simple Http Server project to understand how to build an Http service from scratch

### Start Service

```
cargo run
```

### Commands to test

```bash
# nc command to send data
nc -v 127.0.0.1 4221 

# Curl for Health Check
curl -i -X GET http://localhost:4221

# Curl for unknown path
curl -i -X GET http://localhost:4221/index.html

# Curl for echo data back
curl -i -X GET 127.0.0.1:4221/echo/abc

# Curl to get back user agent
curl -i -X GET 127.0.0.1:4221/user-agent

# Curl to get a file
curl -i -X GET 127.0.0.1:4221/files/<filename>

# Curl to post a file
curl -i -X POST -d "hello world" localhost:4221/files/readme.txt
```