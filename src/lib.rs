extern crate url;

use std::slice;
use std::ptr;
use url::form_urlencoded;

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
pub extern fn form_urlencode(input: i32, inputlen: i32, buf: i32, buflen: i32) -> i32 {
    let utf8 = unsafe { 
        slice::from_raw_parts_mut(input as *mut u8, inputlen as usize)
    };

    let ss: Vec<&str> = form_urlencoded::byte_serialize(utf8).collect();
    let required_buflen = (ss.iter().map(|s| s.len()).sum::<usize>() + 1) as i32;

    if buflen < required_buflen {
        return 0 - required_buflen;
    }
    
    let mut pbuf = buf as *mut u8;
    for s in ss {
        unsafe {
            ptr::copy_nonoverlapping(s.as_ptr(), pbuf, s.len());
            pbuf = pbuf.offset(s.len() as isize);
        }
    }
    unsafe {
        ptr::write(pbuf, 0); // write the ending '\0'
    }
    required_buflen
}
