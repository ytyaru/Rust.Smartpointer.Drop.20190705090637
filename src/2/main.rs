/*
 * スマートポインタ（Dropトレイト）。
 * CreatedAt: 2019-07-05
 */
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) { println!("Drop!!: {}", self.data); }
}
fn main() {
    let c = CustomSmartPointer { data: String::from("my") };
    let d = CustomSmartPointer { data: String::from("other") };
    drop(c);
    println!("Created.");
}
