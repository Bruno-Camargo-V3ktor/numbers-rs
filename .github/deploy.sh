#!/bin/bash

echo "🚧 Compilando..."
dx build --release

cd ./target/dx/numbers/release/web/public


git init
git checkout -b gh-pages
git add -A
git commit -m "deploy: atualizando site"

echo "🚀 Enviando para o GitHub..."
git push -f https://github.com/Bruno-Camargo-V3ktor/numbers-rs.git gh-pages

cd -
echo "✅ Deploy concluído!"