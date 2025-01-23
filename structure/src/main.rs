mod foo;

mod prelude {
    use crate::foo::{Mystruct, Another};
}

use crate::prelude::*;

fn main() {
   let _ms = Mystruct{};

   let _twostruct = Another{};
}
