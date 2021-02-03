做清华大学操作系统rCore的实验。

实验教程地址：https://rcore-os.github.io/rCore-Tutorial-Book-v3/index.html

原实验构建环境为ubuntu，提供了模拟器和真机等多种运行环境的模式。

笔者这里构建环境使用win10，运行环境为qemu，并且为了方便，构建工具从makefile改变为[cargo-make](https://github.com/sagiegurari/cargo-make)，配置文件为Makefile.toml。



## 问题记录与反馈

#### 第一章第四节 [构建用户态执行环境](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/3-1-mini-rt-usrland.html)

在本章中，构建的是用户态的执行程序，所以模拟器也需要可直接执行用户态的`qemu-riscv64`，在查询了qemu官方的资料后得知，win上是不行的。故通过wsl2上的`qemu-riscv64`来运行这节的构建产物。

在尝试实验时我遇到了如下两个问题：

- [实现输出字符串的相关函数](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/3-1-mini-rt-usrland.html#id5) 时，在实现`Write trait`中调用`sys_write`方法时，使用了常量`STDOUT`，该常量并没有在本章中给出。从[后面章节](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter2/2application.html#id6) 得知控制台的是`1`，即需要在此定义`const STDOUT: usize = 1;`。

- 在完成上面的函数后，教程中提到，这时会编译失败。

  > 系统崩溃了！借助以往的操作系统内核编程经验和与下一节调试kernel的成果经验，我们直接定位为是 **栈 stack** 没有设置的问题。我们需要添加建立栈的代码逻辑。

  但实际测试时，是直接通过的，反而进行了下面的栈设置之后才会出错。猜测本章节是用户态的程序，而设置栈相关的事物应该是在裸机环境，可能是编写时，不小心将后面教程的部分放到了这里。

在这一章的测试时，还发现了一个有意思的事情，编译出来的产物，可以直接在wsl2上的Ubuntu运行(20.04)。刚开始还以为是编译时带了什么东西，后来将产物拷贝到几台不同的服务器上测试都不行。编译产物放在了`extra/tmp_file`下。

![docker中运行](./extra/md_img/img1.png)![wsl2中运行](./extra/md_img/img2.png)