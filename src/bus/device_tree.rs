use super::mrom::Mrom;

impl Mrom {
    fn make_dts() -> String {
        "/ {
          #address-cells = <2>;
          #size-cells = <2>;
          compatible = \"ucbbar,spike-bare-dev\";
          model = \"ucbbar,spike-bare\";
          chosen {
            bootargs = \"console=hvc0 earlycon=sbi\";
          };
          cpus {
            #address-cells = <1>;
            #size-cells = <0>;
            timebase-frequency = <10000000>;
            CPU0: cpu@0 {
              device_type = \"cpu\";
              reg = <0>;
              status = \"okay\";
              compatible = \"riscv\";
              riscv,isa = \"rv32imac\";
              mmu-type = \"riscv,sv32\";
              riscv,pmpregions = <16>;
              riscv,pmpgranularity = <4>;
              clock-frequency = <1000000000>;
              CPU0_intc: interrupt-controller {
                #address-cells = <2>;
                #interrupt-cells = <1>;
                interrupt-controller;
                compatible = \"riscv,cpu-intc\";
              };
            };
          };
          memory@80000000 {
            device_type = \"memory\";
            reg = <0x0 0x80000000 0x0 0x80000000>;
          };
          soc {
            #address-cells = <2>;
            #size-cells = <2>;
            compatible = \"ucbbar,spike-bare-soc\", \"simple-bus\";
            ranges;
            clint@2000000 {
              compatible = \"riscv,clint0\";
              interrupts-extended = <&CPU0_intc 3 &CPU0_intc 7 >;
              reg = <0x0 0x2000000 0x0 0xc0000>;
            };
          };
          htif {
            compatible = \"ucb,htif0\";
          };
        };".to_string()
    }

    pub fn load_dtb(_mrom: &Vec<u8>) {
        let dts: String = Self::make_dts();
    }
}

