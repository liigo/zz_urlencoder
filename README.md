# zz_urlencoder
`application/x-www-form-urlencoded` encoder FFI for C and Eyuyan

Windows动态链接库，已静态链接C运行库，可供C语言和易语言调用。

需使用Rust 1.15 nightly或以上版本编译：`cargo build --release`。

```rust
/// 对input文本执行`application/x-www-form-urlencoded`编码，结果写入buf缓冲区
///
/// 参数input为UTF-8格式的文本数据地址，inputlen是其字节数
/// 参数buf为结果缓冲区，buflen是其字节数
///
/// 如果缓冲区长度不足以存放编码结果文本，将返回实际所需缓冲区字节数的负值，保持缓冲区内容不变
/// 如果缓冲区长度足够，将向其写入编码结果文本和结尾'\0'字符，并返回写入的字节数（正值）
/// 
/// by Liigo, 20170207.
#[no_mangle]
pub extern fn form_urlencode(input: i32, inputlen: i32, buf: i32, buflen: i32) -> i32;
```
