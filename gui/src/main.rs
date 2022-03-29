struct Checkbox; 

struct Button;  

struct Radiobutton; 

struct Table; 

struct Window {
    // dyn keyword indicates some kind of implementation for trait 
    // Box means this is being placed on the heap - don't need to indicate size 
    items: Vec<Box<dyn WidgetFuncs>>,
}

// a Trait can be implemented for multiple types 
trait WidgetFuncs {
    fn draw(&self);
}

impl WidgetFuncs for Table {
    fn draw(&self){}
}

impl WidgetFuncs for Window {
    fn draw(&self) {
        for item in self.items.iter() {
            item.draw() 
        }
    }
}

impl Widgetfuncs for Checkbox {
    fn draw(&self) {}
}

fn draw_it(widget: &dyn WidgetFuncs) {
    widget.draw()
}

fn main() {
    println!("Hello, world!");
    let window = Window {
        items: vec![Box:new(Checkbox), Box::new(RadioButton)]
    }
    window.draw();

    draw_it(&*Box::new(Checkbox));
    draw_it(&Checkbox);
}
