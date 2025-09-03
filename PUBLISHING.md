# Publishing to Cargo.io Checklist

Before publishing cfgloader_rs to crates.io, make sure to complete the following steps:

## Prerequisites

1. **Update Repository URLs**: Replace `YOUR_USERNAME` in all Cargo.toml files with your actual GitHub username
   - `/cfgloader/Cargo.toml`
   - `/core/Cargo.toml` 
   - `/macros/Cargo.toml`

2. **Update Author Information**: Replace placeholder author info in all Cargo.toml files
   - Replace `"Your Name <your.email@example.com>"` with your actual name and email

3. **Create GitHub Repository**: 
   - Create a public repository at `https://github.com/YOUR_USERNAME/cfgloader_rs`
   - Push this code to the repository

## Pre-Publication Checks

### 1. Test Everything
```bash
# Test workspace compilation
cargo check --workspace

# Test main example
cd example && APP_NAME=TestApp cargo run

# Run tests (if any)
cargo test --workspace
```

### 2. Documentation
```bash
# Generate and check documentation
cargo doc --workspace --no-deps --open

# Check that README examples work
# (manually verify the examples in README.md)
```

### 3. Package Validation
```bash
# Check that packages can be built for publication
cargo package --manifest-path core/Cargo.toml
cargo package --manifest-path macros/Cargo.toml  
cargo package --manifest-path cfgloader/Cargo.toml
```

## Publishing Order

**Important**: Publish in this exact order due to dependencies:

### 1. Publish cfgloader-core first
```bash
cd core
cargo publish
```

### 2. Publish cfgloader_rs_macros second  
```bash
cd macros
cargo publish
```

### 3. Publish main cfgloader_rs crate last
```bash
cd cfgloader
cargo publish
```

## Post-Publication

1. **Tag the release**:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

2. **Update documentation**: The docs should automatically appear on docs.rs

3. **Announce**: Consider announcing on relevant Rust forums/communities

## Notes

- All crates use version `0.1.0` and should be published together
- The main crate `cfgloader` depends on the other two, so they must be published first
- Make sure your crates.io account has publish permissions
- Consider setting up CI/CD for future releases

## Useful Commands

```bash
# Login to crates.io (if not already)
cargo login

# Check current login status
cargo search cfgloader_rs

# Dry run publish (recommended first)
cargo publish --dry-run
```
