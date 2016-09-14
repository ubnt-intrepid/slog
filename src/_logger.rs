
/// Logger
///
/// Loggers can be freely passed around the code.
///
/// Cloning existing loggers and creating new ones is cheap,
///
/// Root logger starts a new hierarchy and needs drain to be created.
/// Root logger drain, must taken care of error handling. See
/// `DrainExt::ignore_err()` and `DrainExt::fuse()`.
///
/// Child loggers are build from existing loggers, and inherit existing
/// key-value pairs from their parents, which can be supplemented with
/// new ones.
#[derive(Clone)]
pub struct Logger {
    //TODO:
    //drain: Arc<Drain<Error=!>>,
    drain: Arc<Drain<Error=()>>,
    values: Arc<OwnedKeyValueList>,
}

impl Logger {
    /// Build a root logger
    ///
    /// All children and their children and so on form one hierarchy
    /// sharing a common drain.
    ///
    /// Use `o!` macro to help build key-value pairs with a nicer syntax.
    ///
    /// ```
    /// #[macro_use]
    /// extern crate slog;
    ///
    /// fn main() {
    ///     let root = slog::Logger::root(
    ///         slog::discard(),
    ///         o!("key1" => "value1", "key2" => "value2"),
    ///     );
    /// }
    pub fn root<D: 'static + Drain<Error=()> + Sized>(d: D, values: Vec<OwnedKeyValue>) -> Logger {
        Logger {
            drain: Arc::new(d),
            values: Arc::new(OwnedKeyValueList::root(values)),
        }
    }

    /// Build a child logger
    ///
    /// Child logger inherits all existing key-value pairs from it's parent.
    ///
    /// All children, their children and so on, form one hierarchy sharing
    /// a common drain.
    ///
    /// Use `o!` macro to help build key value pairs using nicer syntax.
    ///
    /// ```
    /// #[macro_use]
    /// extern crate slog;
    ///
    /// fn main() {
    ///     let root = slog::Logger::root(slog::discard(),
    ///         o!("key1" => "value1", "key2" => "value2"));
    ///     let log = root.new(o!("key" => "value"));
    /// }
    pub fn new(&self, values: Vec<OwnedKeyValue>) -> Logger {
        Logger {
            drain: self.drain.clone(),
            values: Arc::new(OwnedKeyValueList::new(values, self.values.clone())),
        }
    }

    /// Log one logging record
    ///
    /// Use specific logging functions instead. See `log!` macro
    /// documentation.
    #[inline]
    pub fn log(&self, record: &Record) {
        let _ = self.drain.log(&record, &*self.values);
    }
}

#[doc(hidden)]
pub struct RecordStatic<'a> {
    /// Logging level
    pub level: Level,
    /// File
    pub file: &'static str,
    /// Line
    pub line: u32,
    /// Column (currently not implemented)
    pub column: u32,
    /// Function (currently not implemented)
    pub function: &'static str,
    /// Module
    pub module: &'static str,
    /// Target - for backward compatibility with `log`
    pub target: &'a str,
}

/// Logging record
pub struct Record<'a> {
    meta: &'a RecordStatic<'a>,
    msg: fmt::Arguments<'a>,
    values: &'a [BorrowedKeyValue<'a>],
}


impl<'a> Record<'a> {
    /// Create a new `Record`
    #[inline]
    #[doc(hidden)]
    pub fn new(
        s : &'a RecordStatic<'a>,
        msg: fmt::Arguments<'a>,
        values: &'a [BorrowedKeyValue<'a>],
        ) -> Self {
        Record {
            meta: s,
            msg: msg,
            values: values,
        }
    }

    /// Get a log record message
    pub fn msg(&self) -> fmt::Arguments {
        self.msg
    }

    /// Get record logging level
    pub fn level(&self) -> Level {
        self.meta.level
    }

    /// Get line number
    pub fn line(&self) -> u32 {
        self.meta.line
    }

    /// Get error column
    pub fn column(&self) -> u32 {
        self.meta.column
    }

    /// Get file path
    pub fn file(&self) -> &'static str {
        self.meta.file
    }

    /// Get target
    ///
    /// Mostly for backward compatibility with `log`
    pub fn target(&self) -> &str {
        self.meta.target
    }

    /// Get module
    pub fn module(&self) -> &'static str {
        self.meta.module
    }

    /// Get function
    pub fn function(&self) -> &'static str {
        self.meta.function
    }

    /// Get Record's key-value pairs
    pub fn values(&self) -> &'a [BorrowedKeyValue<'a>] {
        self.values
    }
}
