#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, Copy)]
    pub struct WhatCanCopy {
        foo: i32,
        barr: Option<[i32; 5]>,
    }

    pub fn dingus() {
        let foo: WhatCanCopy = WhatCanCopy { foo: 1, barr: None };
    }
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
