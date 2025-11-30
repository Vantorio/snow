#!/bin/bash

PROJECT_NAME="snow"
RUST_INSTALLER="https://sh.rustup.rs"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

check_rust_installation() {
    if command -v rustc &> /dev/null
    then
        echo -e "${GREEN}We have verified that Rust is installed in your machine.${NC}"
    else
        echo -e "${RED}Rust is NOT installed.${NC}"
        echo -e "${YELLOW}Please run the following command to install Rust:${NC}"
        echo ""
        echo "   curl --proto '=https' --tlsv1.2 -sSf $RUST_INSTALLER | sh"
        echo ""
        exit 1
    fi
}

compile_project() {
    echo -e "${CYAN}Compiling project in Release (Optimized) mode...${NC}"
    
    cargo build --release
    
    if [[ $? -ne 0 ]]; then
        echo -e "${RED}Compilation failed!${NC}"
        exit 1
    fi
    echo -e "${GREEN}Compilation successful.${NC}"
}

get_execution_path() {
    echo "target/release/$PROJECT_NAME"
}

check_rust_installation

EXECUTABLE_PATH=$(get_execution_path)

if [[ ! -f "$EXECUTABLE_PATH" ]]; then
    echo -e "${YELLOW}Executable not found at '$EXECUTABLE_PATH'.${NC}"
    
    read -r -p "Would you like to compile the server in Release mode now? (y/N). Enter to default (N): " INPUT_COMPILE
    
    if [[ "$INPUT_COMPILE" == "y" || "$INPUT_COMPILE" == "Y" ]]; then
        compile_project
    else
        echo -e "${RED}Aborting. Please compile the server first.${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}Executable found. Skipping compilation.${NC}"
fi

echo -e "${YELLOW}Starting Snow Server...${NC}"
"$EXECUTABLE_PATH"