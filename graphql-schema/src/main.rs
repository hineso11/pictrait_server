use graphql::new_schema;

fn main() {
    let schema = new_schema();
    println!("{}", schema.sdl());
}
