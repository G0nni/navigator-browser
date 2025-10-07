# Solution pour Windows (Sans WebKitGTK)

## Problème

WebKitGTK n'est **pas disponible** dans MSYS2 pour Windows. Seul QtWebKit existe, mais il est incompatible avec GTK4.

## Solutions

### Solution 1 : Tester sans UI (Recommandé pour débuter)

Tester uniquement les couches Domain, Application et Infrastructure sans interface graphique.

#### Installation rapide

```bash
# Dans MSYS2 MINGW64
pacman -S mingw-w64-x86_64-pkg-config mingw-w64-x86_64-sqlite3
```

#### Créer un exemple de test

Créez `examples/test_browser.rs` :

```rust
use navigator::domain::*;
use navigator::application::*;
use navigator::infrastructure::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().init();

    println!("🦀 Navigator - Test sans UI\n");

    // Test Domain Layer
    println!("=== Test Domain Layer ===");
    let tab = Tab::new(false);
    println!("✅ Tab créé: {} - {}", tab.id, tab.title);

    let url = ValidatedUrl::parse("https://example.com")?;
    println!("✅ URL validée: {}", url);

    // Test Security Service
    println!("\n=== Test Security Service ===");
    let security = DefaultSecurityService::new();
    let validated = security.validate_url("example.com")?;
    println!("✅ URL sécurisée: {} (HTTPS: {})", validated, validated.is_secure());

    // Test Database
    println!("\n=== Test Database ===");
    let db = Arc::new(SqliteDatabase::new("test_browser.db").await?);

    let bookmark = Bookmark::new("Example".to_string(), url.clone());
    let id = db.save(&bookmark).await?;
    println!("✅ Bookmark sauvegardé: ID {}", id);

    let bookmarks = db.find_all().await?;
    println!("✅ {} bookmark(s) trouvé(s)", bookmarks.len());

    // Test State Management
    println!("\n=== Test State Management ===");
    let state = BrowserState::new();
    state.add_tab(tab);
    println!("✅ Nombre d'onglets: {}", state.tab_count());

    println!("\n✅ Tous les tests réussis!");
    println!("Le navigateur fonctionne correctement sur Windows (sans UI)");

    Ok(())
}
```

#### Compiler et tester

```bash
# Créer le dossier examples
mkdir examples

# Copier l'exemple ci-dessus dans examples/test_browser.rs

# Compiler et exécuter
cargo run --example test_browser
```

### Solution 2 : Utiliser WSL2 (Windows Subsystem for Linux)

Si vous voulez l'UI complète avec WebKitGTK :

#### Installation WSL2

```powershell
# Dans PowerShell (Admin)
wsl --install
wsl --install -d Ubuntu
```

#### Dans WSL2 Ubuntu

```bash
# Mettre à jour
sudo apt update && sudo apt upgrade -y

# Installer les dépendances
sudo apt install -y build-essential libgtk-4-dev libwebkit2gtk-4.1-dev libsqlite3-dev

# Copier le projet depuis Windows
cp -r /mnt/c/Users/lan057432/Desktop/Test\ Claude/Navigator ~/

# Aller dans le projet
cd ~/Navigator

# Compiler
cargo build --release

# Exécuter (nécessite un serveur X)
cargo run
```

#### Serveur X pour WSL2

Windows 11 : WSLg intégré (rien à faire)

Windows 10 : Installer VcXsrv
1. Télécharger [VcXsrv](https://sourceforge.net/projects/vcxsrv/)
2. Lancer XLaunch avec "Disable access control"
3. Dans WSL2 :
```bash
export DISPLAY=$(cat /etc/resolv.conf | grep nameserver | awk '{print $2}'):0
```

### Solution 3 : Backend Web alternatif (Avancé)

Remplacer WebKitGTK par un backend web moderne :

#### Option A : Tauri (WebView natif)

Utilise le WebView système de Windows (Edge WebView2).

```toml
[dependencies]
tauri = "1.5"
```

#### Option B : Serveur Web local + navigateur système

Le navigateur démarre un serveur web local et ouvre le navigateur système.

#### Option C : Qt WebEngine

Utiliser Qt6 avec QtWebEngine (basé sur Chromium).

## Quelle solution choisir ?

| Solution | Avantages | Inconvénients |
|----------|-----------|---------------|
| **Test sans UI** | ✅ Simple, rapide, teste 70% du code | ❌ Pas d'interface |
| **WSL2** | ✅ UI complète, identique à Linux | ❌ Configuration serveur X |
| **Tauri** | ✅ UI native Windows, moderne | ❌ Réécriture partielle |
| **Qt** | ✅ Cross-platform | ❌ Dépendance Qt volumineuse |

## Recommandation

**Pour apprendre et tester** : Solution 1 (sans UI)
- Vous testez toute l'architecture Clean
- Vous validez la logique métier
- Vous apprenez Rust sans problèmes de GUI

**Pour développer sérieusement** : WSL2 ou développer sur Linux
- UI complète
- Meilleure expérience développement Rust

## Script de test rapide

Créez `test_windows.ps1` :

```powershell
Write-Host "=== Test Navigator sur Windows ===" -ForegroundColor Cyan

# Vérifier Rust
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust non installé" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Rust installé" -ForegroundColor Green

# Test compilation Domain
Write-Host "`nTest compilation Domain Layer..." -ForegroundColor Yellow
cargo test --lib domain --no-fail-fast

# Test compilation Application
Write-Host "`nTest compilation Application Layer..." -ForegroundColor Yellow
cargo test --lib application --no-fail-fast

# Test compilation Infrastructure (sans WebKit)
Write-Host "`nTest compilation Infrastructure Layer..." -ForegroundColor Yellow
cargo test --lib infrastructure --no-fail-fast

Write-Host "`n✅ Tests terminés" -ForegroundColor Green
```

Exécuter :
```powershell
.\test_windows.ps1
```

## Support

Pour plus d'aide :
- GitHub Issues du projet
- Documentation Rust : [rust-lang.org](https://www.rust-lang.org/)
- WSL2 : [docs.microsoft.com/wsl](https://docs.microsoft.com/en-us/windows/wsl/)
