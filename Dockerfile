FROM docker.io/hybsearch/docker-base:v1.1

ADD . /jml
WORKDIR /jml/src

ARG cores=4

RUN make -j$cores
RUN mv ./jml /usr/local/bin/jml

ENTRYPOINT /bin/bash
