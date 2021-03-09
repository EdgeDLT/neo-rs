FROM rust:1.40 as builder

# Clone and setup
RUN git clone https://github.com/Liaojinghui/neo-rs.git
WORKDIR neo-rs
COPY . .
RUN cargo install --path .


FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/neo-rs /usr/local/bin/neo-rs
#CMD ["neo-rs"]
CMD /bin/bash