FROM archlinux
ENTRYPOINT ["sh", "/data/rv32_tests.sh"]

RUN pacman -Syyu --noconfirm autoconf automake curl python3 libmpc mpfr git gmp gawk base-devel bison flex texinfo gperf libtool patchutils bc zlib expat
RUN git clone https://github.com/riscv/riscv-gnu-toolchain \
    && cd /riscv-gnu-toolchain \
    && ./configure --prefix=/opt/riscv32 --with-arch=rv32gc --with-abi=ilp32d && make -j8
ENV HOME /root
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH $PATH:/opt/riscv32/bin:$HOME/.cargo/bin
RUN git clone https://github.com/riscv/riscv-tests \
    && cd /riscv-tests \
    && git checkout e30978a71921159aec38eeefd848fca4ed39a826 \
    && git submodule update --init --recursive \
    && autoconf \
    && ./configure --prefix=/opt/riscv32/ --with-xlen=32 \
    && make -j8 \
    && make -j8 install 
WORKDIR /data
