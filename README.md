做清华大学操作系统rCore的实验。

实验教程地址：https://rcore-os.github.io/rCore-Tutorial-Book-v3/index.html

原实验构建环境为ubuntu，提供了模拟器和真机等多种运行环境的模式。

笔者这里构建环境使用win10，运行环境为qemu，并且为了方便，构建工具从makefile改变为[cargo-make](https://github.com/sagiegurari/cargo-make)，配置文件为Makefile.toml。

**提醒：最好按照原教程中要求的环境和数据进行测试，以免由于多种原因出错后难以寻找。**



## 作业：

### 第一章

1. 为了方便 os 处理，Ｍ态软件会将 S 态异常/中断委托给 S 态软件，请指出有哪些寄存器记录了委托信息，rustsbi 委托了哪些异常/中断？（也可以直接给出寄存器的值）

   答：记录委托信息的寄存器为`mideleg`寄存器和`medeleg`寄存器，它们的值分别为：`0x222`和`0xb1ab`。

2. 请学习 gdb 调试工具的使用(这对后续调试很重要)，并通过 gdb 简单跟踪从机器加电到跳转到 0x80200000 的简单过程。只需要描述重要的跳转即可，只需要描述在 qemu 上的情况。

   tips:

> - 事实上进入 rustsbi 之后就不需要使用 gdb 调试了。可以直接阅读代码。[rustsbi起始代码](https://github.com/luojia65/rustsbi/blob/master/platform/qemu/src/main.rs#L93) 。
>
> - 可以使用示例代码 Makefile 中的 `make debug` 指令。
>
> - 一些可能用到的 gdb 指令：
>
>   - `x/10i 0x80000000` : 显示 0x80000000 处的10条汇编指令。
> - `x/10i $pc` : 显示即将执行的10条汇编指令。
>   - `x/10xw 0x80000000` : 显示 0x80000000 处的10条数据，格式为16进制32bit。
>   - `info register`: 显示当前所有寄存器信息。`info r t0`: 显示 t0 寄存器的值。
>   - `break funcname`: 在目标函数第一条指令处设置断点。
>   - `break *0x80200000`: 在 0x80200000 出设置断点。
>   - `continue`: 执行直到碰到断点。
>   - `si`: 单步执行一条汇编指令。
>   

答：通过clion的remote gdb功能，可以直接在ide里方便的在源码上进行调试（注意，需要使用debug模式编译），先以debug模式启动qemu，然后在代码中打上断点，运行remote gdb即可。虽然Clion很方便，但为了学习，接下来还是主要使用dgb来调试。

gdb在启动时，可直接带上调试文件`gdb target_file`，也可以在运行中通过`file`指令切换调试文件并重新加载符号表。所以这里可以先指定rustsbi的调试文件，等执行到kernel时，再切换到kernel调试。可能还需用到以下gdb指令：

- `target remote :1234`: 调试本地端口号1234的gdb信息（默认1234）。
- `where`: 显示当前运行位置。
- `info register csr` : 查看所有csr寄存器信息。
- `list [<start>],[end]`: 显示从start行到end行的源代码。
- `finish`： 执行完当前函数，并返回到函数调用处。

当gdb连上qemu时（机器加电），执行地址停在0x1000，在该处执行`x/10i $pc`查看接下来的10条汇编指令。然后使用si从0x1000开始执行到0x1014时，通过jr指令跳转到0x8000_0000处，即DRAM的内存起始地址，BootLoader也从这里开始（[Rustsbi的代码](https://github.com/luojia65/rustsbi/blob/master/platform/qemu/link-qemu.ld#L6)）。

![gdb01](extra\md_img\gdb01.png)

进入到rustsbi的[`start`方法](https://github.com/luojia65/rustsbi/blob/master/platform/qemu/src/main.rs#L93)，一来就是一个大的(asm!)宏包裹了一堆汇编指令，跳过不看，注意到其中有一条[` j main`指令](https://github.com/luojia65/rustsbi/blob/master/platform/qemu/src/main.rs#L117)代表跳转到`main`方法，rustsbi从这里`start`方法跳转到`main`方法。执行si命令到main方法去。

![gdb02](extra\md_img\gdb02.png)

`main`方法里，一来进入`mp_hook`方法，读取`mhartid`寄存器（Hart编号寄存器 ），并做出一些操作，这里运行时为0，所以直接返回true。

接下来向`mtvec`寄存器（异常入口基地址寄存器）中写入[`_start_trap`方法](https://github.com/luojia65/rustsbi/blob/master/platform/qemu/src/main.rs#L300)（该方法定义在main.rs中的全局asm中）的地址和trap的模式（这里为Direct模式）。

执行到`if mhartid::read() == 0 {`时，可通过`info registers mhartid`查看值，这里为0，所以进入if语句，而if语句中是一系列的操作没有跳转，故跳过。

接下来是一段unsafe方法，里面是设置`mideleg`寄存器和`medeleg`寄存器的值（设置中断和委托），同样通过命令可查看到，在开始设置前，`mideleg`寄存器和`medeleg`的值都是`0x0`，全部设置完后为`mideleg`的值为`0x222`，`medeleg`的值为`0xb1ab`。

下面再次判断`mhartid`的值，为0时打印输出一段sbi相关的信息。最后有一个[`count_harts`方法](https://github.com/luojia65/rustsbi/blob/master/platform/qemu/src/main.rs#L217)，断点进入方法后，使用`finish`指令执行完函数并查看返回值，我这里不知道为啥，只有1。

最后又是[一段unsafe](https://github.com/luojia65/rustsbi/blob/master/platform/qemu/src/main.rs#L223-L225)，这里到重点了。第一句设置`mepc`寄存器（异常PC寄存器）的值为`s_mode_start`方法的地址。第二句设置了`mestatus`寄存器（异常处理状态寄存器）当前的特权等级（`set_mpp`）为`Supervisor`。最后执行rustsbi的`enter_privileged`方法，查看代码，先是交换了`sp`寄存器和`mscratch`寄存器的值，最后执行了`mret`指令，该命令会切换到之前指定的特权等级并跳转到之前指定的`mepc`寄存器存储的地址即`s_mode_start`。

查看[`s_mode_start`的代码](https://github.com/luojia65/rustsbi/blob/master/platform/qemu/src/main.rs#L231)，修改了`ra`寄存器的值为`0x80200000`，执行了跳转指令，最终跳转到了`0x80200000`。

## 问题记录与反馈

#### 第一章第四节 [构建用户态执行环境](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/3-1-mini-rt-usrland.html)

在本章中，构建的是用户态的执行程序，所以模拟器也需要可直接执行用户态的`qemu-riscv64`，在查询了qemu官方的资料后得知，win上是不行的。故通过wsl2上的`qemu-riscv64`来运行这节的构建产物。

在尝试实验时我遇到了如下两个问题：

- [实现输出字符串的相关函数](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/3-1-mini-rt-usrland.html#id5) 时，在实现`Write trait`中调用`sys_write`方法时，使用了常量`STDOUT`，该常量没有在本章中给出。从[后面章节](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter2/2application.html#id6) 得知控制台的是`1`，即需要在此定义`const STDOUT: usize = 1;`。

- 在完成上面的函数后，教程中提到，这时会编译失败。

  > 系统崩溃了！借助以往的操作系统内核编程经验和与下一节调试kernel的成果经验，我们直接定位为是 **栈 stack** 没有设置的问题。我们需要添加建立栈的代码逻辑。

  但实际测试时，是直接通过的，反而进行了下面的栈设置之后才会出错。猜测本章节是用户态的程序，而设置栈相关的事物应该是在裸机环境，可能是编写时，不小心将后面教程的部分放到了这里。

在这一章的测试时，还发现了一个有意思的事情，编译出来的产物，可以直接在wsl2上的Ubuntu运行(20.04)。刚开始还以为是编译时带了什么东西，后来将产物拷贝到几台不同的服务器上测试都不行。编译产物放在了`extra/tmp_file`下。

![docker中运行](./extra/md_img/img1.png)![wsl2中运行](./extra/md_img/img2.png)



#### 第一章第五节 [构建裸机运行时执行环境](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/3-2-mini-rt-baremetal.html)

在尝试实验时我遇到了如下两个问题：

- [实现关机功能](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/3-2-mini-rt-baremetal.html#id6) 时，使用了一个常量`SBI_SHUTDOWN`，该常量没有在本章中给出。通过查询[riscv-sbi-doc](https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc) 得到该值为`8`。但是在设置好之后，其他都是能执行的（打印出了hello world），但还是不能正常退出`qemu`虚拟机，打印出了如下错误，并且`qemu`程序不会结束：

  ```shell
  panicked at 'Unhandled exception! mcause: Exception(StoreFault), mepc: 000000008000261c, mtval: 0000000000100000', platform/qemu/src/main.rs:395:18
  ```
  

上面使用的`win64`的`qemu`，版本为：`QEMU emulator version 5.2.0 (v5.2.0-11850-g0f27b14b91-dirty)`。后在wsl2中使用提供的docker镜像环境测试，运行正常。

- [清空 .bss 段](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/3-2-mini-rt-baremetal.html#bss) 这个功能，教程里描述的是：

  > 我们需要提供清零的 `clear_bss()` 函数。此函数属于执行环境，并在执行环境调用 应用程序的 `rust_main` 主函数前，把 `.bss` 段的全局数据清零。

我理解的是只需要把函数声明在这里，会自动在`rust_main`前调用。教程里这一节也确实没有调用相关的信息。但是在执行时，感觉像是没有被调用的。