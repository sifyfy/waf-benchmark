extern crate sapper;
extern crate env_logger;
#[macro_use]
extern crate log;

#[derive(Clone)]
struct App;

// impl sapper::SAppWrapper for App {
//     fn before(&self, req: &mut sapper::Request) -> sapper::Result<()> {
//         println!("in SAppWrapper before.");
//         Ok(())
//     }
//
//     fn after(&self, req: &sapper::Request, res: &mut sapper::Response) -> sapper::Result<()> {
//         println!("in SAppWrapper after.");
//         Ok(());
//     }
// }

#[derive(Clone)]
struct HelloWorld;

impl HelloWorld {
    fn index(req: &mut sapper::Request) -> sapper::Result<sapper::Response> {
        let mut response = sapper::Response::new();
        response.write_body("Hello World!".to_string());
        Ok(response)
    }
}

impl sapper::SModule for HelloWorld {
    fn before(&self, req: &mut sapper::Request) -> sapper::Result<()> {
        // println!("before HelloWorld.");
        Ok(())
    }

    fn after(&self, req: &sapper::Request, res: &mut sapper::Response) -> sapper::Result<()> {
        // println!("after HelloWorld.");
        Ok(())
    }

    fn router(&self, router: &mut sapper::SRouter) -> sapper::Result<()> {
        router.get("/", HelloWorld::index);
        Ok(())
    }
}

fn main() {
    env_logger::init().unwrap();

    let mut sapp = sapper::SApp::new();
    sapp.address("127.0.0.1")
        .port(3000)
        // .with_wrapper(Box::new(App))
        .add_module(Box::new(HelloWorld));

    println!("Listening on http://127.0.0.1:3000");
    sapp.run();
}
