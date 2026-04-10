# Security Policy

## Sensitive Data Handling

BloatwareHatao is an Android bloatware removal tool that interacts with Android devices via ADB. This document outlines security best practices for contributors and users.

### ADB Access and Device Interaction

BloatwareHatao requires ADB (Android Debug Bridge) access to function:

- **ADB Authorization**: Devices must have USB debugging enabled and authorize the computer
- **Device Access**: The tool interacts with the Android package manager to list and remove packages
- **Root Access**: Some features may require root access for system-level package removal

**Always ensure**:

- You authorize only trusted computers for ADB access
- Disable USB debugging when not in use
- Understand the implications of removing system packages
- Create backups before making changes to your device

### What's Excluded from Git

The following sensitive data types are automatically excluded via `.gitignore`:

1. **Credentials & Keys**
   - Private keys (*.pem,*.key, id_rsa, etc.)
   - Certificates (*.crt,*.cer, *.p12,*.pfx)
   - Keystores (*.jks,*.keystore)
   - Environment files (.env, .env.*)
   - API key files (secrets.json, credentials.json)

2. **Android Artifacts**
   - APK files (*.apk)
   - Device backups and dumps
   - ADB log files with sensitive information

3. **Build & Runtime Artifacts**
   - Rust build artifacts (target/)
   - Configuration files with user data

### Security Scanning

This repository uses:

- **cargo-deny**: Dependency vulnerability scanner
- **clippy**: Rust linter with security-focused checks
- **typos**: Spell checker to prevent typosquatting vulnerabilities

Run security checks before committing:

```bash
cargo deny check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Reporting Security Issues

If you discover a security vulnerability in BloatwareHatao, please report it privately:

- Do not open a public GitHub issue
- Contact the maintainers directly through GitHub Security Advisories
- Provide detailed information about the vulnerability

### Data Privacy

BloatwareHatao is designed to remove unwanted applications from Android devices. Users must:

- Understand which packages they are removing and their dependencies
- Create backups before making system-level changes
- Be aware that removing system packages may affect device functionality
- Follow manufacturer guidelines for device modifications

### Safe Usage Guidelines

- **Research Packages**: Understand what a package does before removing it
- **Use Presets Cautiously**: Presets are community-curated; review them before use
- **Backup First**: Always create a device backup before bulk removal
- **Test Incrementally**: Remove packages in small batches to identify issues
- **Device Compatibility**: Ensure your device model is supported before proceeding

## License

BloatwareHatao is licensed under GPL-3.0. See LICENSE file for details.
