use request::Request;
use response::Response;

// the usage of + Send is weird here because what we really want is + Static
// but that's not possible as of today. We have to use + Send for now.
pub trait MiddlewareHandler: Clone + Send {
    fn invoke (&self, req: &mut Request, res: &mut Response) -> bool {
        true
    }

    // we need this because otherwise clone() would be ambiguous
    fn clone_box(&self) -> Box<MiddlewareHandler + Send> { 
        box self.clone() as Box<MiddlewareHandler + Send> 
    }
}

impl MiddlewareHandler for fn (req: &Request, res: &mut Response) -> bool {
    fn invoke(&self, req: &mut Request, res: &mut Response) -> bool{
        (*self)(req, res)
    }
}

impl Clone for Box<MiddlewareHandler + Send> {
    fn clone(&self) -> Box<MiddlewareHandler + Send> { 
        self.clone_box() 
    }
}


#[deriving(Clone)]
pub struct Middleware {
    handlers: Vec<Box<MiddlewareHandler + Send>>
}

impl Middleware {
    pub fn add<T: MiddlewareHandler> (&mut self, handler: T) {
        self.handlers.push(box handler);
    }

    pub fn invoke (&self, req: &mut Request, res: &mut Response) {
        self.handlers.iter().advance(|handler| (*handler).invoke(req, res));
    }

    pub fn new () -> Middleware {
        Middleware{
            handlers: Vec::new()
        }
    }
}