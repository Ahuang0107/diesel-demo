use diesel::prelude::*;

use diesel_demo::*;

fn main() {
    use self::models::*;
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();

    let _ = create_post(connection, "test post", Some("test post body"));

    let results = posts
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("-----------");
        println!("{}", post.title);
        println!("{:?}", post.body);
        println!("-----------");
    }
}
