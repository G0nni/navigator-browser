# Navigator Architecture Documentation

## Overview

Navigator follows **Clean Architecture** (also known as Hexagonal Architecture or Ports & Adapters) to ensure:
- **Testability**: Easy to mock dependencies and write unit tests
- **Maintainability**: Clear separation of concerns
- **Flexibility**: Easy to swap implementations (e.g., different rendering engines)
- **Security**: Security concerns are isolated and well-defined

## Dependency Rule

The fundamental rule: **dependencies only point inward**

```
Presentation → Application → Domain ← Infrastructure
                    ↓
                  Domain
```

- **Domain** has no dependencies (pure business logic)
- **Application** depends only on Domain
- **Infrastructure** depends on Domain (implements interfaces)
- **Presentation** depends on Application and Infrastructure

## Layer Descriptions

### Domain Layer

**Purpose**: Contains enterprise business rules and entities

**Components**:
- `entities.rs`: Core business objects (Tab, Bookmark, HistoryEntry)
- `value_objects.rs`: Immutable, validated values (TabId, ValidatedUrl)
- `repositories.rs`: Repository traits (interfaces)
- `services.rs`: Service traits for external concerns

**Key Principles**:
- No framework dependencies
- Pure Rust code
- Easily testable
- Framework-agnostic

**Example**:
```rust
// Domain entity - no external dependencies
pub struct Tab {
    pub id: TabId,
    pub title: String,
    pub url: Option<ValidatedUrl>,
    // ...
}

// Domain repository trait
#[async_trait]
pub trait TabRepository: Send + Sync {
    async fn save(&self, tab: &Tab) -> Result<()>;
    async fn find_by_id(&self, id: TabId) -> Result<Option<Tab>>;
}
```

### Application Layer

**Purpose**: Orchestrates use cases and manages application state

**Components**:
- `state.rs`: BrowserState - thread-safe state management
- `use_cases.rs`: Business operations (OpenTab, Navigate, etc.)

**Key Principles**:
- Uses Domain entities and traits
- No knowledge of UI or database details
- Coordinates between Domain and Infrastructure
- Contains application-specific business logic

**Example**:
```rust
pub struct NavigateUseCase {
    state: BrowserState,
    security_service: Arc<dyn SecurityService>,
    history_repository: Arc<dyn HistoryRepository>,
    rendering_engine: Arc<dyn RenderingEngine>,
}

impl NavigateUseCase {
    pub async fn execute(&self, tab_id: TabId, url: &str) -> Result<()> {
        // Validate URL
        let url = self.security_service.validate_url(url)?;

        // Check if blocked
        if self.security_service.is_blocked(&url) {
            return Err(anyhow!("URL blocked"));
        }

        // Navigate
        self.rendering_engine.load_url(&url).await?;

        // Update state and history
        // ...
    }
}
```

### Infrastructure Layer

**Purpose**: Implements interfaces defined in Domain using concrete technologies

**Components**:
- `database.rs`: SQLite implementation of repositories
- `network.rs`: HTTP client using reqwest + rustls
- `rendering.rs`: WebKit rendering engine adapter
- `security.rs`: Security service implementations

**Key Principles**:
- Implements Domain traits
- Contains framework/library specific code
- Can be swapped without affecting Domain or Application

**Example**:
```rust
// Implements Domain trait with concrete technology
#[async_trait]
impl TabRepository for SqliteDatabase {
    async fn save(&self, tab: &Tab) -> Result<()> {
        sqlx::query("INSERT INTO tabs ...")
            .bind(tab.id.to_string())
            // ...
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
```

### Presentation Layer

**Purpose**: User interface and user interaction

**Components**:
- `browser_window.rs`: Main application window
- `vertical_tabs.rs`: Vertical tab sidebar widget
- `url_bar.rs`: Address bar with controls

**Key Principles**:
- Uses Application use cases
- GTK4-specific code
- Minimal business logic
- Delegates to Application layer

**Example**:
```rust
pub struct BrowserWindow {
    window: ApplicationWindow,
    state: BrowserState,
}

impl BrowserWindow {
    pub fn new(app: &Application) -> Result<Self> {
        // Build UI
        let tabs_widget = VerticalTabsWidget::new(state.clone());
        let url_bar = UrlBar::new();

        // Connect to use cases
        url_bar.connect_navigate(move |url| {
            // Delegate to NavigateUseCase
        });

        // ...
    }
}
```

## Data Flow

### Example: User Navigates to URL

```
1. User types URL in UrlBar (Presentation)
   ↓
2. UrlBar emits navigate signal
   ↓
3. BrowserWindow calls NavigateUseCase (Application)
   ↓
4. NavigateUseCase:
   - Validates URL via SecurityService (Infrastructure)
   - Checks if blocked (Domain logic)
   - Loads URL via RenderingEngine (Infrastructure)
   - Updates BrowserState (Application)
   - Saves to HistoryRepository (Infrastructure)
   ↓
5. UI updates to reflect new state (Presentation)
```

## Security Architecture

### Defense in Depth

Multiple layers of security:

1. **Network Layer**: Rustls TLS, HTTPS enforcement
2. **Input Validation**: URL validation, sanitization
3. **Content Security**: CSP, XSS protection
4. **Storage Security**: Encrypted database (planned)
5. **Process Isolation**: Sandboxing (planned)

### Security Components

**SecurityService** (Domain interface):
```rust
pub trait SecurityService {
    fn validate_url(&self, url: &str) -> Result<ValidatedUrl>;
    fn is_blocked(&self, url: &ValidatedUrl) -> bool;
    fn sanitize_html(&self, html: &str) -> String;
}
```

**Implementation** (Infrastructure):
```rust
pub struct DefaultSecurityService {
    blocked_domains: RwLock<HashSet<String>>,
    allow_mixed_content: bool,
}
```

### Threat Model

**Threats Mitigated**:
- ✅ Memory corruption (Rust)
- ✅ XSS attacks (sanitization)
- ✅ Malware/phishing (domain blocking)
- ✅ Man-in-the-middle (TLS)
- ✅ SQL injection (parameterized queries)

**Future Mitigations**:
- 🔲 Process-level tab isolation
- 🔲 Certificate pinning
- 🔲 Content blocker extensions

## State Management

### BrowserState

Thread-safe state container using `Arc<RwLock<T>>`:

```rust
pub struct BrowserState {
    tabs: Arc<RwLock<HashMap<TabId, Tab>>>,
    active_tab: Arc<RwLock<Option<TabId>>>,
    is_private_mode: Arc<RwLock<bool>>,
}
```

**Benefits**:
- Thread-safe sharing between UI and async tasks
- Clone-able (cheap Arc clones)
- Interior mutability with RwLock

**Usage**:
```rust
// Add tab
state.add_tab(tab);

// Get active tab
if let Some(tab) = state.get_active_tab() {
    // Use tab
}
```

## Testing Strategy

### Unit Tests

Each layer can be tested independently:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_creation() {
        let tab = Tab::new(false);
        assert!(!tab.is_private);
        assert_eq!(tab.title, "New Tab");
    }

    #[tokio::test]
    async fn test_open_tab_use_case() {
        let state = BrowserState::new();
        let repo = Arc::new(MockTabRepository::new());
        let use_case = OpenTabUseCase::new(state.clone(), repo);

        let tab_id = use_case.execute(None).await.unwrap();
        assert_eq!(state.tab_count(), 1);
    }
}
```

### Integration Tests

Test cross-layer interactions:

```rust
#[tokio::test]
async fn test_full_navigation_flow() {
    // Create real infrastructure
    let db = SqliteDatabase::new(":memory:").await.unwrap();
    let security = DefaultSecurityService::new();

    // Test complete flow
    // ...
}
```

## Performance Considerations

### Memory Management

- **Tabs**: ~50MB per tab (WebKit)
- **State**: Minimal overhead (HashMap + Arc)
- **Database**: Connection pooling (5 connections)

### Async/Await

- **tokio**: Main async runtime
- **Non-blocking I/O**: Network and database operations
- **UI thread**: GTK main loop (separate from tokio)

### Optimization Strategies

1. **Lazy Loading**: Tabs load content on demand
2. **Caching**: HTTP cache, DNS cache
3. **Connection Pooling**: Reuse database connections
4. **Release Builds**: LTO, codegen-units=1

## Extension Points

### Adding a New Repository

1. Define trait in `domain/repositories.rs`
2. Implement in `infrastructure/database.rs`
3. Use in Application use cases

### Adding a New Use Case

1. Create struct in `application/use_cases.rs`
2. Inject dependencies (repositories, services)
3. Implement `execute()` method
4. Call from Presentation layer

### Swapping Rendering Engine

Replace WebKit with another engine:

1. Implement `RenderingEngine` trait
2. Update `infrastructure/rendering.rs`
3. No changes needed in Domain or Application!

## Best Practices

### Do's
- ✅ Keep Domain pure (no external deps)
- ✅ Use traits for all external concerns
- ✅ Write tests for each layer
- ✅ Use `Result<T>` for error handling
- ✅ Log security events

### Don'ts
- ❌ Put business logic in Presentation
- ❌ Reference Infrastructure from Domain
- ❌ Use `unwrap()` in production code
- ❌ Store secrets in plaintext
- ❌ Skip input validation

## Further Reading

- [Clean Architecture (Robert C. Martin)](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [GTK-rs Documentation](https://gtk-rs.org/)
