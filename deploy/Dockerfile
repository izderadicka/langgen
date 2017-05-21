FROM scorpil/rust:nightly
MAINTAINER ivan.zderadicka@gmail.com

RUN apt-get update &&\
    apt-get install -y wget git

RUN cd /opt &&\
    git clone --depth 1 https://github.com/izderadicka/langgen &&\
    cd langgen &&\
    cargo build --release &&\
    cargo test --release &&\
    wget https://sherlock-holm.es/stories/plain-text/cano.txt

EXPOSE 8080

RUN groupadd -r rust && useradd -r -g rust rust
USER rust

WORKDIR /opt/langgen
CMD target/release/serve -a 0.0.0.0 -p 8080 ./cano.txt



