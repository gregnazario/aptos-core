module 0xc0ffee::m {
    enum Wrapper {
        V1(u64),
    }

    public inline fun use_me_not(): u64 {
        let x = Wrapper::V1(22);
        x.0
    }
}

module 0xc0ffee::n {
    use 0xc0ffee::m;

    fun test(): u64 {
        m::use_me_not()
    }
}
