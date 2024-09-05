
fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window {
  
        width: 500px;
        height: 300px;
        // background: #3960D5;

        Text {
            text: "hello world";
            color: green;
        }
    }
}
