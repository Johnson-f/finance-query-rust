#!/bin/bash

# Finance Query Rust Startup Script
# This script starts the Rust finance query server.

set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Starting Finance Query Rust Server${NC}"
echo "=========================================="

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Navigate to the project directory (where this script is located)
echo -e "${BLUE}📁 Navigating to project directory...${NC}"
cd "$SCRIPT_DIR"
echo "Current directory: $(pwd)"

# Check if Cargo.toml exists
if [ ! -f "Cargo.toml" ]; then
    echo -e "${YELLOW}⚠️  Warning: Cargo.toml not found in current directory${NC}"
    echo "Please ensure you're running this script from the project root."
    exit 1
fi

# Start the server with backtrace enabled
echo -e "${GREEN}🚀 Starting Rust server with backtrace enabled...${NC}"
echo -e "${YELLOW}⚠️  RUST_BACKTRACE is enabled for debugging${NC}"
echo -e "${BLUE}📡 Server will be available at http://0.0.0.0:8080${NC}"
export RUST_BACKTRACE=1
RUST_LOG=info cargo run