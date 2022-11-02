#![allow(warnings, unused)]

// #![...] 将对整个文件有效, #[...]只对该行下面的块有效

// 复合类型 结构体struct和枚举enum

type File = String;

fn open(f: &mut File) -> bool{
    true
}

fn close(f: &mut File) -> bool{
    true
}

#[allow(dead_code)]
// read 函数也非常有趣，它返回一个 ! 类型，这个表明该函数是一个发散函数，不会返回任何值，包括 ()。
fn read(f: &mut File, save_to: &mut Vec<u8>) -> !{
    unimplemented!()
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    //read(&mut f1, &mut vec![]);
    close(&mut f1);
}
