FROM rust:latest
WORKDIR /usr/src/app

# 1. Install Python 3 and Virtual Environments
RUN apt-get update && apt-get install -y python3 python3-pip python3-venv

# 2. Set up an isolated Python environment
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

# 3. Install Maturin (The industry standard tool for bridging Rust and Python)
RUN pip install maturin

COPY . .

# 4. Compile the Rust code and inject it directly into the Python environment!
RUN maturin develop --release

# 5. Execute the Python script
ENTRYPOINT ["python", "test_script.py"]