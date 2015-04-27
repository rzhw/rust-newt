type NewtComponent = u32;

#[link(name = "newt")]
extern {
    fn newtInit();
    fn newtCls();
    fn newtWaitForKey();
    fn newtDrawRootText(col: i32, row: i32, text: *const i8);
    fn newtFinished();
    fn newtCenteredWindow(width: i32, height: i32, text: *const i8) -> i32;
    fn newtForm(vertBar: NewtComponent, help: *const i8, flags: i32) -> NewtComponent;
    fn newtFormAddComponent(form: NewtComponent, co: NewtComponent);
    fn newtFormDestroy(form: NewtComponent);
    fn newtRunForm(form: NewtComponent) -> NewtComponent;
    fn newtButton(left: i32, top: i32, text: *const i8) -> NewtComponent;
    fn newtLabel(left: i32, top: i32, text: *const i8) -> NewtComponent;
}
