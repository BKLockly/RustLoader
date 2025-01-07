# 安装mingw64(C语言环境)

Rust默认使用的C语言依赖Visual Studio，但该工具占用空间大安装也较为麻烦，可以选用轻便的mingw64包。  
1.1 安装地址

(1) 下载地址1-GitHub：[Releases · niXman/mingw-builds-binaries · GitHub](https://github.com/niXman/mingw-builds-binaries/releases)  
(2) 下载地址2-WinLibs：[WinLibs - GCC+MinGW-w64 compiler for Windows](https://winlibs.com/)

​![image](screenshot/image-20241102084848-z0zwgqq.png)​

‍

‍

## 安装rust

不想安装到c盘故设置环境变量

RUSTUP\_HOME D:\\env\\rust\\rustup\_home  
 CARGO\_HOME D:\\env\rust\\cargo\_home

​![image](screenshot/image-20241102085521-qa3j08u.png)​

(1) 直接从官方网站下载会很慢，改用国内镜像加速安装，设置以下环境变量：

RUSTUP_DIST_SERVER https://mirrors.tuna.tsinghua.edu.cn/rustup  
RUSTUP_UPDATE_ROOT https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup

​![image](screenshot/image-20241102085644-yd380q4.png)​

配置库镜像（类似python的pip）：在`C:\\Users\\用户名\\`​下创建`.cargo`​文件夹，在文件夹内创建`config.tom`​文件，如下图：

​![image](screenshot/image-20241102095152-ew3pb74.png)​

[官网](https://www.rust-lang.org/zh-CN/tools/install)下载

​![image](screenshot/image-20241102085345-kirhbqb.png)​

选项要求必须安装C/C++的编译环境，默认是 visual studio安装器，而此次使用mingw64，因此需要手动修改为 **2**，然后输入 **y**，如下图：

​![image](screenshot/image-20241102090013-haltb8d.png)​

继续输入2:

​![image](screenshot/image-20241102090115-my18nhr.png)​

接下来一直回车开始下载:

​![image](screenshot/image-20241102090251-83hwbyl.png)​

看到这里的提示已完成,退出窗口

​![image](screenshot/image-20241102090323-67gs7mo.png)​

验证一下安装情况

```bash
rustc --version
```

​![image](screenshot/image-20241102090538-hm1fnu6.png)​
