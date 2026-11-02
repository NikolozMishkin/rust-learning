trait Drawable {
    fn draw(&self);
    // fn no_self();
    // fn no_self() {}
    fn no_self()
    where
        Self: Sized,
    {
    }

    fn generic_fn<T>(&self, generic: T)
    // add sized later on by by writting
    where
        Self: Sized,
    {
    }
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!(
            "Drawing a circle at ({}, {}), radius {}",
            self.x, self.y, self.radius
        );
    }
    fn no_self() {
        println!("no_self() for Circle");
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!(
            "Drawing a square at ({}, {}), side {}",
            self.x, self.y, self.side
        );
    }
    fn no_self() {
        println!("no_self() for Square");
    }
}

fn draw_ones<T: Drawable>(obj: &T) {
    obj.generic_fn(3);
    // obj.no_self();
    // (dyn Drawable)::no_self(); //type erased

    Circle::no_self();
    Square::no_self();
    //Drawable::no_self(); // not allowed
}

fn draw(obj: &dyn Drawable) {
    // obj.generic_fn(3);
    // obj.no_self();
    // (dyn Drawable)::no_self(); //type erased

    Circle::no_self();
    Square::no_self();
    //Drawable::no_self(); // not allowed
}
fn main() {}

/*
---------------------------------------------------------------------------------------------------
Тема                                | Объяснение
---------------------------------------------------------------------------------------------------
Object safety и методы без self     | Методы трейта без self нельзя вызвать через trait object.
                                    | Во время выполнения они не привязаны к конкретному типу.
                                    | Это нарушает object safety (dyn compatibility) трейта.

Проблема построения vtable          | vtable хранит указатели на функции конкретных типов.
                                    | Метод без self нельзя сопоставить с конкретной реализацией:
                                    | нет объекта, значит нет доступа к vtable.
                                    | Поэтому такие методы нельзя осмысленно положить в vtable.

Исключение через Self: Sized        | where Self: Sized убирает метод из vtable.
                                    | Такой метод нельзя вызвать через trait object (dyn Trait).
                                    | Но его можно вызывать на конкретных типах (Circle::no_self()).

Связь с дженерик-методами           | Дженерик-методы тоже не object safe по умолчанию:
                                    | мономорфизация даёт бесконечно много версий функции.
                                    | Self: Sized исключает их из динамической диспетчеризации,
                                    | и остальной трейт остаётся object safe.
---------------------------------------------------------------------------------------------------
*/
