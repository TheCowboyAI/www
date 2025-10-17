#!/usr/bin/env bash
set -e

echo "Building Cowboy AI Presentation for WASM..."

# Install wasm-pack if not available
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM package
echo "Building WASM package..."
wasm-pack build --target web --out-dir pkg

# Create the HTML file
echo "Creating index.html..."
cat > index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Cowboy AI - Presentation</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            overflow: hidden;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
        }
        #loading {
            position: fixed;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            font-size: 24px;
            color: #667eea;
        }
    </style>
</head>
<body>
    <div id="loading">Loading Cowboy AI Presentation...</div>
    <canvas id="iced-canvas"></canvas>
    
    <script type="module">
        import init, { run } from './pkg/cowboy_presentation.js';
        
        async function main() {
            await init();
            
            // Hide loading message
            document.getElementById('loading').style.display = 'none';
            
            // Start the application
            run();
        }
        
        main().catch(console.error);
    </script>
</body>
</html>
EOF

echo "Build complete! Open index.html in a browser to view the presentation."
echo "For development, you can use: python3 -m http.server 8080"