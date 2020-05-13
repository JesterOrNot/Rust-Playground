FROM gitpod/workspace-full-vnc

RUN sudo apt-get update \
    && sudo apt-get install -yq \
        libgtk-3-dev \
        python-dev

RUN bash -cl "cargo install cargo-web"
