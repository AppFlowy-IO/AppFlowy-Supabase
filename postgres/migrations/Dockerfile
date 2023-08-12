FROM postgres:latest

RUN apt-get update && \
    apt-get install -y build-essential postgresql-server-dev-all git && \
    git clone https://github.com/michelp/pgjwt.git && \
    cd pgjwt && \
    make && \
    make install

RUN localedef -i en_US -c -f UTF-8 -A /usr/share/locale/locale.alias en_US.UTF-8
ENV LANG en_US.UTF-8
ENV LC_ALL en_US.UTF-8
