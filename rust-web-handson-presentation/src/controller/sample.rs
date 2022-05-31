use mockall::*;
use mockall::predicate::*;
#[automock]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

fn call_with_four(x: &MyTrait) -> u32 {
    x.foo(4)
}

#[cfg(test)]
mod tests {
    #[test]
    fn sampleEtoETestBlocking() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo()
            .with(predicate::eq(4))
            .times(1)
            .returning(|x| x + 1);
        assert_eq!(5, 4);
        assert_eq!(5, call_with_four(&mock));
    }
}

