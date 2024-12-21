# 检查是否以管理员权限运行
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

# 获取用户主目录
$userProfile = $env:USERPROFILE

# 创建临时目录用于下载
$tempDir = Join-Path $env:TEMP "gpt-shell-install"
if (-not (Test-Path $tempDir)) {
    New-Item -ItemType Directory -Path $tempDir | Out-Null
}

# 创建目标目录
$binPath = Join-Path $userProfile "bin"
if (-not (Test-Path $binPath)) {
    Write-Host "Creating new Path: $binPath" -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $binPath | Out-Null
}

try {
    # Get Latest version
    Write-Host "checking latest version..." -ForegroundColor Cyan
    $apiUrl = "https://api.github.com/repos/wangenius/gpt-shell/releases/latest"
    $release = Invoke-RestMethod -Uri $apiUrl -Headers @{
        "Accept" = "application/vnd.github.v3+json"
        "User-Agent" = "PowerShell"
    }

    # get Windows 版本的下载链接
    $asset = $release.assets | Where-Object { $_.name -eq "gpt-windows-amd64.exe" }
    if (-not $asset) {
        throw "can't find executive application of Windows version"
    }

    # 下载文件
    $downloadPath = Join-Path $tempDir "gpt.exe"
    Write-Host "正在下载最新版本..." -ForegroundColor Cyan
    Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $downloadPath

    # 复制可执行文件到目标目录
    $targetPath = Join-Path $binPath "gpt.exe"
    Write-Host "正在安装到 $targetPath" -ForegroundColor Yellow
    Copy-Item $downloadPath $targetPath -Force

    # 检查用户 PATH 环境变量中是否已包含 bin 目录
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$binPath*") {
        Write-Host "正在添加到环境变量 PATH..." -ForegroundColor Yellow
        
        if ($userPath) {
            $newPath = "$userPath;$binPath"
        } else {
            $newPath = $binPath
        }
        
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        $env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + $newPath
    }

    Write-Host "`n安装完成！" -ForegroundColor Green
    Write-Host "现在你可以在任何目录使用 'gpt' 命令了。" -ForegroundColor Green
    Write-Host "注意：你可能需要重新打开终端才能使用 gpt 命令。" -ForegroundColor Yellow

    # 显示版本信息
    Write-Host "`n当前版本:" -ForegroundColor Cyan
    try {
        & $targetPath --version
    } catch {
        Write-Host "无法获取版本信息" -ForegroundColor Red
    }

} catch {
    Write-Host "安装过程中出现错误：" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    exit 1
} finally {
    # 清理临时文件
    if (Test-Path $tempDir) {
        Remove-Item -Path $tempDir -Recurse -Force
    }
} 