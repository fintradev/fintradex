FROM docker.io/paritytech/ci-unified:latest as builder

WORKDIR /fintra
COPY . /fintra

RUN cargo fetch
RUN cargo build --locked --release

FROM docker.io/parity/base-bin:latest

COPY --from=builder /fintra/target/release/fintra-node /usr/local/bin

USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /fintra fintra && \
	mkdir -p /data /fintra/.local/share && \
	chown -R fintra:fintra /data && \
	ln -s /data /fintra/.local/share/fintra && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/fintra-node --version

USER fintra

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/fintra-node"]
