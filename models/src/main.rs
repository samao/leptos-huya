use models::User;

fn main() {
    let u = User {
        user_name: "王二小".to_string(),
        id: 0,
        avatar: "pic".to_string(),
        phone: "123".to_string(),
        password: "OOOO".to_string(),
    };

    let time = chrono::Local::now().timestamp();

    println!("{:?}", User::compute_token(&u, time));
}
