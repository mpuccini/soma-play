#!/bin/bash
# Build and serve documentation locally

set -e

echo "🔧 Building documentation locally..."

# Generate Rust API documentation
echo "📚 Generating API documentation..."
cargo doc --no-deps --document-private-items

# Build mdBook user guide
echo "📖 Building user guide..."
cd docs
mdbook build
cd ..

# Prepare site structure
echo "🌐 Preparing site..."
rm -rf site
mkdir -p site
cp -r target/doc site/api
cp -r docs/book site/guide

# Create index page
cat > site/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Soma Player Documentation</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 2rem; background: #f5f5f5; }
        .container { max-width: 800px; margin: 0 auto; background: white; padding: 2rem; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #333; border-bottom: 2px solid #007acc; padding-bottom: 0.5rem; }
        .docs-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1.5rem; margin-top: 2rem; }
        .doc-card { border: 1px solid #ddd; border-radius: 6px; padding: 1.5rem; background: #fafafa; }
        .doc-card h2 { margin-top: 0; color: #007acc; }
        .doc-card a { color: #007acc; text-decoration: none; font-weight: 500; }
        .doc-card a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎵 Soma Player Documentation</h1>
        <p>Welcome to the Soma Player documentation. Choose the documentation type that best fits your needs:</p>
        <div class="docs-grid">
            <div class="doc-card">
                <h2>📚 API Documentation</h2>
                <p>Comprehensive technical documentation generated from the source code, including all modules, functions, and types.</p>
                <a href="api/">View API Documentation →</a>
            </div>
            <div class="doc-card">
                <h2>📖 User Guide</h2>
                <p>Extended user guide with installation instructions, configuration options, and usage examples.</p>
                <a href="guide/">View User Guide →</a>
            </div>
        </div>
    </div>
</body>
</html>
EOF

echo "✅ Documentation built successfully!"
echo "📁 Files available in ./site/"
echo ""
echo "To serve locally, run:"
echo "  cd site && python3 -m http.server 8000"
echo "  Then open: http://localhost:8000"
