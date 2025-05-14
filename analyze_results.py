import json
import math
import statistics

# Load the data
with open('query_sizes.json', 'r') as f:
    size_data = json.load(f)

with open('bench-results/res.json', 'r') as f:
    time_data = json.load(f)

# 1. Proof Size Analysis
print("1. PROOF SIZE ANALYSIS")
print("======================")

def format_bytes(size_bytes):
    """Format bytes to a human-readable string"""
    if size_bytes < 1024:
        return f"{size_bytes} B"
    elif size_bytes < 1024**2:
        return f"{size_bytes/1024:.2f} KB"
    elif size_bytes < 1024**3:
        return f"{size_bytes/1024**2:.2f} MB"
    return f"{size_bytes/1024**3:.2f} GB"

# Calculate overhead ratios for each credential count
for cred_count in ["1-credentials", "4-credentials"]:
    print(f"\n{cred_count}:")
    for query_type in ["query", "can_drive", "employment_status"]:
        # Get sizes with and without proof
        with_proof = size_data[cred_count]["query_sizes_with_proof"][query_type]["size_bytes"]
        no_proof = size_data[cred_count]["query_sizes_no_proof"][query_type]["size_bytes"]
        
        # Calculate overhead
        overhead = with_proof / no_proof
        proof_only = with_proof - no_proof
        
        print(f"  {query_type}:")
        print(f"    - With proof: {format_bytes(with_proof)}")
        print(f"    - Without proof: {format_bytes(no_proof)}")
        print(f"    - Proof only: {format_bytes(proof_only)}")
        print(f"    - Overhead factor: {overhead:.2f}x")

# 2. Performance Comparison Across Environments
print("\n\n2. PERFORMANCE COMPARISON ACROSS ENVIRONMENTS")
print("===========================================")

environments = ["macos-ci", "ubuntu-ci", "macos-local"]
implementations = ["minimal", "ed25519-preprocessed"]
operations = ["prove", "verify"]

# Convert underscore to hyphen for query names in res.json
size_to_res_query_map = {
    "query": "query",
    "can_drive": "can-drive",
    "employment_status": "employment-status"
}

# Calculate average times for each environment
for impl in implementations:
    print(f"\n{impl} implementation:")
    
    for operation in operations:
        print(f"  {operation.capitalize()} operation:")
        
        # Calculate averages across environments
        for env in environments:
            queries = ["query", "can-drive", "employment-status"]
            avg_time = statistics.mean([time_data[env][impl][query][operation] for query in queries])
            print(f"    - {env}: {avg_time:.2f} seconds")

# 3. Implementation Overhead Assessment
print("\n\n3. IMPLEMENTATION OVERHEAD ASSESSMENT")
print("====================================")

for env in environments:
    print(f"\n{env}:")
    
    for query in ["query", "can-drive", "employment-status"]:
        minimal_prove = time_data[env]["minimal"][query]["prove"]
        ed25519_prove = time_data[env]["ed25519-preprocessed"][query]["prove"]
        overhead = ed25519_prove / minimal_prove
        
        print(f"  {query}:")
        print(f"    - Minimal prove time: {minimal_prove:.2f} seconds")
        print(f"    - Ed25519 prove time: {ed25519_prove:.2f} seconds")
        print(f"    - Overhead factor: {overhead:.2f}x")

# 4. Proof Generation vs. Verification Time Ratio
print("\n\n4. PROOF GENERATION VS. VERIFICATION TIME RATIO")
print("=============================================")

for env in environments:
    print(f"\n{env}:")
    
    for impl in implementations:
        print(f"  {impl}:")
        
        for query in ["query", "can-drive", "employment-status"]:
            prove_time = time_data[env][impl][query]["prove"]
            verify_time = time_data[env][impl][query]["verify"]
            ratio = prove_time / verify_time
            
            print(f"    {query}:")
            print(f"      - Prove: {prove_time:.2f}s, Verify: {verify_time:.2f}s")
            print(f"      - Ratio (prove/verify): {ratio:.2f}x")

# 5. Query Complexity Impact Assessment
print("\n\n5. QUERY COMPLEXITY IMPACT")
print("==========================")

# Map credential counts to the implementation versions
cred_map = {
    "1-credentials": "minimal",
    "4-credentials": "ed25519-preprocessed"
}

for cred_count, impl in cred_map.items():
    print(f"\n{cred_count} ({impl}):")
    
    # Get the query size data
    size_with_proof = {
        query_type: size_data[cred_count]["query_sizes_with_proof"][query_type]["size_bytes"]
        for query_type in ["query", "can_drive", "employment_status"]
    }
    
    # Get corresponding performance data (using macos-local for consistency)
    env = "macos-local"
    perf_data = {
        size_to_res_query_map[query]: {
            "prove": time_data[env][impl][size_to_res_query_map[query]]["prove"],
            "verify": time_data[env][impl][size_to_res_query_map[query]]["verify"]
        }
        for query in ["query", "can_drive", "employment_status"]
    }
    
    # Show relationships between size and performance
    for query_type in ["query", "can_drive", "employment_status"]:
        res_query = size_to_res_query_map[query_type]
        size = size_with_proof[query_type]
        prove_time = perf_data[res_query]["prove"]
        verify_time = perf_data[res_query]["verify"]
        
        print(f"  {res_query}:")
        print(f"    - Proof size: {format_bytes(size)}")
        print(f"    - Prove time: {prove_time:.2f}s")
        print(f"    - Verify time: {verify_time:.2f}s")
        print(f"    - Prove MB/s: {(size/1024/1024)/prove_time:.2f}")
        print(f"    - Verify MB/s: {(size/1024/1024)/verify_time:.2f}")

# 6. Scaling analysis from 1 to 4 credentials
print("\n\n6. SCALING ANALYSIS: 1 TO 4 CREDENTIALS")
print("=======================================")

for query_type in ["query", "can_drive", "employment_status"]:
    # Size with proof
    size_1cred = size_data["1-credentials"]["query_sizes_with_proof"][query_type]["size_bytes"]
    size_4cred = size_data["4-credentials"]["query_sizes_with_proof"][query_type]["size_bytes"]
    scaling_factor = size_4cred / size_1cred
    
    print(f"\n{query_type}:")
    print(f"  - 1 credential size: {format_bytes(size_1cred)}")
    print(f"  - 4 credentials size: {format_bytes(size_4cred)}")
    print(f"  - Size scaling factor: {scaling_factor:.2f}x")
    
    # Calculate scaling of just the proof portion
    proof_1cred = size_1cred - size_data["1-credentials"]["query_sizes_no_proof"][query_type]["size_bytes"]
    proof_4cred = size_4cred - size_data["4-credentials"]["query_sizes_no_proof"][query_type]["size_bytes"]
    proof_scaling = proof_4cred / proof_1cred
    
    print(f"  - Proof only (1 cred): {format_bytes(proof_1cred)}")
    print(f"  - Proof only (4 cred): {format_bytes(proof_4cred)}")
    print(f"  - Proof scaling factor: {proof_scaling:.2f}x")

print("\nAnalysis complete.") 