@echo off

start "Docker Compose Debug" /B docker-compose -f docker/docker-compose.yml up debug

echo "Press Ctrl+C to stop containers and exit."

pause >nul

docker-compose -f docker/docker-compose.yml down