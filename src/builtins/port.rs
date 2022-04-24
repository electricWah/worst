
use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use crate::base::*;
use crate::interpreter::{Builder, Handle};

// currently no OutputPort/InputPort split since e.g. StringBuffer is both
#[derive(Clone)]
enum Port {
    Stdin,
    Stdout,
    Stderr,
    StringBuffer(Rc<RefCell<VecDeque<char>>>),
    // Write(Rc<RefCell<Box<dyn std::io::Write>>>, String),
}

impl PartialEq for Port {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Port::Stdin, Port::Stdin) => true,
            (Port::Stdout, Port::Stdout) => true,
            (Port::Stderr, Port::Stderr) => true,
            // (Port::Write(a, _),
            //  Port::Write(b, _)) => Rc::ptr_eq(a, b),
            (Port::StringBuffer(a),
             Port::StringBuffer(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
impl Eq for Port {}

impl std::fmt::Debug for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Port::Stdin => write!(f, "<stdin>"),
            Port::Stdout => write!(f, "<stdout>"),
            Port::Stderr => write!(f, "<stderr>"),
            // Port::Write(_, name) => write!(f, "<{}>", name),
            Port::StringBuffer(s) =>
                write!(f, "<\"{}\"...>",
                       s.borrow().iter().take(8).map(char::clone).collect::<String>()),
        }
    }
}

impl ImplValue for Port {}

impl Port {
    fn new_string_buffer() -> Self {
        Port::StringBuffer(Rc::new(RefCell::new(VecDeque::default())))
    }

    fn write_string(&mut self, v: impl AsRef<str>) -> Result<(), Val> {
        match self {
            Port::Stdin => Err(Val::from("not writable"))?,
            Port::Stdout => {
                std::io::stdout().write(v.as_ref().as_bytes()).map_err(|e| {
                    format!("{:?}", e.kind())
                })?;
            },
            Port::Stderr => {
                std::io::stdout().write(v.as_ref().as_bytes()).map_err(|e| {
                    format!("{:?}", e.kind())
                })?;
            },
            // Port::Write(p, _) => {
            //     p.borrow_mut().write(v.as_ref().as_bytes()).map_err(|e| {
            //         format!("{:?}", e.kind()).into()
            //     })?;
            // },
            Port::StringBuffer(s) => {
                let mut b = s.borrow_mut();
                for c in v.as_ref().chars() {
                    b.push_back(c);
                }
            },
        }
        Ok(())
    }

    // fn peekable(&self) -> bool {
    //     match self {
    //         Port::StringBuffer(_) => true,
    //         _ => false,
    //     }
    // }

    fn peek_char(&self) -> Result<Option<char>, Val> {
        match self {
            Port::StringBuffer(s) => Ok(s.borrow().front().map(char::clone)),
            _ => Err("not peekable".into()),
        }
    }

    fn read_char(&mut self) -> Result<Option<char>, Val> {
        match self {
            Port::StringBuffer(s) => Ok(s.borrow_mut().pop_front()),
            _ => Err("TODO port::read_char".into()),
        }
    }

    fn read_all(&mut self) -> Result<String, Val> {
        match self {
            Port::StringBuffer(s) => Ok(s.borrow_mut().iter().collect::<String>()),
            _ => Err("TODO port::read_all".into()),
        }
    }

    /// includes final newline character (unless it hits EOF)
    fn read_line(&mut self) -> Result<String, Val> {
        match self {
            Port::Stdin => {
                let mut l = String::new();
                std::io::stdin().read_line(&mut l)
                    .map_err(|e| format!("{:?}", e.kind()))?;
                Ok(l)
            },
            Port::StringBuffer(s) => {
                match s.borrow().iter().position(|c| c == &'\n') {
                    Some(n) => Ok(s.borrow_mut().drain(..n).collect()),
                    None => Ok(s.borrow_mut().drain(..).collect()),
                }
            },
            _ => Err("not read-line-able".into()),
        }
    }

}

pub fn install(mut i: Builder) -> Builder {
    i.define("new-string-port", |mut i: Handle| async move {
        i.stack_push(Port::new_string_buffer()).await;
    });
    i.define("current-output-port", |mut i: Handle| async move {
        i.stack_push(Port::Stdout).await;
    });
    i.define("current-error-port", |mut i: Handle| async move {
        i.stack_push(Port::Stderr).await;
    });
    i.define("port-write-string", |mut i: Handle| async move {
        let s = i.stack_pop::<String>().await;
        let mut p = i.stack_pop::<Port>().await;
        // TODO retry with while?
        if let Err(e) = p.write_string(s) {
            i.stack_push(e).await;
            i.pause().await;
        }
        i.stack_push(p).await;
    });
    i.define("value->string", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        i.stack_push(format!("{:?}", v)).await;
    });

    i.define("current-input-port", |mut i: Handle| async move {
        i.stack_push(Port::Stdin).await;
    });

    i.define("port-read-line", |mut i: Handle| async move {
        let mut p = i.stack_pop::<Port>().await;
        let l = p.read_line();
        i.stack_push(p).await;
        match l {
            Ok(l) => i.stack_push(l).await,
            Err(v) => {
                i.stack_push(v).await;
                i.pause().await;
            },
        }
    });

    // i.define("port-peekable", |mut i: Handle| async move {
    //     let p = i.stack_pop::<Port>().await;
    //     let peekable = p.peekable();
    //     i.stack_push(p).await;
    //     i.stack_push(peekable).await;
    // })
    i.define("port-peek-char", |mut i: Handle| async move {
        let p = i.stack_pop::<Port>().await;
        let ch = p.peek_char();
        i.stack_push(p).await;
        match ch {
            Ok(Some(v)) => i.stack_push(String::from(v)).await,
            Ok(None) => i.stack_push(false).await,
            Err(v) => {
                i.stack_push(v).await;
                i.pause().await;
            },
        }
    });

    i.define("port-read-char", |mut i: Handle| async move {
        let mut p = i.stack_pop::<Port>().await;
        let ch = p.read_char();
        i.stack_push(p).await;
        match ch {
            Ok(Some(v)) => i.stack_push(String::from(v)).await,
            Ok(None) => i.stack_push(false).await,
            Err(v) => {
                i.stack_push(v).await;
                i.pause().await;
            },
        }
    });

    i.define("port-read-all", |mut i: Handle| async move {
        let mut p = i.stack_pop::<Port>().await;
        let s = p.read_all();
        i.stack_push(p).await;
        match s {
            Ok(s) => i.stack_push(s).await,
            Err(v) => {
                i.stack_push(v).await;
                i.pause().await;
            },
        }
    });

    i
}

