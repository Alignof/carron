FROM ghcr.io/alignof/riscv-tests-docker:master
ENTRYPOINT ["sh", "/data/riscv-tests.sh"]

ENV HOME /root
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH $PATH:/opt/riscv32/bin:$HOME/.cargo/bin
WORKDIR /data
