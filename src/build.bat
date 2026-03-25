@echo off
if not exist bin mkdir bin

echo Compiling Genesis Engine (VM + Darwin + Bio + Arena)...
g++ -std=c++17 -O2 src/main.cpp src/vm.cpp src/darwin.cpp src/bio.cpp src/arena.cpp -o bin/genesis.exe

if %errorlevel% neq 0 (
    echo Build Failed!
    exit /b %errorlevel%
)

echo Build Success! Run bin\genesis.exe
