extern crate quickcheck;
use self::quickcheck::Testable;

trait Unwrap {
    type Item;
    fn unwrap (self) -> Self::Item;
}

// Tagging structs for quickcheck testing
pub struct Valid <A : Testable> (A);
pub struct Invalid <A : Testable> (A);

impl <A : Testable> Unwrap for Valid <A> {
    type Item = A;
    fn unwrap (self) -> Self::Item {
        let Valid (underlying) = self;
        underlying
    }
}

impl <B : Testable> Unwrap for Invalid <B> {
    type Item = B;
    fn unwrap (self) -> Self::Item {
        let Invalid (underlying) = self;
        underlying
    }
}
