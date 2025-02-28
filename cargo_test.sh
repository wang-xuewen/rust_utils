#!/bin/bash

# --nocapture 参数禁用输出捕获，使所有测试（无论通过与否）的 println!、dbg! 等打印内容完整显示在终端‌
cargo test -- --nocapture
