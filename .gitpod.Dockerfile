FROM gitpod/workspace-full-vnc

RUN sudo apt-get update \
    && sudo apt-get install -yq \
        libgtk-3-dev

RUN cargo install cargo-web
