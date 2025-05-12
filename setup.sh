cd data

rm -rf generated
rm -rf .cache
rm -rf queries

npm install
npm run build

# Copy a single file from the ed25519-preprocessed directory
mkdir -p ../minimal/
cp ./generated/ed25519-preprocessed/barcode-bob.jsonld-preprocessed.json ../minimal/barcode-bob.jsonld-preprocessed.json

cd ..
