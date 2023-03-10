# Goldilocks
[![Deploy](https://github.com/fanoway/goldilocks/actions/workflows/deploy.yml/badge.svg)](https://github.com/fanoway/goldilocks/actions/workflows/deploy.yml)

## local deployment

### backend
cargo run

http://127.0.0.1:8000/api/v1/locations/all

### frontend

trunk serve --proxy-backend=http://127.0.0.1:8000