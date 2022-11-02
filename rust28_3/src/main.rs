#![allow(unused, warnings)]
// 特征对象
#[derive(Debug)]
enum UiObject {
   Button,
   SelectBox, 
}
fn draw(o: &UiObject) {
    println!("{:?}", o);
}
// 在编写这个 UI 库时，我们无法知道所有的 UI 对象类型


//  Rust 并没有继承，我们得另寻出路。
// 先来为之前的 UI 组件定义一个特征
pub trait Draw {
    fn draw(&self);
}
// 只要组件实现了 Draw 特征，就可以调用 draw 方法来进行渲染
// 假设有一个 Button 和 SelectBox 组件实现了 Draw 特征：
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮的代码
    }   
}
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
    }
}
struct Customized {
    random1: u32,
    random2: String,
    random3: Vec<u32>,
}
impl Draw for Customized {
    fn draw(&self) {
        println!("Create Customized struct");
    }
}
// 此时，还需要一个动态数组来存储这些 UI 对象：
// 存储了一个动态数组，里面元素的类型是 Draw 特征对象：
// Box<dyn Draw>，任何实现了 Draw 特征的类型，都可以存放其中。
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
// Screen 定义 run 方法，用于将列表中的 UI 组件渲染在屏幕上：
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
// 这种写法限制了 Screen 实例的 Vec<T> 中的每个元素必须是 
// Button 类型或者全是 SelectBox 类型
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
//     where T: Draw {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }


// 特征对象的动态分发
// 泛型是在编译期完成处理的：编译器会为每一个泛型参数对应的
// 具体类型生成一份代码，这种方式是静态分发(static dispatch)
// 当使用特征对象时，Rust 必须使用动态分发。

// 特征对象的限制
// 不是所有特征都能拥有特征对象，只有对象安全的特征才行
// 方法的返回类型不能是 Self 方法没有任何泛型参数




fn main() {
    let objects = [
        UiObject::Button,
        UiObject::SelectBox,
    ];
    for o in objects.iter(){
        draw(o);
    }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(Customized {
                random1: 1,
                random2: "2".to_string(),
                random3:vec![1,2,3],
            })
        ],
    };
    screen.run();
    // 上面使用 Box::new(T) 的方式来创建了两个 Box<dyn Draw> 特征对象，如果以后还需要增加一个 UI 组件，那么让该组件实现 Draw 特征，
    // 则可以很轻松的将其渲染在屏幕上，甚至用户可以引入我们的库作为三方库，然后在自己的库中为自己的类型实现 Draw 特征，然后进行渲染。

    // 在动态类型语言中，有一个很重要的概念：鸭子类型(duck typing)，简单来说，就是只关心值长啥样，而不关心它实际是什么。当一个东西走起来像鸭子，叫起来像鸭子，
    // 那么它就是一只鸭子，就算它实际上是一个奥特曼，也不重要，我们就当它是鸭子。
    
    // 在上例中，Screen 在 run 的时候，我们并不需要知道各个组件的具体类型是什么。它也不检查组件到底是 Button 还是 SelectBox 的实例，只要它实现了 Draw 特征，
    // 就能通过 Box::new 包装成 Box<dyn Draw> 特征对象，然后被渲染在屏幕上。
    
    // 使用特征对象和 Rust 类型系统来进行类似鸭子类型操作的优势是，无需在运行时检查一个值是否实现了特定方法或者担心在调用时因为值没有实现方法而产生错误。
    // 如果值没有实现特征对象所需的特征， 那么 Rust 根本就不会编译这些代码：
}
