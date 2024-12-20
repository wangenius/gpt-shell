# 检查是否以管理员权限运行
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

# 获取用户主目录
$userProfile = $env:USERPROFILE

# 创建目标目录
$binPath = Join-Path $userProfile "bin"
if (-not (Test-Path $binPath)) {
    Write-Host "创建目录: $binPath" -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $binPath | Out-Null
}

# 复制可执行文件
$exePath = Join-Path $PSScriptRoot "target\release\gpt.exe"
$targetPath = Join-Path $binPath "gpt.exe"

if (-not (Test-Path $exePath)) {
    Write-Host "错误: 未找到可执行文件，请先运行 'cargo build --release'" -ForegroundColor Red
    exit 1
}

Write-Host "复制 gpt.exe 到 $targetPath" -ForegroundColor Yellow
Copy-Item $exePath $targetPath -Force

# 检查用户 PATH 环境变量中是否已包含 bin 目录
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$binPath*") {
    Write-Host "添加 $binPath 到用户 PATH 环境变量" -ForegroundColor Yellow
    
    if ($userPath) {
        $newPath = "$userPath;$binPath"
    } else {
        $newPath = $binPath
    }
    
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    $env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + $newPath
}

Write-Host "`n安装完成！" -ForegroundColor Green
Write-Host "你现在可以在任何目录使用 'gpt' 命令了。" -ForegroundColor Green
Write-Host "注意：你可能需要重新打开终端才能使用 gpt 命令。" -ForegroundColor Yellow

# 检查是否已经构建了发布版本
if ((Get-Item $exePath).Length -lt 1MB) {
    Write-Host "`n警告: 可执行文件似乎太小，可能未优化。建议重新运行:" -ForegroundColor Yellow
    Write-Host "cargo build --release" -ForegroundColor Cyan
}

# 显示版本信息
Write-Host "`n当前版本:" -ForegroundColor Cyan
try {
    & $targetPath --version
} catch {
    Write-Host "无法获取版本信息" -ForegroundColor Red
} 