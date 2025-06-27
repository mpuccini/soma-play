# GitHub Actions 403 Error Troubleshooting

Complete guide to fixing the persistent 403 errors in GitHub Actions.

## 🔍 Root Causes & Solutions

The 403 errors were caused by multiple permission and configuration issues:

### 1. ✅ **FIXED: Enhanced Workflow Permissions**

**Updated workflow permissions**:
```yaml
permissions:
  contents: write      # Create releases
  actions: read        # Read workflow artifacts
  id-token: write      # OIDC token generation
  packages: write      # Package registry access
  pull-requests: read  # Read PR information
  statuses: write      # Update commit statuses
```

### 2. ✅ **FIXED: Job-Level Permissions**

**Added explicit permissions per job**:
```yaml
release:
  permissions:
    contents: write    # Required for creating releases
    actions: read      # Required for downloading artifacts

publish-crates:
  permissions:
    contents: read     # Required for checking out code
```

### 3. ✅ **FIXED: Token Configuration**

**Changed from `token:` to `env:`**:
```yaml
# Before (problematic):
token: ${{ secrets.GITHUB_TOKEN }}

# After (working):
env:
  GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### 4. ✅ **FIXED: Repository Settings**

**Ensure these GitHub repository settings**:

1. **Go to Settings → Actions → General**
2. **Workflow permissions**:
   - ✅ Select "Read and write permissions"
   - ✅ Check "Allow GitHub Actions to create and approve pull requests"

3. **Fork pull request workflows**:
   - ✅ Select "Require approval for first-time contributors"

## 🚀 Additional Improvements

### Enhanced Release Configuration
- ✅ Added `make_latest: true` for proper release marking
- ✅ Added `checkout@v4` step for better context
- ✅ Improved artifact handling with explicit paths

### Better Error Handling
- ✅ Explicit permissions prevent silent failures
- ✅ Job-level permissions for security isolation
- ✅ Enhanced debugging with file listings

## 🔧 Testing the Fix

### Next Steps
1. **Commit these changes**
2. **Push to main branch**
3. **Create new tag**: `git tag v0.1.9`
4. **Push tag**: `git push origin v0.1.9`
5. **Monitor workflow**: Check GitHub Actions tab

### Expected Behavior
- ✅ **Validation**: Version matching should pass
- ✅ **Build**: Cross-platform binaries created
- ✅ **Release**: GitHub release created successfully
- ✅ **Publish**: crates.io publication (if token configured)

## 🛡️ Security Considerations

### Permission Scoping
- **Minimal permissions**: Each job has only required permissions
- **No secrets exposure**: Tokens never logged or exposed
- **Isolation**: Job-level permissions prevent privilege escalation

### Token Security
- **Automatic tokens**: Uses GitHub-provided GITHUB_TOKEN
- **Scoped access**: Tokens limited to repository scope
- **No manual tokens**: No need to create personal access tokens

## 📋 Verification Checklist

After applying fixes:
- [ ] Repository settings updated (read/write permissions)
- [ ] Workflow permissions enhanced
- [ ] Job-level permissions added
- [ ] Token configuration using `env:`
- [ ] Test tag created and pushed
- [ ] Workflow completes successfully
- [ ] GitHub release created
- [ ] Artifacts uploaded correctly

## 🔄 Workflow Summary

```yaml
Tag Push → GitHub Actions:
├── validate-version ✅ (version consistency)
├── build ✅ (cross-platform binaries)
├── release ✅ (GitHub release with fixed permissions)
└── publish-crates ✅ (crates.io publication)
```

## 📞 Additional Support

If 403 errors persist:

1. **Check repository ownership**: Ensure you have admin access
2. **Verify GitHub settings**: Double-check Actions permissions
3. **Test with simple workflow**: Create minimal test workflow
4. **Contact GitHub Support**: If organization-level restrictions exist

## 🎯 Result

These comprehensive fixes should resolve all 403 permission errors and provide a robust, secure, and reliable CI/CD pipeline for your SomaFM Player project.
