#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod pkg_desc {
    pub const HEAD_SIZE:usize = 24;
    pub const MAX_BODY_SIZE: usize = 64*1024;
    pub const MAGIC_IN_HEAD: i32 = 9527;
    pub const MAX_PKG_SIZE: usize = MAX_BODY_SIZE + HEAD_SIZE;

    pub struct pkg_head {
        pub magic: i32,
        pub pkg_len: usize,
        pub client_id:i64,
    }

    impl pkg_head {
        pub fn new() -> pkg_head {
            pkg_head {
                magic : 0,
                pkg_len : 0,
                client_id : 0,
            }
        }
    }

    pub struct pkg_body {
        pub message:[u8;MAX_BODY_SIZE]
    }

    impl pkg_body {
        pub fn new() -> pkg_body {
            pkg_body {
                message : [0; MAX_BODY_SIZE]
            }
        }
    }

    pub struct pkg {
        pub head: pkg_head,
        pub body: pkg_body,
    }

    impl pkg {
        pub fn new() -> pkg {
            pkg {
                head : pkg_head::new(),
                body : pkg_body::new(),
            }
        }
    }

}