# 检查是否以管理员权限运行
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

# 获取用户主目录
$userProfile = $env:USERPROFILE

# 创建目标目录
$binPath = Join-Path $userProfile "bin"
if (-not (Test-Path $binPath)) {
    Write-Host "creating directory: $binPath" -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $binPath | Out-Null
}

# 复制可执行文件
$exePath = Join-Path $PSScriptRoot "target\release\gpt.exe"
$targetPath = Join-Path $binPath "gpt.exe"

if (-not (Test-Path $exePath)) {
    Write-Host "error: executable file not found, please run 'cargo build --release'" -ForegroundColor Red
    exit 1
}

Write-Host "copying gpt.exe to $targetPath" -ForegroundColor Yellow
Copy-Item $exePath $targetPath -Force

# 检查用户 PATH 环境变量中是否已包含 bin 目录
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$binPath*") {
    Write-Host "adding $binPath to user PATH environment variable" -ForegroundColor Yellow
    
    if ($userPath) {
        $newPath = "$userPath;$binPath"
    } else {
        $newPath = $binPath
    }
    
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    $env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + $newPath
}

Write-Host "`ninstallation complete!" -ForegroundColor Green
Write-Host "you can now use 'gpt' command in any directory." -ForegroundColor Green
Write-Host "note: you may need to reopen the terminal to use the gpt command." -ForegroundColor Yellow

# 检查是否已经构建了发布版本
if ((Get-Item $exePath).Length -lt 1MB) {
    Write-Host "`nwarning: executable file seems too small, may not be optimized. please run:" -ForegroundColor Yellow
    Write-Host "cargo build --release" -ForegroundColor Cyan
}

# 显示版本信息
Write-Host "`ncurrent version:" -ForegroundColor Cyan
try {
    & $targetPath --version
} catch {
    Write-Host "failed to get version information" -ForegroundColor Red
}