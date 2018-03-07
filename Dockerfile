FROM debian:stretch

RUN apt-get update && apt-get -qy install make gcc g++ bash

ADD . /jml
WORKDIR /jml/src

RUN make

ENTRYPOINT /bin/bash
