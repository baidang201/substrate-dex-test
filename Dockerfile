# Based from https://github.com/paritytech/substrate/blob/master/.maintain/Dockerfile

FROM phusion/baseimage:bionic-1.0.0 as builder
LABEL maintainer="hello@ubuntutest.network"
LABEL description="This is the build stage for ubuntutest Node. Here we create the binary."

ENV DEBIAN_FRONTEND=noninteractive

ARG PROFILE=release
ARG GIT_COMMIT=
ENV GIT_COMMIT=$GIT_COMMIT

ADD . /var/www/node-template
WORKDIR /var/www/node-template

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	export PATH="$PATH:$HOME/.cargo/bin" && \
	rustup default nightly-2021-03-04 && \
	rustup target add wasm32-unknown-unknown --toolchain nightly-2021-03-04 && \
	cargo build --release

# ===== SECOND STAGE ======

FROM phusion/baseimage:bionic-1.0.0
LABEL maintainer="hello@ubuntutest.network"
LABEL description="This is the 2nd stage: a very small image where we copy the ubuntutest Node binary."
ARG PROFILE=release

RUN mv /usr/share/ca* /tmp && \
	rm -rf /usr/share/*  && \
	mv /tmp/ca-certificates /usr/share/ && \
  mkdir -p /var/www/node-template/.local/share/node-template/ && \
  useradd -m -u 1000 -U -s /bin/sh -d /var/www/node-template ubuntutest && \
  chown -R ubuntutest:ubuntutest /var/www/node-template/.local


COPY --from=builder /var/www/node-template/target/release/node-template /var/www/node-template

# checks
RUN ldd /var/www/node-template/node-template && \
	/var/www/node-template/node-template --version

# Shrinking
RUN rm -rf /usr/lib/python* && \
	rm -rf /usr/bin /usr/sbin /usr/share/man

USER ubuntutest
EXPOSE 30333 9933 9944 9615

CMD /var/www/node-template/node-template --dev --ws-external --rpc-methods=unsafe  --rpc-cors=all