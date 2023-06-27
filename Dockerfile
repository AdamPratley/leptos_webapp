FROM rust:1.67

RUN git clone https://github.com/AdamPratley/leptos_webapp.git
WORKDIR /leptos_webapp

RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked cargo-leptos
RUN cargo leptos build --release



CMD ["cargo", "leptos", "serve", "--release"]