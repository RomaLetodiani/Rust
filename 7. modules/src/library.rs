pub mod members {
    pub fn member1() {
        println!("member1");
    }
    pub fn member2() {
        println!("member2");
    }

    pub mod special_members {
        pub fn special_member1() {
            println!("special_member1");
        }
        pub fn special_member2() {
            println!("special_member2");
        }
    }
}