# Install project dependencies
install:
    cargo build --release

# Run the generator with the provided specification file, and optional title and output directory
run specfile_path="example/specs/basic.yaml" output="output" title="":
    set -e
    if [ -z "{{title}}" ]; then \
        cargo run --release -- --specification {{specfile_path}} --output {{output}}; \
    else \
        cargo run --release -- --specification {{specfile_path}} --output {{output}} --title {{title}}; \
    fi
    echo "{{output}}" > .last_output_directory

# Format Rust code
format:
    cargo fmt

# Run tests
test:
    cargo test

# Generate documentation for the generator project
doc:
    cargo doc --open

# Clean up build artifacts
clean:
    cargo clean

# Start the generated microservice
# Uses the path to the last generated output directory with the provided service_name
start-service service_name:
    if [ -f .last_output_directory ]; then \
        output_directory=$(cat .last_output_directory); \
        cd "$output_directory/{{service_name}}" && cargo run; \
    else \
        echo "Error: No last generated output directory found."; \
        exit 1; \
    fi

# Generate documentation for the generated microservice
# Uses the path to the last generated output directory with the provided service_name
service-doc service_name:
    if [ -f .last_output_directory ]; then \
        output_directory=$(cat .last_output_directory); \
        cd "$output_directory/{{service_name}}" && cargo doc --open; \
    else \
        echo "Error: No last generated output directory found."; \
        exit 1; \
    fi
