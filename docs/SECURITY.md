# Security Documentation

## Security Philosophy

Navigator is built with **security-first** principles:

1. **Secure by Default**: All features default to the most secure option
2. **Defense in Depth**: Multiple layers of protection
3. **Fail Secure**: Errors result in secure behavior
4. **Least Privilege**: Minimal permissions for all components
5. **Memory Safety**: Leveraging Rust's ownership system

## Threat Model

### Assets to Protect
- User's browsing data (history, bookmarks, cookies)
- User's credentials and personal information
- System resources and files
- Network privacy

### Threat Actors
- **Malicious websites**: XSS, drive-by downloads, phishing
- **Network attackers**: MITM, eavesdropping
- **Malware**: Exploiting browser vulnerabilities
- **Trackers**: Privacy invasion

### Attack Vectors
1. Network (MITM, DNS poisoning)
2. Web content (XSS, CSRF, malicious scripts)
3. Memory vulnerabilities (buffer overflows, use-after-free)
4. Social engineering (phishing, malicious URLs)

## Security Features

### 1. Memory Safety

**Rust's Ownership System**:
- No buffer overflows
- No use-after-free bugs
- No data races in concurrent code
- No null pointer dereferences

**Example**:
```rust
// This won't compile - ownership prevents use-after-free
let tab = Tab::new(false);
drop(tab);
// tab.title // ERROR: value borrowed after move
```

### 2. Network Security

#### TLS/SSL
- **Implementation**: rustls (memory-safe TLS in Rust)
- **Minimum Version**: TLS 1.2 (recommended: TLS 1.3)
- **Certificate Validation**: Strict validation using webpki-roots
- **No OpenSSL**: Avoiding C-based crypto libraries

```rust
let client = Client::builder()
    .use_rustls_tls()
    .build()?;
```

#### HTTPS-First
- URLs default to HTTPS when no scheme specified
- Automatic upgrade from HTTP to HTTPS
- Warning on insecure connections

```rust
fn validate_url(&self, url: &str) -> Result<ValidatedUrl> {
    let url_with_scheme = if !url.contains("://") {
        format!("https://{}", url)  // Default to HTTPS
    } else {
        url.to_string()
    };
    // ...
}
```

#### DNS-over-HTTPS (DoH)
- Encrypted DNS queries
- Prevents DNS hijacking and surveillance
- Default provider: Cloudflare DoH

### 3. Content Security

#### Content Security Policy (CSP)
```rust
fn default_csp() -> String {
    CspBuilder::new()
        .default_src(&["'self'"])
        .script_src(&["'self'", "'unsafe-inline'"])
        .style_src(&["'self'", "'unsafe-inline'"])
        .img_src(&["'self'", "data:", "https:"])
        .upgrade_insecure_requests()
        .build()
}
```

#### XSS Prevention
```rust
fn sanitize_html(&self, html: &str) -> String {
    html.replace("<script", "&lt;script")
        .replace("javascript:", "")
        .replace("onerror=", "")
        .replace("onclick=", "")
        .replace("onload=", "")
}
```

#### URL Validation
```rust
pub struct ValidatedUrl {
    url: url::Url,
}

impl ValidatedUrl {
    pub fn parse(input: &str) -> Result<Self, url::ParseError> {
        let url = url::Url::parse(input)?;
        // Validation logic
        Ok(Self { url })
    }

    pub fn is_secure(&self) -> bool {
        self.url.scheme() == "https"
    }
}
```

### 4. Data Protection

#### Private Browsing
- No history recording
- No bookmark saving
- Session-only cookies
- No disk writes

```rust
pub struct Tab {
    pub is_private: bool,
    // ...
}

// Use case respects privacy
if !tab.is_private {
    self.history_repository.add(&entry).await?;
}
```

#### Database Security
- **Parameterized Queries**: Prevents SQL injection
- **Encrypted at Rest**: (Planned) SQLCipher integration
- **Access Control**: File permissions on database

```rust
// Safe: parameterized query
sqlx::query("INSERT INTO history (url, title) VALUES (?, ?)")
    .bind(url)
    .bind(title)
    .execute(&pool)
    .await?;

// NEVER: string concatenation
// query(&format!("INSERT ... VALUES ('{}')", user_input)) ❌
```

### 5. Process Isolation

#### Current State
- Single process architecture
- Tab state isolation via ownership

#### Planned
- Multi-process architecture
- One process per tab
- Sandboxed renderer processes
- Privileged browser process

```
┌─────────────────────────────────────┐
│       Browser Process               │
│    (Privileged, UI, Storage)        │
└─────────────────────────────────────┘
         │            │          │
    ┌────┴────┐  ┌───┴───┐  ┌──┴────┐
    │ Tab 1   │  │ Tab 2 │  │ Tab 3 │
    │Renderer │  │Renderer│ │Renderer│
    │(Sandbox)│  │(Sandbox)│ │(Sandbox)│
    └─────────┘  └────────┘  └───────┘
```

### 6. Permission Management

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Permission {
    Camera,
    Microphone,
    Location,
    Notifications,
    Storage,
}

pub struct SecurityContext {
    pub permissions: Vec<Permission>,
}
```

**Default**: All permissions denied
**Request Flow**: User prompt → Explicit grant → Per-session

### 7. Content Blocking

#### Malware/Phishing Protection
```rust
pub struct DefaultSecurityService {
    blocked_domains: RwLock<HashSet<String>>,
}

impl DefaultSecurityService {
    fn is_blocked(&self, url: &ValidatedUrl) -> bool {
        if let Some(host) = url.host_str() {
            if let Ok(blocked) = self.blocked_domains.read() {
                return blocked.contains(host);
            }
        }
        false
    }
}
```

#### Tracker/Ad Blocking (Planned)
- EasyList integration
- Privacy Badger-style learning
- Per-site controls

## Security Best Practices for Developers

### Code Review Checklist

- [ ] All user input is validated
- [ ] No `unwrap()` on external data
- [ ] Parameterized database queries
- [ ] HTTPS for all external requests
- [ ] No secrets in code or logs
- [ ] Error messages don't leak sensitive info
- [ ] File operations use proper permissions

### Input Validation

```rust
// ✅ Good: Validate before use
fn handle_url(url_str: &str) -> Result<()> {
    let url = ValidatedUrl::parse(url_str)?;
    navigate(url)
}

// ❌ Bad: Assume input is safe
fn handle_url(url_str: &str) {
    navigate(url_str) // No validation!
}
```

### Error Handling

```rust
// ✅ Good: Handle errors securely
match validate_certificate(&url).await {
    Ok(cert) => proceed(cert),
    Err(e) => {
        tracing::warn!("Certificate validation failed");
        show_security_warning()
    }
}

// ❌ Bad: Expose details, ignore errors
match validate_certificate(&url).await {
    Ok(cert) => proceed(cert),
    Err(e) => {
        println!("Cert error: {}", e); // Logs sensitive info
        proceed_anyway() // Ignores security error!
    }
}
```

### Logging

```rust
// ✅ Good: Safe logging
tracing::info!("User navigated to domain: {}", url.host_str());

// ❌ Bad: Logs full URL (may contain tokens/credentials)
tracing::info!("User navigated to: {}", url.as_str());
```

## Security Testing

### Automated Testing

```bash
# Security audit for known vulnerabilities
cargo audit

# Lint for common mistakes
cargo clippy

# Memory sanitizer (nightly)
RUSTFLAGS="-Z sanitizer=address" cargo +nightly test
```

### Manual Testing

1. **XSS Tests**:
   - `<script>alert('xss')</script>`
   - `javascript:alert('xss')`
   - Event handler injection

2. **SQL Injection Tests**:
   - `' OR '1'='1`
   - `'; DROP TABLE history; --`

3. **Path Traversal**:
   - `../../../etc/passwd`
   - `..\\..\\..\\windows\\system32`

4. **Certificate Validation**:
   - Self-signed certificates
   - Expired certificates
   - Wrong hostname

### Fuzzing

```bash
# Install cargo-fuzz
cargo install cargo-fuzz

# Fuzz URL parser
cargo fuzz run fuzz_url_parser

# Fuzz HTML sanitizer
cargo fuzz run fuzz_html_sanitizer
```

## Incident Response

### Vulnerability Disclosure

**Contact**: security@navigator-browser.org (create this!)

**Process**:
1. Report received → Acknowledge within 24h
2. Investigate → Confirm severity
3. Fix developed → Security patch
4. Coordinated disclosure → 90 days or when fixed
5. Public advisory → CVE assigned

### Security Updates

- **Critical**: Released immediately
- **High**: Within 7 days
- **Medium**: Next regular release
- **Low**: Next minor version

## Compliance & Standards

### Standards Followed
- OWASP Top 10 (Web Application Security)
- CWE Top 25 (Common Weakness Enumeration)
- NIST Cybersecurity Framework

### Certifications (Planned)
- CVE Numbering Authority (CNA)
- OpenSSF Best Practices Badge

## Privacy

### Data Collection
**Navigator collects NO telemetry by default**

Optional diagnostic data (opt-in):
- Crash reports (anonymized)
- Performance metrics (aggregated)

### Data Storage
- Bookmarks: Local SQLite database
- History: Local SQLite database
- Cookies: WebKit cookie store (local)
- Passwords: OS keychain integration (planned)

### Data Sharing
**Navigator shares NO data with third parties**

Network requests only to:
- User-requested websites
- DoH provider (for DNS)
- Update server (for security updates)

## Security Roadmap

### Short-term (3 months)
- [ ] Process-per-tab isolation
- [ ] Certificate pinning
- [ ] Encrypted database storage
- [ ] Password manager integration

### Medium-term (6 months)
- [ ] Extension system with permissions
- [ ] Advanced content blocker
- [ ] Fingerprinting protection
- [ ] Security audit by third party

### Long-term (12 months)
- [ ] Built-in VPN support
- [ ] Tor integration option
- [ ] WebAuthn/FIDO2 support
- [ ] Bug bounty program

## References

- [OWASP Web Security](https://owasp.org/www-project-web-security-testing-guide/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [Browser Security Handbook](https://code.google.com/archive/p/browsersec/wikis/Main.wiki)
- [WebKit Security](https://webkit.org/security/)

## Contact

Security issues: security@navigator-browser.org
General questions: GitHub Issues
