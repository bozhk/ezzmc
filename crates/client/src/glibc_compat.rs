//! Совместимость со старым glibc: в glibc 2.43/2.44 часть функций libm получила новые версии
//! символов (`atan2f@GLIBC_2.43`, `sinh@GLIBC_2.44` …), из-за чего бинарник, собранный на свежем
//! Arch, не стартует на системах со старым glibc. Определяем эти функции в самом exe
//! и перенаправляем на старые версии (`@GLIBC_2.2.5`), которые есть во всех glibc.
#![cfg(all(target_os = "linux", target_env = "gnu", target_arch = "x86_64"))]

macro_rules! compat {
    ($($name:literal),*) => {
        core::arch::global_asm!(
            $(
                concat!(".symver ", $name, "_glibc_old, ", $name, "@GLIBC_2.2.5\n"),
                concat!(".globl ", $name, "\n"),
                concat!(".type ", $name, ", @function\n"),
                concat!($name, ":\n"),
                concat!("jmp ", $name, "_glibc_old@PLT\n"),
            )*
        );
    };
}

compat!("atan2f", "acoshf", "asinf", "acosf", "sinhf", "coshf", "sinh", "cosh");
