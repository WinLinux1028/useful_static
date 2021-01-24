use std::sync::Mutex;

/// # The safety global variable wrapper
/// You should use this if you have a special reason
/// ## Examples
/// ```
/// use useful_static::Global;
///
/// static TEST: Global<String> = Global::new();
///
/// fn main() {
///     TEST.set(String::from("Hello World!"));
///     println!("{}", TEST.lock().unwrap()) //Hello World!
/// }
/// ```
/// ## Panics
/// If you use `deref` method to this without `.set` method, It will panics.
/// ```
/// use useful_static::Global;
///
/// static TEST: Global<String> = Global::new();
///
/// fn main() {
///     println!("{}", TEST.lock().unwrap()) //Panic!
/// }
/// ```
pub struct Global<T> {
    content: RawGlobal<Mutex<T>>,
}

impl<T> Global<T> {
    ///Create new `Global` instance.
    pub const fn new() -> Global<T> {
        Global {
            content: RawGlobal::new(),
        }
    }

    ///Set value to `Global`.
    pub fn set(&self, content: T) {
        self.content.set(Mutex::new(content));
    }
}

impl<T> std::ops::Deref for Global<T> {
    type Target = Mutex<T>;

    fn deref(&self) -> &Self::Target {
        &*self.content
    }
}

/// # The global variable for other Mutex
/// If you want to use other Mutex, you should use this.
/// This will useful to read-only global variable too.
/// ## Examples
/// ```
/// use useful_static::RawGlobal;
/// use tokio::sync::Mutex;
///
/// static TEST: RawGlobal<Mutex<String>> = RawGlobal::new();
///
/// #[tokio::main]
/// async fn main() {
///     TEST.set(Mutex::new(String::from("Hello World!")));
///     println!("{}", TEST.lock().await) //Hello World!
/// }
/// ```
/// ## Panics
/// If you use `deref` method to this without `.set` method, It will panics.
/// ```
/// use useful_static::RawGlobal;
/// use std::sync::Mutex;
///
/// static TEST: RawGlobal<Mutex<String>> = RawGlobal::new();
///
/// fn main() {
///     println!("{}", TEST.lock().unwrap()) //Panic!
/// }
/// ```
pub enum RawGlobal<T> {
    No(Option<Mutex<()>>),
    Yes(T),
}

impl<T> RawGlobal<T> {
    ///Create new `RawGlobal` instance.
    pub const fn new() -> RawGlobal<T> {
        RawGlobal::No(None)
    }

    /// Set value to `RawGlobal`.
    pub fn set(&self, content: T) {
        unsafe {
            let global = self as *const _ as *mut _;
            *global = RawGlobal::Yes(content);
        }
    }
}

impl<T> std::ops::Deref for RawGlobal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            RawGlobal::No(_) => panic!("Set value to this variable."),
            RawGlobal::Yes(a) => a,
        }
    }
}