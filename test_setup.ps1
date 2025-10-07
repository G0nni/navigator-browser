# Script de vérification de l'environnement Navigator sur Windows

Write-Host "=== Vérification de l'environnement Navigator ===" -ForegroundColor Cyan

# Rust
Write-Host "`n[1/5] Vérification de Rust..." -ForegroundColor Yellow
if (Get-Command rustc -ErrorAction SilentlyContinue) {
    $rustVersion = rustc --version
    Write-Host "✅ $rustVersion" -ForegroundColor Green
} else {
    Write-Host "❌ Rust non trouvé - Installer depuis https://rustup.rs/" -ForegroundColor Red
    $script:hasErrors = $true
}

# Cargo
Write-Host "`n[2/5] Vérification de Cargo..." -ForegroundColor Yellow
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    $cargoVersion = cargo --version
    Write-Host "✅ $cargoVersion" -ForegroundColor Green
} else {
    Write-Host "❌ Cargo non trouvé" -ForegroundColor Red
    $script:hasErrors = $true
}

# pkg-config
Write-Host "`n[3/5] Vérification de pkg-config..." -ForegroundColor Yellow
if (Get-Command pkg-config -ErrorAction SilentlyContinue) {
    $pkgVersion = pkg-config --version
    Write-Host "✅ pkg-config $pkgVersion installé" -ForegroundColor Green
} else {
    Write-Host "❌ pkg-config non trouvé" -ForegroundColor Red
    Write-Host "   Installer MSYS2 depuis https://www.msys2.org/" -ForegroundColor Yellow
    Write-Host "   Puis: pacman -S mingw-w64-x86_64-pkg-config" -ForegroundColor Yellow
    $script:hasErrors = $true
}

# GTK4
Write-Host "`n[4/5] Vérification de GTK4..." -ForegroundColor Yellow
if (Get-Command pkg-config -ErrorAction SilentlyContinue) {
    try {
        $gtk4Version = pkg-config --modversion gtk4 2>$null
        if ($gtk4Version) {
            Write-Host "✅ GTK4 $gtk4Version installé" -ForegroundColor Green
        } else {
            throw "Not found"
        }
    } catch {
        Write-Host "❌ GTK4 non trouvé" -ForegroundColor Red
        Write-Host "   Dans MSYS2: pacman -S mingw-w64-x86_64-gtk4" -ForegroundColor Yellow
        $script:hasErrors = $true
    }
}

# WebKitGTK
Write-Host "`n[5/5] Vérification de WebKitGTK..." -ForegroundColor Yellow
if (Get-Command pkg-config -ErrorAction SilentlyContinue) {
    try {
        $webkitVersion = pkg-config --modversion webkitgtk-6.0 2>$null
        if ($webkitVersion) {
            Write-Host "✅ WebKitGTK $webkitVersion installé" -ForegroundColor Green
        } else {
            throw "Not found"
        }
    } catch {
        Write-Host "❌ WebKitGTK non trouvé" -ForegroundColor Red
        Write-Host "   Dans MSYS2: pacman -S mingw-w64-x86_64-webkitgtk6" -ForegroundColor Yellow
        $script:hasErrors = $true
    }
}

# PATH verification
Write-Host "`n[6/6] Vérification du PATH..." -ForegroundColor Yellow
$msys2Path = "C:\msys64\mingw64\bin"
if ($env:Path -like "*$msys2Path*") {
    Write-Host "✅ MSYS2 dans le PATH" -ForegroundColor Green
} else {
    Write-Host "⚠️  MSYS2 non détecté dans le PATH" -ForegroundColor Yellow
    Write-Host "   Ajouter au PATH: $msys2Path" -ForegroundColor Yellow
}

Write-Host "`n=== Résumé ===" -ForegroundColor Cyan

if ($script:hasErrors) {
    Write-Host "❌ Configuration incomplète - Suivre les instructions ci-dessus" -ForegroundColor Red
    Write-Host "`nConsulter docs\WINDOWS_SETUP.md pour plus de détails" -ForegroundColor Yellow
    exit 1
} else {
    Write-Host "✅ Environnement prêt pour la compilation!" -ForegroundColor Green
    Write-Host "`nVous pouvez maintenant exécuter:" -ForegroundColor Cyan
    Write-Host "  cargo build" -ForegroundColor White
    Write-Host "  cargo run" -ForegroundColor White
    exit 0
}
