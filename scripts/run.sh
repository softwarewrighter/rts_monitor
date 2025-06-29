#!/bin/bash

# Run script for RTS Monitor
# Starts the HTTP server and opens the browser

set -e  # Exit on error

echo "🚀 Starting RTS Monitor..."

# Change to project root directory
cd "$(dirname "$0")/.."

# Check if basic-http-server is installed
if ! command -v basic-http-server &> /dev/null; then
    echo "❌ basic-http-server is not installed!"
    echo "📦 Install it with: cargo install basic-http-server"
    exit 1
fi

# Start the server in the background
echo "🌐 Starting HTTP server on port 4000..."
basic-http-server -a 0.0.0.0:4000 &
SERVER_PID=$!

# Give the server a moment to start
sleep 1

# Open the browser
echo "🔗 Opening http://localhost:4000/"
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    open "http://localhost:4000/"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    xdg-open "http://localhost:4000/" 2>/dev/null || echo "Please open http://localhost:4000/ in your browser"
else
    # Windows or other
    echo "Please open http://localhost:4000/ in your browser"
fi

echo "✅ Server is running! Press Ctrl+C to stop."
echo "📝 Server PID: $SERVER_PID"

# Wait for Ctrl+C
trap "echo '🛑 Stopping server...'; kill $SERVER_PID 2>/dev/null; exit" INT TERM
wait $SERVER_PID