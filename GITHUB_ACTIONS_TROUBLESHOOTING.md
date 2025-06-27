# GitHub Actions Troubleshooting Guide

This guide addresses common issues with the GitHub Actions workflows.

## 🚨 Current Issues & Fixes

### ✅ FIXED: SBOM Generation Error

**Problem**: `cargo-cyclonedx` command syntax error
```
error: unexpected argument '--output-file' found
```

**Root Cause**: The newer version of `cargo-cyclonedx` (v0.5.7+) changed its command-line interface.

**Fix Applied**: Updated both workflow and script to use correct syntax:
```bash
# ❌ Old syntax (doesn't work)
cargo cyclonedx --format json --output-file sbom.json

# ✅ New syntax (working)
cargo cyclonedx --format json > sbom.json
```

### 🔧 GitHub Release 403 Errors

**Problem**: GitHub release creation fails with 403 permission errors.

**Possible Solutions**:

#### 1. Repository Settings Check
Go to your repository settings and verify:
- **Settings** → **Actions** → **General**
- Under "Workflow permissions":
  - ✅ Select "Read and write permissions"
  - ✅ Check "Allow GitHub Actions to create and approve pull requests"

#### 2. Personal Access Token (Alternative)
If repository permissions don't work, create a personal access token:

1. **Generate Token**:
   - Go to: https://github.com/settings/tokens
   - Click "Generate new token (classic)"
   - Scopes needed: `repo`, `write:packages`

2. **Add to Repository Secrets**:
   - Repository → Settings → Secrets and variables → Actions
   - Add secret: `GH_TOKEN` = your personal access token

3. **Update Workflow**:
   ```yaml
   env:
     GITHUB_TOKEN: ${{ secrets.GH_TOKEN }}
   ```

#### 3. Repository Ownership
Ensure you have admin/write access to the repository. If this is a fork, the original repository owner may need to enable Actions.

### 🎯 Testing the Fixes

#### Test SBOM Generation Locally
```bash
# Install the tool
cargo install cargo-cyclonedx

# Test the new syntax
cargo cyclonedx --format json > test-sbom.json
cargo cyclonedx --format xml > test-sbom.xml

# Or use the script
./generate-sbom.sh
```

#### Test GitHub Actions
1. **Create a new tag**: `git tag v0.1.9 && git push origin v0.1.9`
2. **Monitor the workflow**: Go to Actions tab in GitHub
3. **Check each job**: Build, Release, Publish

## 📋 Complete Workflow Status

### Current Workflow Jobs

1. **✅ validate-version** - Ensures tag matches Cargo.toml
2. **✅ build** - Cross-platform builds (Linux + macOS)  
3. **🔧 release** - GitHub release creation (fixing 403 issues)
4. **⏳ publish-crates** - Automated crates.io publishing (pending token setup)

### Prerequisites Checklist

- [ ] Repository permissions set to "Read and write"
- [ ] SBOM generation syntax updated (✅ Done)
- [ ] crates.io API token added to GitHub secrets
- [ ] Version bumped in Cargo.toml
- [ ] Changes committed and tagged

## 🛠️ Quick Fixes

### If GitHub Release Still Fails
```bash
# Option 1: Check repository permissions (recommended)
# Go to Settings → Actions → General → Workflow permissions

# Option 2: Manual release creation
gh release create v0.1.8 --generate-notes

# Option 3: Use personal access token
# Add GH_TOKEN secret with personal access token
```

### If crates.io Publishing Fails
```bash
# Common issues:
# 1. Missing CRATES_IO_TOKEN secret
# 2. Version already published  
# 3. Package validation errors

# Test locally first:
cargo publish --dry-run
```

### If SBOM Generation Fails
```bash
# Update cargo-cyclonedx to latest version
cargo install cargo-cyclonedx --force

# Test new syntax
cargo cyclonedx --format json > sbom.json
```

## 📞 Getting Help

### Check Workflow Logs
1. Go to repository → Actions tab
2. Click on the failed workflow run
3. Expand the failed job to see detailed logs

### Common Log Locations
- **Build logs**: Build job → "Build binary" step
- **SBOM logs**: Build job → "Generate SBOM" step  
- **Release logs**: Release job → "Create Release" step
- **Publish logs**: Publish-crates job → "Publish to crates.io" step

### Useful Commands
```bash
# Check local build
cargo build --release

# Test package creation
cargo package --list

# Verify version consistency
grep version Cargo.toml
git tag --list | tail -5

# Test SBOM generation
./generate-sbom.sh
```

## 🎯 Next Steps

1. **Fix repository permissions** (most likely cause of 403 errors)
2. **Test with a new tag** (e.g., v0.1.9)
3. **Add crates.io token** for automated publishing
4. **Monitor workflow execution** in GitHub Actions

The SBOM generation issue has been fixed. The GitHub release 403 error is likely a permissions issue that can be resolved in repository settings.
