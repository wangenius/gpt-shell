param (
    [Parameter(Mandatory=$false)]
    [string]$Version
)

# 从 Cargo.toml 获取当前版本号
$cargoToml = Get-Content "Cargo.toml" -Raw
if ($cargoToml -match 'version = "(\d+)\.(\d+)\.(\d+)"') {
    $major = [int]$matches[1]
    $minor = [int]$matches[2]
    $patch = [int]$matches[3]
    $currentVersion = "$major.$minor.$patch"
} else {
    Write-Host "错误：无法从 Cargo.toml 中获取当前版本号。" -ForegroundColor Red
    exit 1
}

# 如果没有提供版本号，自动递增最后一位
if (-not $Version) {
    $patch++
    $Version = "$major.$minor.$patch"
    Write-Host "当前版本：$currentVersion" -ForegroundColor Cyan
    Write-Host "新版本：$Version" -ForegroundColor Green
}
# 如果提供了版本号，检查格式
elseif ($Version -notmatch '^\d+\.\d+\.\d+$') {
    Write-Host "错误：版本号格式不正确。应该是 x.y.z 格式，例如：1.0.0" -ForegroundColor Red
    exit 1
}

# 检查是否有未提交的更改
$status = git status --porcelain
if ($status) {
    Write-Host "错误：有未提交的更改。请先提交或存储这些更改。" -ForegroundColor Red
    git status
    exit 1
}

# 检查当前分支是否是 master
$currentBranch = git rev-parse --abbrev-ref HEAD
if ($currentBranch -ne "master") {
    Write-Host "错误：不在 master 分支上。请切换到 master 分支。" -ForegroundColor Red
    exit 1
}

try {
    # 更新 Cargo.toml 中的版本号
    Write-Host "正在更新 Cargo.toml 中的版本号..." -ForegroundColor Cyan
    $newCargoToml = $cargoToml -replace 'version = "\d+\.\d+\.\d+"', "version = `"$Version`""
    Set-Content "Cargo.toml" $newCargoToml -NoNewline

    # 提交更改
    Write-Host "正在提交更改..." -ForegroundColor Cyan
    git add Cargo.toml
    git commit -m "bump version to $Version"

    # 推送更改
    Write-Host "正在推送更改..." -ForegroundColor Cyan
    git push

    # 创建并推送标签
    Write-Host "正在创建标签 v$Version..." -ForegroundColor Cyan
    git tag "v$Version"
    git push origin master "v$Version"

    Write-Host "`n✨ 发布流程完成！" -ForegroundColor Green
    Write-Host "版本号已更新为：$Version" -ForegroundColor Green
    Write-Host "标签 v$Version 已创建并推送" -ForegroundColor Green
    Write-Host "`n正在等待 GitHub Actions 构建..." -ForegroundColor Yellow
    Write-Host "你可以在这里查看构建进度：https://github.com/wangenius/gpt-shell/actions" -ForegroundColor Yellow

} catch {
    Write-Host "`n❌ 发布过程中出现错误：" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    
    # 尝试回滚更改
    Write-Host "`n正在尝试回滚更改..." -ForegroundColor Yellow
    git reset --hard HEAD^
    git tag -d "v$Version" 2>$null
    
    exit 1
} 