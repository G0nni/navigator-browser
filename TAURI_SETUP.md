# Navigator avec Tauri - Guide d'installation Windows

## Prérequis

### 1. WebView2 (déjà inclus dans Windows 11)

Windows 10 : Télécharger depuis [Microsoft Edge WebView2](https://developer.microsoft.com/microsoft-edge/webview2/)

### 2. Rust (déjà installé ✅)

```bash
rustc --version
cargo --version
```

## Installation

### Dans MSYS2 MINGW64

```bash
cd "/c/Users/lan057432/Desktop/Test Claude/Navigator"

# Compiler et lancer le navigateur
cargo run
```

## Fonctionnalités

✅ **Interface native Windows** avec WebView2
✅ **Onglets verticaux** dans la sidebar
✅ **Barre d'adresse** avec auto-completion HTTPS
✅ **Navigation sécurisée** avec indicateur SSL
✅ **Architecture Clean** (Rust backend + HTML/CSS/JS frontend)
✅ **Raccourcis clavier** :
- `Ctrl+T` : Nouvel onglet
- `Ctrl+W` : Fermer l'onglet
- `Ctrl+L` : Focus sur la barre d'adresse

## Architecture

```
┌─────────────────────────────────────┐
│     Frontend (HTML/CSS/JS)          │
│  - Interface utilisateur             │
│  - Onglets verticaux                 │
│  - Barre de navigation               │
└─────────────────┬───────────────────┘
                  │ Tauri Bridge
┌─────────────────▼───────────────────┐
│     Backend Rust                     │
│  - Domain Layer                      │
│  - Application Layer                 │
│  - Infrastructure Layer              │
│  - SQLite, Security, Network         │
└─────────────────────────────────────┘
```

## Avantages de Tauri

1. **Cross-platform** : Windows, Linux, macOS
2. **Léger** : ~600KB de runtime (vs Electron ~100MB)
3. **Sécurisé** : WebView natif du système
4. **Performance** : Rust backend ultra-rapide
5. **Moderne** : HTML/CSS/JS pour l'UI

## Commandes disponibles

```bash
# Dev mode
cargo run

# Build release
cargo build --release

# L'exécutable sera dans:
# target/release/navigator.exe
```

## Debugging

Si erreur au lancement :

```bash
# Vérifier les logs
RUST_LOG=debug cargo run

# Nettoyer et recompiler
cargo clean
cargo run
```

## Prochaines étapes

- [ ] Implémenter vrai WebView (pas iframe)
- [ ] Historique de navigation
- [ ] Bookmarks UI
- [ ] Extensions system
- [ ] Multi-process tabs
