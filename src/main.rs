
#[derive(Debug)]
enum IP {
    V4(String, i32),
    V6{name: String, pl: f64}
}

#[derive(Debug)]
struct Network {
    name: String,
    ip: IP,
}

fn main(){
    let web_2: Network = Network {
        name: String::from("Web 2.0"),
        ip: IP::V4(String::from("Web 2.0"), 3),
    };
    println!("{:#?}", web_2);

    let web_3:Network = Network {
        name: String::from("Web 3.0"),
        ip: IP::V6{name: String::from("Algeria"), pl: 12.36}
    };

    println!("{:#?}", web_3);

}

