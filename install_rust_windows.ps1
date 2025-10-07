# Script d'installation automatique de Rust pour Windows

Write-Host "=== Installation de Rust pour Navigator ===" -ForegroundColor Cyan

# Vérifier si Rust est déjà installé
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "✅ Rust est déjà installé!" -ForegroundColor Green
    cargo --version
    rustc --version
    exit 0
}

Write-Host "`n📥 Téléchargement de rustup-init..." -ForegroundColor Yellow

# Télécharger rustup-init
$rustupUrl = "https://win.rustup.rs/x86_64"
$rustupPath = "$env:TEMP\rustup-init.exe"

try {
    Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath
    Write-Host "✅ Téléchargement réussi" -ForegroundColor Green
} catch {
    Write-Host "❌ Erreur de téléchargement: $_" -ForegroundColor Red
    Write-Host "`nVeuillez télécharger manuellement depuis: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

Write-Host "`n🔧 Lancement de l'installation..." -ForegroundColor Yellow
Write-Host "Suivez les instructions (appuyez sur Entrée pour installation par défaut)" -ForegroundColor Yellow

# Lancer l'installeur
Start-Process -FilePath $rustupPath -Wait -NoNewWindow

Write-Host "`n✅ Installation terminée!" -ForegroundColor Green
Write-Host "`n⚠️  IMPORTANT: Redémarrez votre terminal PowerShell/CMD" -ForegroundColor Yellow
Write-Host "Puis exécutez:" -ForegroundColor Cyan
Write-Host "  cargo --version" -ForegroundColor White

# Nettoyer
Remove-Item $rustupPath -ErrorAction SilentlyContinue
