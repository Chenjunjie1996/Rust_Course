#![allow(unused,warnings)]
// Result<T, E> 用于可恢复错误，panic! 用于不可恢复错误。
// 一定是不可恢复的错误，才调用 panic! 处理
// 只有当你不知道该如何处理时，再去调用 panic!.
/*
如果是 main 线程，则程序会终止，如果是其它子线程，该线程会终止
但是不会影响 main 线程。因此，尽量不要在 main 线程中做太多任务，
将这些任务交由子线程去做，就算子线程 panic 也不会导致整个程序的结束。
*/

use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    // parse试图将字符串"127.0.0.1"解析为一个 IP 地址类型 IpAddr
    // 它返回一个 Result<IpAddr, E> 类型，如果解析成功，
    // 则把 Ok(IpAddr) 中的值赋给 home，
    // 如果失败，则不处理 Err(E)，而是直接 panic。
    // 因此 unwrap 简而言之：成功则返回值，失败则 panic

}
