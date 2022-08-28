// fn main() {
//     lview::render(StartPlay);
// }

// //You can draw elements as pure functions if they are simple enough
// fn app() -> View {
//     vbuild!(
//         <view->
//             <StartPlay>
//         <-view>
//     )
// }

// //Or draw elements as structs if you need more control
// struct Counter {
//     count: i32,
// }
// impl VDesign for Counter {
//     fn design(&self) -> DesignQuery {
//         vdesign!(
//             * => {
//                 transform = relative(90., 90.)
//             },
//             #txt-button => {
//                 color = Color::new(0, 0, 0, 255),
//                 bold = true,
//             }
//         )
//     }
// }
// impl VStructure for Counter {
//     fn root(&self) -> View {
//         vbuild!(
//             <view->
//                 <button on-press=Counter::add_count->
//                     <text id="txt-button"->Play!<-text>
//                 <-button>
//             <-view>
//         )
//     }
// }
// impl Counter {
//     fn add_count(&mut self) {
//         self.count += 1;
//     }
// }
fn main() {}
