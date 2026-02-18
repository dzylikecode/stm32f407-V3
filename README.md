[stm32f407探索者开发板V3](http://www.openedv.com/docs/boards/stm32/zdyz_stm32f407_explorerV3.html)

推荐入门：[教程](https://logiase.github.io/The-Embedded-Rust-Book-CN/intro/index.html) | [原文](https://docs.rust-embedded.org/book/index.html)

## 硬件

- Cortex-M3 架构

## 工具

- OpenOCD： 调试翻译器，连接在 MCU 与 GDB 之间
- gdb-multiarch： MCU 的调试器
- QEMU： 模拟器

```bash
sudo apt install gdb-multiarch openocd qemu-system-arm
```

可以先检查权限，不用安装 udev 规则

```bash
lsusb
```

添加 MCU 工具链

```bash
rustup target add thumbv7em-none-eabihf #Cortex-M3
```


## config

### 配置

[简短](https://haobogu.github.io/posts/keyboard/rust-embedded-tutorial/)

[模板](https://github.com/knurling-rs/app-template)


[embassy](https://www.yuque.com/haobogu/vgcc41/eh2ke8okhav1twgs) | [官方](https://embassy.dev/book/#_getting_started)

```bash
cargo init .  --bin --name=config
```

1. 安装 cortex-m, cortex-m-rt

    ```bash
    cargo add cortex-m --features critical-section-single-core,inline-asm
    ```

    - critical-section-single-core: 只支持单核 MCU 的临界区实现
    - inline-asm: 支持内联汇编
    
    ```bash
    cargo add cortex-m-rt
    ```

2. 不用教程里面传统的同步框架， 采用现代化的异步框架 [embassy](https://github.com/embassy-rs/embassy)，必须指定 features

    ```bash
    cargo add embassy-stm32 --features stm32f407zg,time-driver-any,exti,defmt,memory-x
    ```
    - stm32f407zg: 指定芯片型号
    - time-driver-any: 使用 embassy-time 作为异步定时器驱动
    - exti: 使能外部中断支持
    - defmt: 支持 defmt 日志格式
    - 如果开启了embassy-stm32的memory-x feature，那么就需要移除memory.x相关的配置，使用embassy内置的。

    embassy-executor 需要你在 Cargo.toml 里开启至少一个 feature（比如 arch-cortex-m，再选一种执行器实现如 executor-thread 或 executor-interrupt）。否则它不知道该如何在你的 MCU 上跑 async 任务。

    ```bash
    cargo add embassy-executor --features arch-cortex-m,executor-thread,defmt
    ```


3. panic 处理，否则不知道如何处理异常，这里采用特征： 通过 RTT + defmt 输出错误信息与堆栈

    ```bash
    cargo add panic-probe --features print-defmt
    ```

4. 日志库

    ```bash
    cargo add defmt defmt-rtt
    ```

    - defmt-rtt 是 嵌入式设备的超轻量日志输出系统，使用 RTT（Real-Time Transfer） 通道，把 MCU 内部的日志通过调试器（如 ST-Link / J-Link）实时传到主机显示。

5. 时间处理

    embassy-time 是 Embassy 的异步定时器与时间抽象库

    ```bash
    cargo add embassy-time  --features defmt,defmt-timestamp-uptime,tick-hz-32_768
    ```

    - defmt: 支持 defmt 日志格式
    - defmt-timestamp-uptime: defmt 日志的时间戳自动来自系统运行时间（Uptime）
    - tick-hz-32_768: 设置 tick 频率为 32768Hz（RTC 常用低功耗时钟）提升定时精度

### 编译

编译可以命令行执行

```bash
cargo build --target thumbv7em-none-eabihf
```

或者写入到 `.cargo/config.toml`， 然后执行 cargo build 与上面等价

```toml
[build]
target = "thumbv7em-none-eabihf"
```

会生成 `target/thumbv7em-none-eabihf/debug/<projectName>` 二进制文件

### 烧录与调试

安装烧录工具 [probe-rs](https://probe.rs/)

```bash
cargo install probe-rs-tools --locked
```

查看烧录器

```bash
probe-rs list
```


配置 `.cargo/config.toml`

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# replace STM32F429ZITx with your chip as listed in `probe-rs chip list`
runner = "probe-rs run --chip stm32f407zg"
```

配置连接，写在 build.rs 里面

```rust
fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
```

> 如果开启了embassy-stm32的memory-x feature，那么就需要移除memory.x相关的配置，使用embassy内置的。

### 烧录 hex 文件

针对 keil 生成的 hex 文件

```bash
probe-rs download --binary-format hex test.hex  --chip STM32F407ZG
```

### issue

- 烧录的时候记得硬件复位
- ide 提示 `#![no_std]` can't find crate for test

   - vscode 配置

        ```json
        {
            "rust-analyzer.check.allTargets": false,
            "rust-analyzer.check.targets": "x86_64-unknown-linux-gnu"
        }
        ```
    
    - 或者在 `Cargo.toml` 里面排除 test

        ```toml
        [[bin]]
        name = "config"
        test = false
        bench = false
        ```

## tutorial

运行文件都放到 src/bin 目录下, 采用 `t_num_name.rs` 命名方式

如果配置了

```toml
[[bin]]
name = "blink"
path = "src/bin/t_00_blink.rs"
test = false
```

则可以直接运行

```bash
cargo run --bin blink
```
 
如果是用了 lib.rs 这样的入口文件，则一定要声明 `#![no_std]`

```rust
// lib.rs
#![no_std]
```

### blink

推挽输出在 Output 空间下面

```rust
let mut led0 = Output::new(p.PF9, Level::High, Speed::High);
led0.set_low();
led0.set_high();
```

### led

引入 embassy_hal_internal 这个库用来作封装

### beep

推挽输出

### key

输入

```rust
let key = Input::new(p.PA0, Pull::Up);
key.is_high();
key.is_low();
```
### 外部中断

| EXTI线    | 中断函数             |
| --------- | -------------------- |
| EXTI0     | EXTI0_IRQHandler     |
| EXTI1     | EXTI1_IRQHandler     |
| EXTI2     | EXTI2_IRQHandler     |
| EXTI3     | EXTI3_IRQHandler     |
| EXTI4     | EXTI4_IRQHandler     |
| EXTI5–9   | EXTI9_5_IRQHandler   |
| EXTI10–15 | EXTI15_10_IRQHandler |

### DMA

需要自己查芯片
