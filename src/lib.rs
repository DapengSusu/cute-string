pub mod cute_string;

#[cfg(test)]
mod tests {
    use crate::cute_string::CuteString;
    use crate::cute_string::MiniString;

    #[test]
    fn it_works() {
        let len1 = std::mem::size_of::<CuteString>();
        let len2 = std::mem::size_of::<MiniString>();

        println!("len1: {}, len2: {}", len1, len2);

        let s1: CuteString = "Hello Rust".into();
        let s2: CuteString = "这是一个超过了三十个字节的很长很长的字符串".into();

        // debug输出
        println!("s1: {:?}, s2: {:?}", s1, s2);

        // display输出
        println!(
            "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
            s1, s1.len(), s1.chars().count(),
            s2, s2.len(), s2.chars().count()
        );

        // CuteString 可以使用一切 &str 接口
        assert!(s1.ends_with("Rust"));
        assert!(s2.starts_with('这'));

        let s = String::from("这是一个超过了三十个字节的很长很长的字符串");
        println!("s: {:p}", &*s);
        // From<T: AsRef<str>> 的实现会导致额外的复制
        let s3: CuteString = s.into();
        println!("s3: {:p}", &*s3);

        let mut s4: CuteString = "Hello Rust! ".into();
        println!("s4: {:?}", s4);
        s4.push_str("这是一个超过了三十个字节的很长很长的字符串");
        println!("s4: {:?}", s4);
    }
}
