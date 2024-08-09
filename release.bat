@echo off

start "Docker Compose Release" /B docker-compose -f docker/docker-compose.yml up release

echo "Press Ctrl+C to stop containers and exit."

pause >nul

docker-compose -f docker/docker-compose.yml down