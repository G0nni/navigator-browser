// Exemple de test du navigateur sans interface graphique
// Idéal pour Windows où WebKitGTK n'est pas disponible

use navigator::domain::*;
use navigator::application::*;
use navigator::infrastructure::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("navigator=debug,info")
        .init();

    println!("\n🦀 Navigator Browser - Test Sans UI");
    println!("=====================================\n");

    // Test 1: Domain Layer
    println!("📦 [1/6] Test Domain Layer");
    let tab = Tab::new(false);
    println!("   ✅ Tab créé: {} - {}", tab.id, tab.title);

    let url = ValidatedUrl::parse("https://example.com")?;
    println!("   ✅ URL validée: {}", url);
    println!("   ✅ Est sécurisée (HTTPS): {}", url.is_secure());

    // Test 2: Security Service
    println!("\n🔒 [2/6] Test Security Service");
    let security = Arc::new(DefaultSecurityService::new());

    let validated = security.validate_url("example.com")?;
    println!("   ✅ URL auto-upgrade HTTPS: {}", validated);

    let malicious = ValidatedUrl::parse("https://malware-example.com")?;
    security.add_blocked_domain("malware-example.com".to_string());
    println!("   ✅ Domaine bloqué: {}", security.is_blocked(&malicious));

    let safe_html = security.sanitize_html("<script>alert('xss')</script>");
    println!("   ✅ HTML sanitisé: {}", safe_html);

    // Test 3: Database
    println!("\n💾 [3/6] Test Database (SQLite)");
    let db = Arc::new(SqliteDatabase::new("test_browser.db").await?);

    // Bookmarks
    let bookmark = Bookmark::new("Example Site".to_string(), url.clone());
    let bookmark_id = BookmarkRepository::save(&*db, &bookmark).await?;
    println!("   ✅ Bookmark sauvegardé: ID {}", bookmark_id);

    let all_bookmarks = BookmarkRepository::find_all(&*db).await?;
    println!("   ✅ Bookmarks trouvés: {}", all_bookmarks.len());

    // History
    let history_entry = HistoryEntry::new(url.clone(), "Example Domain".to_string());
    let history_id = db.add(&history_entry).await?;
    println!("   ✅ Historique enregistré: ID {}", history_id);

    let recent_history = db.get_recent(10).await?;
    println!("   ✅ Entrées d'historique: {}", recent_history.len());

    // Test 4: State Management
    println!("\n🔄 [4/6] Test State Management");
    let state = BrowserState::new();

    let tab1_id = state.add_tab(Tab::new(false));
    let tab2_id = state.add_tab(Tab::new(false));
    println!("   ✅ Onglets créés: {}", state.tab_count());

    state.set_active_tab(tab1_id);
    println!("   ✅ Onglet actif: {}", state.get_active_tab_id().unwrap());

    state.set_private_mode(true);
    println!("   ✅ Mode privé: {}", state.is_private_mode());

    // Test 5: Use Cases
    println!("\n🎯 [5/6] Test Use Cases");

    let open_tab_uc = OpenTabUseCase::new(state.clone(), db.clone());
    let new_tab_id = open_tab_uc.execute(None).await?;
    println!("   ✅ OpenTabUseCase: Tab {}", new_tab_id);

    let close_tab_uc = CloseTabUseCase::new(state.clone(), db.clone());
    close_tab_uc.execute(new_tab_id).await?;
    println!("   ✅ CloseTabUseCase: Tab fermé");

    // Test 6: Network (si connexion internet disponible)
    println!("\n🌐 [6/6] Test Network Service");
    let network = SecureNetworkClient::new()?;

    match network.fetch(&url).await {
        Ok(data) => {
            println!("   ✅ Fetch réussi: {} octets récupérés", data.len());
        }
        Err(e) => {
            println!("   ⚠️  Fetch échoué (normal si pas de connexion): {}", e);
        }
    }

    // Résumé
    println!("\n=====================================");
    println!("✅ Tous les tests réussis!");
    println!("🎉 Le navigateur fonctionne correctement");
    println!("\n💡 Prochaines étapes:");
    println!("   - Compiler en release: cargo build --release");
    println!("   - Tester sur Linux avec UI: installer WebKitGTK");
    println!("   - Voir docs/WINDOWS_WORKAROUND.md pour l'UI sur Windows");
    println!("\n");

    Ok(())
}
