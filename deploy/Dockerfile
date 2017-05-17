FROM scorpil/rust:nightly
MAINTAINER ivan.zderadicka@gmail.com

ENV http_proxy http://emea-proxy.uk.oracle.com:80
ENV https_proxy http://emea-proxy.uk.oracle.com:80

RUN apt-get update &&\
    apt-get install -y wget git

RUN cd /opt &&\
    git clone --depth 1 https://github.com/izderadicka/langgen &&\
    cargo build --release &&\
    cargo test --release &&\
    wget https://sherlock-holm.es/stories/plain-text/cano.txt

WORKDIR /opt/langgen
CMD cargo run --release --bin serve -- -a 0.0.0.0 -p 80 ./cano.txt



