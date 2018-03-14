FROM docker.io/hybsearch/docker-base:v1.1

ADD . /jml
WORKDIR /jml/src

RUN make

ENTRYPOINT /bin/bash
