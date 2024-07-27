#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    #[derive(Debug, Clone, Copy)]
    pub struct WhatCanCopy {
        foo: i32,
        barr: Option<[i32; 5]>,
    }

    pub fn dingus() {
        let foo: WhatCanCopy = WhatCanCopy { foo: 1, barr: None };
        println!("{:?}", foo)
    }
    #[test]
    fn it_works() {
        let result = 2 + 2;
        dingus();
        assert_eq!(result, 4);
    }
}
