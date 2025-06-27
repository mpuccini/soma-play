# Automated crates.io Publishing Setup

This guide shows how to set up automated crates.io publishing via GitHub Actions.

## ✅ What's Already Done

Your GitHub Actions workflow now includes:
- **Automated crates.io publishing** when you push version tags
- **Safety checks**: Only publishes stable releases (not alpha/beta/rc)
- **Full validation**: Builds, tests, and dry-runs before publishing
- **Parallel execution**: Publishes to crates.io alongside GitHub releases

## 🔐 Setting Up the crates.io API Token

### Step 1: Get Your crates.io API Token

1. **Go to crates.io**: https://crates.io/
2. **Sign in** with your GitHub account
3. **Go to Account Settings**: https://crates.io/settings/tokens
4. **Create a new token**:
   - Click "New Token"
   - Name: `soma-player-github-actions` (or similar)
   - **Copy the token** (you'll only see it once!)

### Step 2: Add Token to GitHub Secrets

1. **Go to your GitHub repository**: https://github.com/mpuccini/soma-play
2. **Navigate to Settings**: Click the "Settings" tab
3. **Go to Secrets and Variables**: 
   - Click "Secrets and variables" in the left sidebar
   - Select "Actions"
4. **Add the secret**:
   - Click "New repository secret"
   - **Name**: `CRATES_IO_TOKEN`
   - **Secret**: Paste your crates.io API token
   - Click "Add secret"

## 🚀 How It Works

### Automatic Publishing Flow

1. **You create a version tag**: `git tag v0.1.8 && git push origin v0.1.8`
2. **GitHub Actions triggers** and runs these jobs in parallel:
   - **Build**: Creates binaries for Linux and macOS
   - **Validate**: Ensures tag version matches Cargo.toml
3. **After builds succeed**:
   - **GitHub Release**: Creates release with binaries and SBOM
   - **crates.io Publish**: Publishes to the Rust package registry

### Publishing Conditions

The workflow will **only publish to crates.io** when:
- ✅ Tag starts with `v` (e.g., `v0.1.8`)
- ✅ Tag does NOT contain `alpha`, `beta`, or `rc`
- ✅ Version validation passes
- ✅ All builds succeed
- ✅ Tests pass
- ✅ Dry-run publish succeeds

### Pre-release Handling

Tags like these will **NOT** be published to crates.io:
- `v0.1.8-alpha`
- `v0.1.8-beta.1`
- `v0.1.8-rc.1`

They will still create GitHub releases, but won't go to crates.io.

## 📝 Publishing Process

### For Stable Releases

```bash
# 1. Update version in Cargo.toml
# Edit Cargo.toml: version = "0.1.8"

# 2. Commit changes
git add Cargo.toml
git commit -m "Bump version to 0.1.8"

# 3. Create and push tag
git tag v0.1.8
git push origin main
git push origin v0.1.8

# 4. GitHub Actions handles the rest!
```

### For Pre-releases

```bash
# 1. Update version in Cargo.toml
# Edit Cargo.toml: version = "0.1.8"

# 2. Commit changes
git add Cargo.toml
git commit -m "Bump version to 0.1.8-beta.1"

# 3. Create and push pre-release tag
git tag v0.1.8-beta.1
git push origin main
git push origin v0.1.8-beta.1

# This creates GitHub release but DOES NOT publish to crates.io
```

## 🔍 Monitoring the Process

### Check GitHub Actions

1. Go to your repository
2. Click the "Actions" tab
3. Look for the workflow run triggered by your tag
4. Monitor the "Publish to crates.io" job

### Verify Publication

After successful workflow:
1. **crates.io**: Check https://crates.io/crates/soma-player
2. **GitHub**: Check https://github.com/mpuccini/soma-play/releases

## 🛡️ Security Features

### Token Security
- **Never logged**: Token is never printed in workflow logs
- **Scoped access**: Token only has crates.io publish permissions
- **Encrypted storage**: GitHub secrets are encrypted at rest

### Publication Safety
- **Dry-run first**: Always tests publication before actual publish
- **Version validation**: Ensures versions are consistent
- **Test execution**: Runs full test suite before publishing
- **Manual override**: You can still publish manually if needed

## 🔧 Troubleshooting

### Common Issues

#### "crate already exists"
- **Cause**: Version already published to crates.io
- **Solution**: Increment version number and create new tag

#### "authentication failed"
- **Cause**: Invalid or missing crates.io token
- **Solution**: 
  1. Generate new token on crates.io
  2. Update `CRATES_IO_TOKEN` secret in GitHub

#### "manifest validation failed"
- **Cause**: Missing required metadata in Cargo.toml
- **Solution**: Already fixed in your current Cargo.toml

#### Workflow doesn't trigger
- **Cause**: Tag format incorrect or missing
- **Solution**: Ensure tag starts with `v` (e.g., `v0.1.8`)

### Debug Steps

1. **Check workflow runs**: GitHub Actions tab
2. **Verify token**: Regenerate if uncertain
3. **Test locally**: `cargo publish --dry-run`
4. **Check logs**: Look at failed workflow steps

## 📋 Next Steps

### Immediate Setup
1. ✅ Get crates.io API token
2. ✅ Add `CRATES_IO_TOKEN` to GitHub repository secrets
3. ✅ Test with next version bump

### First Automated Release
```bash
# When ready for your first automated crates.io release:
git tag v0.1.8
git push origin v0.1.8

# Then watch the magic happen in GitHub Actions!
```

## 🎯 Benefits

### For You
- **No manual publishing**: Fully automated process
- **Consistent releases**: Same process every time
- **Safety checks**: Multiple validation steps
- **Parallel distribution**: GitHub + crates.io simultaneously

### For Users
- **Faster availability**: Immediate publication on tag push
- **Multiple install methods**: Both `cargo install` and GitHub releases
- **Reliable releases**: Automated testing and validation

## 📚 Reference

### Workflow Features
- **Version validation**: Ensures consistency
- **Cross-platform builds**: Linux and macOS binaries
- **SBOM generation**: Software Bill of Materials
- **Comprehensive testing**: Full test suite execution
- **Secure publishing**: Token-based authentication

### Manual Override
If you ever need to publish manually:
```bash
cargo login <your-token>
cargo publish
```

Your automated crates.io publishing is now ready! Just add the GitHub secret and start tagging releases. 🚀
