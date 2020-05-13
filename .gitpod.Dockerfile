FROM gitpod/workspace-full-vnc

RUN sudo apt-get update \
    && sudo apt-get install -yq \
        libgtk-3-dev \
        python-dev \
        python3-dev

RUN bash -cl "cargo install cargo-web"
