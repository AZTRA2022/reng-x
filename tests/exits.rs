#[cfg(test)]
mod tests {
    #[test]
    fn voz() {
        let s: &str = "123";
        let ptr: *const u8 = s.as_ptr();
        println!("voz test");
        unsafe {
            assert_eq!(*ptr.add(1), b'2');
            assert_eq!(*ptr.add(2), b'3');
        }
    }
}
