use std::marker::PhantomData;

use derive_new::new;

use crate::client::mysql::Rds;

// 継承元になるクラス
#[derive(new)]
pub struct RdsRepositoryImpl<T> {
    pool: Rds,
    
    // コンパイル時に完全に無視されるデータ
    // RdsRepositoryImpl<Todo> という型を作ってくれる
    // RdsRepositoryImpl<New> とは全く別のクラス
    // Uuid<User>, Uuid<BidReq>, Uuid<ad>
    // impl Uuid { fn generate() -> {} } というので書くことができる
    // 型としてサポートしてくれる
    // TryFrom<T> も同じことをしている
    _marker: PhantomData<T>,
}

pub mod todo;
