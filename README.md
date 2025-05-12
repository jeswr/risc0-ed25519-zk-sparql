# useserde::;RISC0 ED25519 ZK SPARQL

A zero knowledge proof library for verifying the correctness of SPARQL queries over verifiable credentials. This library enables confidential querying of credentials signed with ED25519 signatures, allowing the generation of proofs that a query was executed correctly without revealing the underlying data.

## Overview

This library combines the power of zero knowledge proofs using RISC Zero's zkVM with semantic web technologies. It allows you to:

1. Load verifiable credentials with ED25519 signatures
2. Execute SPARQL queries against those credentials
3. Generate zero knowledge proofs that the query was executed correctly
4. Verify those proofs without exposing the underlying credential data

## Setup and Usage

### Prerequisites

- Rust toolchain (installed via rustup)
- RISC Zero toolchain

### Installation

1. Clone the repository

```bash
git clone https://github.com/jeswr/risc0-ed25519-zk-sparql.git
cd risc0-ed25519-zk-sparql
```

2. Set up the test files

```bash
bash ./setup.sh
```

### Running the Project

To generate proofs and verify them:

```bash
# Build the project in release mode
cargo build --release

# Generate a proof
./target/release/host -p ./minimal/ --mode prove --output-file ./sparql_result.json

# Verify the proof
./target/release/host --mode verify --output-file ./sparql_result.json
```

### Development Mode

For faster iteration during development:

```bash
RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run
```

## Example Input

Here's an example of an input credential document (preprocessed JSON-LD):

```json
{
  "verifyData": {
    "proofHash": "169db558030c76a582c72075de05448216fdca6033b13714d6d5ce3f90e1bd73",
    "docHash": "8758be9006372177c7aa63d357d6d4232f533f2a0e8ce1cdb24afebda2f2ac2a",
    "canonicalProof": "_:c14n0 <http://purl.org/dc/terms/created> \"2025-05-12T00:46:08Z\"^^<http://www.w3.org/2001/XMLSchema#dateTime> .\n_:c14n0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2020> .\n_:c14n0 <https://w3id.org/security#proofPurpose> <https://w3id.org/security#assertionMethod> .\n_:c14n0 <https://w3id.org/security#verificationMethod> <did:example:bob#key-1> .\n",
    "canonicalDocument": "<did:example:4e89e3b5-0010-4bb2-b1ea-81bb125b03a9> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/vdl#LicensedDriver> .\n<did:example:4e89e3b5-0010-4bb2-b1ea-81bb125b03a9> <https://w3id.org/vdl#license> _:c14n0 .\n<urn:uuid:9f756655-18ea-4fab-a435-7441d6330478> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/vdl#Iso18013DriversLicenseCredential> .\n<urn:uuid:9f756655-18ea-4fab-a435-7441d6330478> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .\n<urn:uuid:9f756655-18ea-4fab-a435-7441d6330478> <https://schema.org/description> \"A license granting driving privileges in Utopia.\" .\n_:c14n0 <https://w3id.org/vdl#documentNumber> \"542426814\" .\n_:c14n0 <https://w3id.org/vdl#familyName> \"DOE\" .\n_:c14n0 <https://w3id.org/vdl#givenName> \"JOHN\" .\n_:c14n0 <https://w3id.org/vdl#issuingAuthority> \"UADMV\" .\n"
  },
  "verificationMethod": {
    "@context": "https://w3id.org/security/multikey/v1",
    "id": "did:example:bob#key-1",
    "type": "Multikey",
    "controller": "did:example:bob",
    "publicKeyMultibase": "z6MktFdPTf7cVB39HC85tKD2ik26Xn7jfkxkEi9ci8664YfY"
  },
  "proof": {
    "type": "Ed25519Signature2020",
    "created": "2025-05-12T00:46:08Z",
    "verificationMethod": "did:example:bob#key-1",
    "proofPurpose": "assertionMethod",
    "proofValue": "z4FzmUjf7uSpHk4xLVcs5Bzk1MGu6JjEM1EqWNzZjLNwo3sxjfAV6P16vuZ4RMMoBWMFdrFMRsKhXwRaT8JGoZmpj"
  }
}
```

## Example SPARQL Query

```sparql
SELECT ?s ?p ?o WHERE { ?s ?p ?o }
```

Or for more complex queries:

```sparql
PREFIX schema: <https://schema.org/>
PREFIX vdl: <https://w3id.org/vdl#>
PREFIX citizenship: <https://w3id.org/citizenship#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

SELECT DISTINCT ?s ?givenName ?familyName
WHERE {
  ?s citizenship:employmentAuthorizationDocument [ citizenship:lprCategory "C09" ] ;
    citizenship:residentSince ?date ;
    schema:birthDate ?birthDate ;
    schema:givenName ?givenName ;
    schema:familyName ?familyName ;
    vdl:license [ vdl:expiryDate ?expiryDate ] .

  # Has been a resident for at least 5 years
  FILTER( ?date < xsd:dateTime("2020-01-01T00:00:00") )

  # Is over 18
  FILTER( ?birthDate < xsd:dateTime("2007-01-01T00:00:00") )

  # License is valid
  FILTER( ?expiryDate > xsd:dateTime("2025-01-01T00:00:00") )
}
```

## Sample Result

When a query is executed, a zkSPARQL proof is generated. This proof cryptographically verifies that:

1. The credential was correctly signed with the ED25519 signature
2. The SPARQL query was executed correctly against the credential
3. The query results are accurate

Here's an example of the output from running a simple SPARQL query (`SELECT ?s ?p ?o WHERE { ?s ?p ?o }`):

```json
{
  "head": {
    "vars": [
      "s",
      "p",
      "o"
    ]
  },
  "proof": {
    "inner": {
      "Composite": {
        // Cryptographic proof data
      }
    },
    "journal": "351400007b2268656164...",
    "keys": [
      {
        "id": "did:example:bob#key-1",
        "value": "z6MktFdPTf7cVB39HC85tKD2ik26Xn7jfkxkEi9ci8664YfY"
      }
    ],
    "query_string": "SELECT ?s ?p ?o WHERE { ?s ?p ?o }",
    "type": "Risc0ZKVM",
    "zkvm_id": [
      3825859523,
      1526921933,
      2132885766,
      3976020638,
      107559324,
      1976373375,
      401979348,
      3699676838
    ]
  },
  "results": {
    "bindings": [
      {
        "o": {
          "type": "uri",
          "value": "did:example:bob"
        },
        "p": {
          "type": "uri",
          "value": "https://www.w3.org/2018/credentials#issuer"
        },
        "s": {
          "type": "uri",
          "value": "urn:uuid:9f756655-18ea-4fab-a435-7441d6330478"
        }
      },
      {
        "o": {
          "type": "uri",
          "value": "did:example:4e89e3b5-0010-4bb2-b1ea-81bb125b03a9"
        },
        "p": {
          "type": "uri",
          "value": "https://www.w3.org/2018/credentials#credentialSubject"
        },
        "s": {
          "type": "uri",
          "value": "urn:uuid:9f756655-18ea-4fab-a435-7441d6330478"
        }
      },
      {
        "o": {
          "type": "literal",
          "value": "Utopia Driver's License"
        },
        "p": {
          "type": "uri",
          "value": "https://schema.org/name"
        },
        "s": {
          "type": "uri",
          "value": "urn:uuid:9f756655-18ea-4fab-a435-7441d6330478"
        }
      },
      // Additional bindings omitted for brevity
    ]
  }
}
```

The output includes both the query results and a cryptographic proof that can be verified without revealing the original credential data.
