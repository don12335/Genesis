@echo off
echo =========================================
echo Genesis Complete Edition - GitHub Uploader
echo =========================================
echo.

echo [1/5] Initializing Git repository...
git init

echo [2/5] Adding specified files...
git add .

echo [3/5] Committing changes...
git commit -m "Update"

echo [4/5] Setting up remote repository...
git branch -M main
git remote add origin https://github.com/don12335/Genesis.git

echo [5/5] Pushing to GitHub (Force push to overwrite old version)...
git push -u origin main --force

echo.
echo =========================================
echo Upload Complete! 
echo Check your repository at: https://github.com/don12335/Genesis
echo =========================================
pause
