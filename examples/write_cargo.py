import os
import re

template = '''[package]
name = "example-{name}"
version = "0.0.0"
authors = ["Stellar Development Foundation <info@stellar.org>"]
license = "Apache-2.0"
edition = "2021"
publish = false

[lib]
crate-type = ["cdylib"]
doctest = false

[dependencies]
loam-sdk = {{ workspace = true, features = ["loam-soroban-sdk"] }}
loam-subcontract-core = {{ workspace = true }}

[dev_dependencies]
loam-sdk = {{ workspace = true, features = ["soroban-sdk-testutils"] }}
'''

def generate_cargo_toml(directory):
    # Convert directory name to package name format
    name = directory.replace('_', '-')
    
    # Create the content for Cargo.toml
    content = template.format(name=name)
    
    # Write the content to Cargo.toml in the directory
    with open(os.path.join('.', 'soroban', directory, 'Cargo.toml'), 'w') as f:
        f.write(content)

def main():
    soroban_dir = os.path.join('.','soroban')
    
    # Ensure the soroban directory exists
    if not os.path.exists(soroban_dir):
        print(f"The directory {soroban_dir} does not exist.")
        return

    # Iterate over all directories in examples/soroban
    for directory in os.listdir(soroban_dir):
        full_path = os.path.join(soroban_dir, directory)
        if os.path.isdir(full_path):
            generate_cargo_toml(directory)
            print(f"Generated Cargo.toml for {directory}")

if __name__ == "__main__":
    main()
