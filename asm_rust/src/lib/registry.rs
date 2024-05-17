use std::{
    cell::Cell,
    fmt::{Debug, Display},
};

#[derive(Debug)]
struct Registry {
    value: Cell<u64>,
    abi_name: &'static str,
    name: &'static str,
    info: &'static str,
}

impl Registry {
    pub const fn new(name: &'static str, abi_name: &'static str, info: &'static str) -> Self {
        Self {
            value: Cell::new(0x0000_0000_0000_0000),
            abi_name,
            name: name,
            info: info,
        }
    }
}

impl Display for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:4} {:5} 0x{:016X}  {:#20}  {}",
            self.name,
            self.abi_name,
            self.value.get(),
            self.value.get(),
            self.info
        )
    }
}

#[test]
fn test_registry() {
    let registry = Registry::new("test", "x1", "test info");
    println!("{}", registry);
    registry.value.set(0xf234_5678_90AB_CDEF);
    println!("{}", registry);
}

pub struct CommonRegistrys {
    registrys: [Registry; 32],
}

impl CommonRegistrys {
    pub fn new() -> Self {
        Self {
            registrys: [
                Registry::new("x0", "zero", "常数0"),
                Registry::new("x1", "ra", "返回地址"),
                Registry::new("x2", "sp", "堆栈指针"),
                Registry::new("x3", "gp", "全局指针"),
                Registry::new("x4", "tp", "线程指针"),
                Registry::new("x5", "t0", "临时寄存器0"),
                Registry::new("x6", "t1", "临时寄存器1"),
                Registry::new("x7", "t2", "临时寄存器2"),
                Registry::new("x8", "s0/fp", "保存寄存器/帧指针"),
                Registry::new("x9", "s1", "保存寄存器1"),
                Registry::new("x10", "a0", "参数0/返回值0"),
                Registry::new("x11", "a1", "参数1/返回值1"),
                Registry::new("x12", "a2", "参数2"),
                Registry::new("x13", "a3", "参数3"),
                Registry::new("x14", "a4", "参数4"),
                Registry::new("x15", "a5", "参数5"),
                Registry::new("x16", "a6", "参数6"),
                Registry::new("x17", "a7", "参数7"),
                Registry::new("x18", "s2", "保存寄存器2"),
                Registry::new("x19", "s3", "保存寄存器3"),
                Registry::new("x20", "s4", "保存寄存器4"),
                Registry::new("x21", "s5", "保存寄存器5"),
                Registry::new("x22", "s6", "保存寄存器6"),
                Registry::new("x23", "s7", "保存寄存器7"),
                Registry::new("x24", "s8", "保存寄存器8"),
                Registry::new("x25", "s9", "保存寄存器9"),
                Registry::new("x26", "s10", "保存寄存器10"),
                Registry::new("x27", "s11", "保存寄存器11"),
                Registry::new("x28", "t3", "临时寄存器3"),
                Registry::new("x29", "t4", "临时寄存器4"),
                Registry::new("x30", "t5", "临时寄存器5"),
                Registry::new("x31", "t6", "临时寄存器6"),
            ],
        }
    }
}
impl Display for CommonRegistrys {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CommonRegistrys:\n")?;
        write!(f, "--------------\n")?;
        for registry in self.registrys.iter() {
            write!(f, "{}\n", registry)?;
        }
        write!(f, "--------------\n")
    }
}

#[test]
fn test_registrys() {
    let registrys = CommonRegistrys::new();
    println!("{}", registrys);
}

struct FloatRegistrys {
    registrys: [Registry; 32],
}

impl FloatRegistrys {
    fn new() -> Self {
        Self {
            registrys: [
                Registry::new("f0", "ft0", "浮点临时寄存器0"),
                Registry::new("f1", "ft1", "浮点临时寄存器1"),
                Registry::new("f2", "ft2", "浮点临时寄存器2"),
                Registry::new("f3", "ft3", "浮点临时寄存器3"),
                Registry::new("f4", "ft4", "浮点临时寄存器4"),
                Registry::new("f5", "ft5", "浮点临时寄存器5"),
                Registry::new("f6", "ft6", "浮点临时寄存器6"),
                Registry::new("f7", "ft7", "浮点临时寄存器7"),
                Registry::new("f8", "fs0", "浮点保存寄存器0"),
                Registry::new("f9", "fs1", "浮点保存寄存器1"),
                Registry::new("f10", "fa0", "浮点参数0"),
                Registry::new("f11", "fa1", "浮点参数1"),
                Registry::new("f12", "fa2", "浮点参数2"),
                Registry::new("f13", "fa3", "浮点参数3"),
                Registry::new("f14", "fa4", "浮点参数4"),
                Registry::new("f15", "fa5", "浮点参数5"),
                Registry::new("f16", "fa6", "浮点参数6"),
                Registry::new("f17", "fa7", "浮点参数7"),
                Registry::new("f18", "fs2", "浮点保存寄存器2"),
                Registry::new("f19", "fs3", "浮点保存寄存器3"),
                Registry::new("f20", "fs4", "浮点保存寄存器4"),
                Registry::new("f21", "fs5", "浮点保存寄存器5"),
                Registry::new("f22", "fs6", "浮点保存寄存器6"),
                Registry::new("f23", "fs7", "浮点保存寄存器7"),
                Registry::new("f24", "fs8", "浮点保存寄存器8"),
                Registry::new("f25", "fs9", "浮点保存寄存器9"),
                Registry::new("f26", "fs10", "浮点保存寄存器10"),
                Registry::new("f27", "fs11", "浮点保存寄存器11"),
                Registry::new("f28", "ft8", "浮点临时寄存器8"),
                Registry::new("f29", "ft9", "浮点临时寄存器9"),
                Registry::new("f30", "ft10", "浮点临时寄存器10"),
                Registry::new("f31", "ft11", "浮点临时寄存器11"),
            ],
        }
    }
}

impl Display for FloatRegistrys {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FloatRegistrys:\n")?;
        write!(f, "--------------\n")?;
        for registry in self.registrys.iter() {
            write!(f, "{}\n", registry)?;
        }
        write!(f, "--------------\n")
    }
}
