相关连接：
- https://tokio.rs/tokio/tutorial/hello-tokio
- https://tokio-zh.github.io/document/io/overview.html
- https://github.com/dslchd/tokio-cn-doc/blob/master/doc/HelloTokio.md

Tokio介绍：
    Tokio是一个事件驱动的非阻塞I/O平台，用于使用Rust编程语言编写异步应用程序。在较高的层面上，它提供了一些主要组件：
    - 基于多线程，工作窃取的任务调度程序。
    - 由操作系统的事件队列（epoll，kqueue，IOCP等）支持的反应器。
    - 异步TCP和UDP套接字。
    这些组件提供构建异步应用程序所需的运行时组件。Tokio是rust编程语言的异步运行时，它提供了编写网络应用所需的构建块

准备工作：
    - 安装mini-redis,用于调试我们的开发： cargo install mini-redis
    - 起一个终端执行：mini-redis-server
    - 再起一个终端执行： mini-redis-cli get foo

