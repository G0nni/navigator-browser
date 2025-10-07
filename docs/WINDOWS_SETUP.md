# Installation sur Windows

## Prérequis

### 1. Installer Rust

Téléchargez et installez Rust depuis [rustup.rs](https://rustup.rs/):

```powershell
# Télécharger et exécuter rustup-init.exe
# Suivre les instructions à l'écran
# Redémarrer le terminal après installation
```

Vérifier l'installation :
```powershell
rustc --version
cargo --version
```

### 2. Installer MSYS2

**MSYS2** fournit les bibliothèques GTK4 et WebKitGTK nécessaires.

1. Télécharger MSYS2 depuis [msys2.org](https://www.msys2.org/)
2. Exécuter l'installeur (installer dans `C:\msys64`)
3. Ouvrir **MSYS2 MINGW64** depuis le menu Démarrer

### 3. Installer les dépendances GTK4 et WebKitGTK

Dans le terminal **MSYS2 MINGW64** :

```bash
# Mettre à jour MSYS2
pacman -Syu

# Fermer et rouvrir MSYS2 MINGW64, puis :
pacman -Su

# Installer les outils de compilation
pacman -S --needed base-devel mingw-w64-x86_64-toolchain

# Installer GTK4
pacman -S mingw-w64-x86_64-gtk4

# Installer WebKitGTK
# Note: Vérifier la version disponible avec: pacman -Ss webkit
# Les noms possibles: webkitgtk, webkit2gtk
pacman -S mingw-w64-x86_64-webkitgtk

# Si l'erreur "target not found" apparaît, essayer:
# pacman -S mingw-w64-x86_64-webkit2gtk

# Installer pkg-config (nécessaire pour la compilation)
pacman -S mingw-w64-x86_64-pkg-config

# Installer SQLite
pacman -S mingw-w64-x86_64-sqlite3
```

### 4. Configurer les variables d'environnement

Ajouter MSYS2 au PATH Windows :

**Via PowerShell (Admin)** :
```powershell
[Environment]::SetEnvironmentVariable(
    "Path",
    $env:Path + ";C:\msys64\mingw64\bin",
    [EnvironmentVariableTarget]::User
)
```

**Ou manuellement** :
1. Rechercher "Variables d'environnement" dans Windows
2. Cliquer sur "Variables d'environnement..."
3. Dans "Variables utilisateur", éditer `Path`
4. Ajouter : `C:\msys64\mingw64\bin`
5. OK pour valider

**Redémarrer votre terminal/IDE après cette étape**

## Compilation du projet

### Option A : Via PowerShell/CMD (après config PATH)

```powershell
# Naviguer vers le projet
cd "C:\Users\lan057432\Desktop\Test Claude\Navigator"

# Compiler en mode debug (plus rapide)
cargo build

# Ou compiler en mode release (optimisé)
cargo build --release

# Exécuter
cargo run
```

### Option B : Via MSYS2 MINGW64

```bash
# Naviguer vers le projet
cd "/c/Users/lan057432/Desktop/Test Claude/Navigator"

# Compiler
cargo build --release

# Exécuter
cargo run
```

## Problèmes courants et solutions

### Erreur : "pkg-config not found"

**Solution** :
```bash
# Dans MSYS2 MINGW64
pacman -S mingw-w64-x86_64-pkg-config
```

Ajouter au PATH : `C:\msys64\mingw64\bin`

### Erreur : "cannot find -lgtk-4"

**Solution** : GTK4 n'est pas installé ou pas dans le PATH
```bash
pacman -S mingw-w64-x86_64-gtk4
```

### Erreur : "webkit2gtk-6.0 not found"

**Solution** : WebKitGTK n'est pas installé
```bash
pacman -S mingw-w64-x86_64-webkitgtk6
```

### Erreur de compilation avec sqlx

**Solution** : Créer un fichier `.cargo/config.toml` pour utiliser SQLite embarqué

```bash
# Dans MSYS2 MINGW64
pacman -S mingw-w64-x86_64-sqlite3
```

### Le programme ne démarre pas : DLL manquantes

**Solution** : Copier les DLLs ou ajouter MSYS2 au PATH système

Les DLLs nécessaires se trouvent dans `C:\msys64\mingw64\bin\`

### Performance : Compilation très lente

**Solutions** :
1. Utiliser `cargo build` (debug) plutôt que `--release` en développement
2. Ajouter dans `Cargo.toml` :
```toml
[profile.dev]
opt-level = 1  # Optimisation légère en debug
```

## Alternative : WSL2 (Windows Subsystem for Linux)

Si MSYS2 pose problème, vous pouvez utiliser WSL2 :

```powershell
# Installer WSL2
wsl --install

# Installer Ubuntu
wsl --install -d Ubuntu

# Dans WSL2 Ubuntu
sudo apt update
sudo apt install build-essential libgtk-4-dev libwebkit2gtk-4.1-dev libsqlite3-dev

# Cloner le projet
cd ~
cp -r "/mnt/c/Users/lan057432/Desktop/Test Claude/Navigator" .

# Compiler
cd Navigator
cargo build --release
cargo run
```

**Note** : WSL2 nécessite un serveur X (comme VcXsrv ou WSLg intégré dans Windows 11)

## Test rapide sans UI complète

Si vous voulez juste tester la compilation sans l'UI :

1. Créer un fichier `examples/test_domain.rs` :

```rust
use navigator::domain::*;

fn main() {
    let tab = Tab::new(false);
    println!("Tab created: {:?}", tab);

    let url = ValidatedUrl::parse("https://example.com").unwrap();
    println!("URL validated: {}", url);

    println!("✅ Domain layer works!");
}
```

2. Compiler l'exemple :
```powershell
cargo run --example test_domain
```

## Configuration Visual Studio Code

Si vous utilisez VS Code, installer :

1. **rust-analyzer** : Extension Rust officielle
2. **CodeLLDB** : Debugger pour Rust

Fichier `.vscode/settings.json` :
```json
{
    "rust-analyzer.check.command": "clippy",
    "terminal.integrated.env.windows": {
        "PATH": "C:\\msys64\\mingw64\\bin;${env:PATH}"
    }
}
```

## Vérification de l'installation

Script de test complet :

```powershell
# test_setup.ps1

Write-Host "=== Vérification de l'environnement Navigator ==="

# Rust
Write-Host "`n[1/5] Vérification de Rust..."
if (Get-Command rustc -ErrorAction SilentlyContinue) {
    rustc --version
    Write-Host "✅ Rust installé" -ForegroundColor Green
} else {
    Write-Host "❌ Rust non trouvé" -ForegroundColor Red
}

# Cargo
Write-Host "`n[2/5] Vérification de Cargo..."
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    cargo --version
    Write-Host "✅ Cargo installé" -ForegroundColor Green
} else {
    Write-Host "❌ Cargo non trouvé" -ForegroundColor Red
}

# pkg-config
Write-Host "`n[3/5] Vérification de pkg-config..."
if (Get-Command pkg-config -ErrorAction SilentlyContinue) {
    pkg-config --version
    Write-Host "✅ pkg-config installé" -ForegroundColor Green
} else {
    Write-Host "❌ pkg-config non trouvé - Installer via MSYS2" -ForegroundColor Red
}

# GTK4
Write-Host "`n[4/5] Vérification de GTK4..."
if (Get-Command pkg-config -ErrorAction SilentlyContinue) {
    $gtk4 = pkg-config --modversion gtk4 2>$null
    if ($gtk4) {
        Write-Host "✅ GTK4 $gtk4 installé" -ForegroundColor Green
    } else {
        Write-Host "❌ GTK4 non trouvé" -ForegroundColor Red
    }
}

# WebKitGTK
Write-Host "`n[5/5] Vérification de WebKitGTK..."
if (Get-Command pkg-config -ErrorAction SilentlyContinue) {
    $webkit = pkg-config --modversion webkitgtk-6.0 2>$null
    if ($webkit) {
        Write-Host "✅ WebKitGTK $webkit installé" -ForegroundColor Green
    } else {
        Write-Host "❌ WebKitGTK non trouvé" -ForegroundColor Red
    }
}

Write-Host "`n=== Fin de la vérification ==="
```

Exécuter :
```powershell
.\test_setup.ps1
```

## Support

Si vous rencontrez des problèmes :

1. Vérifier les logs : `cargo build -vv` (mode verbose)
2. Consulter les issues GitHub du projet
3. Vérifier que MSYS2 est à jour : `pacman -Syu`
