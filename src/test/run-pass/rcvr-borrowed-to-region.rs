trait get {
    fn get() -> int;
}

// Note: impl on a slice
impl &int: get {
    fn get() -> int {
        return *self;
    }
}

fn main() {
    /*
    let x = @mut 6;
    let y = x.get();
    assert y == 6;
    */

    let x = @6;
    let y = x.get();
    debug!{"y=%d", y};
    assert y == 6;

    let x = ~mut 6;
    let y = x.get();
    debug!{"y=%d", y};
    assert y == 6;

    let x = ~6;
    let y = x.get();
    debug!{"y=%d", y};
    assert y == 6;

    let x = &6;
    let y = x.get();
    debug!{"y=%d", y};
    assert y == 6;
}
