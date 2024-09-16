@echo off
REM This script compiles all files except .bat, .sh, and .spirv files in the current directory to SPIR-V using glslc.

REM Check if glslc is available
where glslc >nul 2>&1
if errorlevel 1 (
    echo glslc not found. Please install it and make sure it's in your PATH.
    exit /b 1
)

REM Loop through all files in the current directory, excluding .bat, .sh, and .spirv
for %%f in (*) do (
    REM Get the file extension
    set "ext=%%~xf"
    
    REM Skip .bat, .sh, and .spirv files
    if /i not "%%~xf"==".cmd" if /i not "%%~xf"==".sh" if /i not "%%~xf"==".spirv" (
        REM Get the filename without extension
        set "filename=%%~nf"
        
        echo Compiling %%f to %%~nf.spirv...
        glslc %%f -o %%~nf.spirv
        if errorlevel 1 (
            echo Failed to compile %%f
            exit /b 1
        )
    )
)

echo All eligible files compiled successfully.
pause