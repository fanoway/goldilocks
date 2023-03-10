# Goldilocks

## local deployment

### backend
cargo run

http://127.0.0.1:8000/api/v1/locations/all

### frontend

trunk serve --proxy-backend=http://127.0.0.1:8000