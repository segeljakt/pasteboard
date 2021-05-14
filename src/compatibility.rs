pub(crate) trait IntoExpected<T> {
    fn into_expected(self) -> T;
}

impl IntoExpected<i8> for bool {
    fn into_expected(self) -> i8 {
        if self {
            1
        } else {
            0
        }
    }
}

impl IntoExpected<bool> for i8 {
    fn into_expected(self) -> bool {
        self > 0
    }
}

impl IntoExpected<i8> for i8 {
    fn into_expected(self) -> i8 {
        self
    }
}

impl IntoExpected<bool> for bool {
    fn into_expected(self) -> bool {
        self
    }
}
