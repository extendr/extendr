use extendr_api::prelude::*;

#[derive(Debug, PartialEq)]
struct Mock {}

impl From<Mock> for RObj {
    fn from(_: Mock) -> Self {
        r!(())
    }
}

struct NotExecutedMock {}

impl From<NotExecutedMock> for RObj {
    fn from(_: NotExecutedMock) -> Self {
        unreachable!()
    }
}

#[test]
fn into_robj() {
    test! {
        let left : Either<Mock, NotExecutedMock> = Left(Mock{});
        let robj : RObj = left.into();

        assert_eq!(r!(()), robj);

        let right : Either<NotExecutedMock, Mock> = Right(Mock{});
        let robj : RObj = right.into();

        assert_eq!(r!(()), robj);
    }
}
