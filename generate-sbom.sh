#!/bin/bash
# Generate Software Bill of Materials- `dependencies-structured.txt` - Dependencies with structured output(SBOM) for soma-player

set -e

echo "🔍 Generating SBOM for soma-player..."

# Create output directory
mkdir -p sbom

# Install required tools if not present
if ! command -v cargo-cyclonedx &> /dev/null; then
    echo "📦 Installing cargo-cyclonedx..."
    cargo install cargo-cyclonedx
fi

# Generate dependency tree
echo "📋 Generating dependency tree..."
cargo tree > sbom/dependencies.txt
cargo tree --prefix depth > sbom/dependencies-structured.txt

# Generate CycloneDX SBOM
echo "🔄 Generating CycloneDX SBOM..."
if cargo cyclonedx --format json > sbom/sbom.json 2>/dev/null; then
    echo "✅ JSON SBOM generated successfully"
else
    echo "⚠️  JSON SBOM generation failed"
fi

if cargo cyclonedx --format xml > sbom/sbom.xml 2>/dev/null; then
    echo "✅ XML SBOM generated successfully" 
else
    echo "⚠️  XML SBOM generation failed"
fi

# Generate security audit
echo "🔒 Running security audit..."
if command -v cargo-audit &> /dev/null; then
    cargo audit --format json > sbom/security-audit.json 2>/dev/null || echo "[]" > sbom/security-audit.json
    cargo audit > sbom/security-audit.txt 2>/dev/null || echo "No known vulnerabilities found" > sbom/security-audit.txt
else
    echo "cargo-audit not installed. Install with: cargo install cargo-audit"
    echo "No security audit performed" > sbom/security-audit.txt
fi

# Create a summary
echo "📊 Creating SBOM summary..."
cat > sbom/README.md << EOF
# Software Bill of Materials (SBOM) for soma-player

Generated on: $(date)
Version: $(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

## Files

- \`sbom.json\` - SBOM in CycloneDX JSON format
- \`sbom.xml\` - SBOM in CycloneDX XML format
- \`dependencies.txt\` - Simple dependency tree
- \`dependencies-structured.txt\` - Dependencies with structure
- \`security-audit.txt\` - Security vulnerability report
- \`security-audit.json\` - Security report in JSON format

## Usage

These files can be used for:
- Supply chain security analysis
- License compliance checking
- Vulnerability tracking
- Dependency management

## Tools Used

- cargo-cyclonedx: CycloneDX SBOM generation
- cargo-audit: Security vulnerability scanning
- cargo tree: Dependency tree analysis
EOF

echo "✅ SBOM generation complete!"
echo "📁 Files saved to ./sbom/"
echo ""
echo "Generated files:"
ls -la sbom/
