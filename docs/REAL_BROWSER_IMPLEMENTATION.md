# Implémentation d'un VRAI navigateur (sans iframes)

## Problème actuel

Les iframes ont des limitations :
- ❌ Beaucoup de sites bloquent les iframes (X-Frame-Options)
- ❌ CORS restrictions
- ❌ Pas de contrôle total sur le rendu
- ❌ Pas d'accès aux DevTools
- ❌ Performances limitées

## Solutions pour un vrai navigateur

### Solution 1 : Multi-Window Tauri (Le plus simple pour vous)

**Principe** : Chaque onglet = une fenêtre Tauri séparée avec sa propre WebView

**Avantages** :
✅ WebView native Windows (Edge/Chromium)
✅ Aucune restriction iframe
✅ Navigation complète
✅ DevTools intégrés
✅ Performances natives

**Implémentation** :

```rust
#[tauri::command]
async fn open_tab_window(
    app: tauri::AppHandle,
    url: String,
) -> Result<String, String> {
    let window_label = format!("tab-{}", uuid::Uuid::new_v4());

    tauri::WindowBuilder::new(
        &app,
        &window_label,
        tauri::WindowUrl::External(url.parse().unwrap())
    )
    .title("Navigator Tab")
    .inner_size(1200.0, 800.0)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(window_label)
}
```

**Architecture** :
```
┌─────────────────────────┐
│  Fenêtre principale     │
│  (Sidebar + Controls)   │
└─────────────────────────┘
         │
    ┌────┴────────────────┐
    │                     │
┌───▼────┐  ┌────▼────┐  ┌────▼────┐
│ Tab 1  │  │  Tab 2  │  │  Tab 3  │
│WebView │  │ WebView │  │ WebView │
└────────┘  └─────────┘  └─────────┘
```

### Solution 2 : Webview2 Direct (Plus de contrôle)

**Principe** : Utiliser directement l'API WebView2 de Windows

**Avantages** :
✅ Contrôle total sur le WebView
✅ Peut héberger plusieurs WebViews dans une fenêtre
✅ API complète (cookies, cache, etc.)

**Inconvénients** :
❌ Plus complexe
❌ Code Windows uniquement
❌ Besoin d'interfacer avec C++/COM

**Implémentation** (via crate `webview2-com`) :

```rust
use webview2_com::*;

// Créer un WebView2 controller
let controller = WebView2Controller::create(hwnd, env)?;
controller.navigate(url)?;
```

### Solution 3 : Servo (Moteur Rust pur)

**Principe** : Moteur de rendu web en pur Rust (par Mozilla)

**Avantages** :
✅ 100% Rust
✅ Contrôle complet
✅ Personnalisable à l'infini

**Inconvénients** :
❌ Encore expérimental
❌ Pas tous les standards web supportés
❌ Très complexe à intégrer

### Solution 4 : CEF (Chromium Embedded Framework)

**Principe** : Embarquer Chromium complet

**Avantages** :
✅ Chromium complet
✅ 100% compatible web

**Inconvénients** :
❌ Très lourd (~150MB)
❌ Complexe à compiler
❌ Bindings Rust compliqués

## 🎯 Recommandation pour VOUS

### Implémentation Multi-Window Tauri

**Pourquoi ?**
- ✅ Rapide à implémenter (1-2h)
- ✅ Fonctionne sur Windows immédiatement
- ✅ Utilise Edge/Chromium natif (déjà installé)
- ✅ Navigation complète sans restrictions
- ✅ Garde votre architecture Clean intacte

**Architecture proposée** :

```
src/
├── main.rs              # Fenêtre principale (sidebar + controls)
├── tab_window.rs        # Gestion des fenêtres d'onglets
├── domain/              # Votre logique (inchangée)
├── application/         # Use cases (inchangés)
└── infrastructure/      # Adapters (inchangés)

ui/
├── index.html           # UI principale (sidebar)
└── tab.html            # Template pour chaque onglet (optionnel)
```

**Communication entre fenêtres** :
- Fenêtre principale → Onglets : Events Tauri
- Onglets → Fenêtre principale : IPC Tauri
- État partagé : Rust backend (votre BrowserState)

## Implémentation étape par étape

### Étape 1 : Ajouter commande pour créer des fenêtres

```rust
#[tauri::command]
async fn create_tab_window(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    url: String,
) -> Result<String, String> {
    // Valider URL
    let validated_url = state.security.validate_url(&url)
        .map_err(|e| format!("Invalid URL: {}", e))?;

    // Créer fenêtre
    let window_id = uuid::Uuid::new_v4().to_string();

    let window = tauri::WindowBuilder::new(
        &app,
        &window_id,
        tauri::WindowUrl::External(
            validated_url.as_str().parse().unwrap()
        )
    )
    .title(&format!("Navigator - {}", validated_url))
    .inner_size(1200.0, 800.0)
    .decorations(false)  // Personnaliser les contrôles
    .build()
    .map_err(|e| e.to_string())?;

    // Sauvegarder l'onglet
    let tab = Tab::with_url(validated_url, false);
    let tab_id = state.browser_state.lock().unwrap().add_tab(tab);

    Ok(window_id)
}
```

### Étape 2 : Gérer les événements de navigation

```rust
// Dans la fenêtre de l'onglet, écouter les changements d'URL
window.on_navigation(|event| {
    // Notifier la fenêtre principale
    app.emit_all("tab-navigated", {
        window_id: window.label(),
        url: event.url(),
    }).ok();
});
```

### Étape 3 : Synchroniser avec la sidebar

```javascript
// Dans ui/app.js
window.__TAURI__.event.listen('tab-navigated', (event) => {
    const { window_id, url } = event.payload;
    // Mettre à jour l'onglet dans la sidebar
    updateTabUrl(window_id, url);
});
```

## Alternative : WebView Container

Si vous voulez garder tout dans une fenêtre mais avec de vrais WebViews :

```rust
use webview2_com::*;

struct TabContainer {
    tabs: HashMap<TabId, WebView2Controller>,
    active_tab: Option<TabId>,
}

impl TabContainer {
    fn switch_tab(&mut self, tab_id: TabId) {
        // Cacher tous les webviews
        for controller in self.tabs.values() {
            controller.set_visible(false);
        }
        // Afficher le webview actif
        if let Some(controller) = self.tabs.get(&tab_id) {
            controller.set_visible(true);
        }
    }
}
```

## Prochaines étapes

1. **Test Multi-Window** : Commencer avec l'approche fenêtres séparées
2. **Si besoin de tabs intégrés** : Passer à webview2-com
3. **Si besoin de contrôle ultime** : Considérer Servo (long terme)

Quelle approche voulez-vous essayer ?
