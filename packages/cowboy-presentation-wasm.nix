{ stdenv, lib }:

stdenv.mkDerivation rec {
  pname = "cowboy-presentation-wasm";
  version = "1.0.0";

  src = ../iced-wasm;

  buildPhase = ''
    # WASM files are already built, just copy them
    echo "Preparing WASM presentation files..."
  '';

  installPhase = ''
    mkdir -p $out
    
    # Copy the HTML file
    cp ${src}/index.html $out/
    
    # Copy the WASM package
    cp -r ${src}/pkg $out/wasm-pkg
    
    # Fix the import path in index.html
    sed -i 's|./pkg/cowboy_presentation.js|./wasm-pkg/cowboy_presentation.js|g' $out/index.html
    
    # Ensure proper permissions
    chmod -R 755 $out
  '';

  meta = with lib; {
    description = "Cowboy AI WASM Presentation";
    homepage = "https://thecowboy.ai";
    license = licenses.mit;
    platforms = platforms.all;
  };
}