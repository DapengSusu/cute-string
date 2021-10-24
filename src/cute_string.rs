use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::str;

const MINI_STRING_MAX_LEN: usize = 30;

// CuteString 里，String 有 3 个 word，共 24 字节，所以它以 8 字节对齐
// enum 的 tag + padding 最少 8 字节，整个结构占 32 字节
// MiniString 可以最多有 30 字节（再加上 1 字节长度和 1字节 tag），就是 32 字节
pub struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        // 我们在拷贝内容时一定要要使用字符串的字节长度
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];

        data[..len].copy_from_slice(bytes);

        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // 由于生成 MiniString 的接口是隐藏的，它只能来自字符串，所以下面这行是安全的
        str::from_utf8(&self.data[..self.len as usize]).unwrap()

        // 也可以直接用 unsafe 版本
        // unsafe { str::from_utf8_unchecked(&self.data[..self.len as usize]) }
    }
}

impl Debug for MiniString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 这里由于实现了 Deref trait，可以直接得到一个 &str 输出
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
pub enum CuteString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for CuteString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            CuteString::Inline(ref v) => v.deref(),
            CuteString::Standard(ref v) => v.deref(),
        }
    }
}

impl<T> From<T> for CuteString
    where T: AsRef<str>
{
    fn from(s: T) -> Self {
        match s.as_ref().len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(s.as_ref().to_owned()),
            _ => Self::Inline(MiniString::new(s)),
        }
    }
}

impl Display for CuteString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl CuteString {
    pub fn push_str(&mut self, s: &str) {
        match *self {
            CuteString::Inline(ref mut v) => {
                let len = v.len as usize;
                let len_s = s.len();

                if len + len_s > MINI_STRING_MAX_LEN {
                    let mut owned = v.deref().to_string();

                    owned.push_str(s);
                    *self = CuteString::Standard(owned);
                } else {
                    v.data[len..len + len_s].copy_from_slice(s.as_bytes());
                    v.len = (len + len_s) as u8;
                }
            },
    
            CuteString::Standard(ref mut v) => v.push_str(s),
        }
    }
}
