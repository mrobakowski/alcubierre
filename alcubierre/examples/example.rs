use alcubierre::{
    get,
    warp::{
        reply::json,
        Reply,
    },
};
use serde::Serialize;

mod v1 {
    use super::*;

    #[get]
    fn foo() -> &'static str {
        "bar"
    }

    #[derive(Serialize)]
    struct Baz {
        qux: String,
        important_coeff: i64
    }

    #[get]
    fn quux(qux: String) -> impl Reply {
        json(&Baz { qux, important_coeff: 69 })
    }
}

mod v2 {
    mod special {
        use super::super::*;
        #[get]
        fn secret() -> String {
            "nuke codes".into()
        }
    }
}

fn main() {
    println!("Server starting at http://localhost:8080/");
    println!("Press Ctrl+C to exit...");

    // serves everything at /<crate name>/...
    // alcubierre::engage(([0, 0, 0, 0], 8080));

    // serves only v1 at /
    // alcubierre::engage_rooted(concat!(module_path!(), "v1"), ([0, 0, 0, 0], 8080));

    // this will serve the following endpoints:
    //   GET /v1/foo
    //   GET /v1/quux/<qux>
    //   GET /v2/special/secret
    alcubierre::engage_rooted(module_path!(), ([0, 0, 0, 0], 8080)); // serves everything at /
}