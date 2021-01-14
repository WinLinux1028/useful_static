use std::sync::Mutex;

//First, you have to write in your program like:
//static FOO: Global<String> = Global::No;
pub enum Global<T> {
    No,
    Yes(Mutex<T>),
}

impl<T> Global<T> {
    //Second, set value by this method.
    pub fn set(&self, content: T) {
        unsafe {
            let global = self as *const _ as *mut _;
            *global = Global::Yes(Mutex::new(content));
        }
    }
}

impl<T> std::ops::Deref for Global<T>{
    type Target = Mutex<T>;

    //Third, you can use global variables like std::sync::Mutex.
    fn deref(&self) -> &Mutex<T> {
        match self {
            Global::No => panic!("You have to do .set method before use this."),
            Global::Yes(ref a) => a,
        }
    }
}