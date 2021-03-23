use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point{x:1,y:2};
    let ser = serde_json::to_string(&point).unwrap();
    println!("ser = {}", ser);

    let deser = serde_json::from_str::<Point>(&ser).unwrap();
    println!("deser = {:?}", deser);
}

mod tests {
    use serde::{Serialize, Deserialize};
    #[derive(Debug, Serialize, Deserialize)]
    struct Params(i32);

    #[derive(Serialize, Deserialize, Debug)] //{"Request":{"id":"33","method":"get","params":33}}
    //#[serde(tag = "type")] //{"type":"Request","id":"33","method":"get","params":33}
    //#[serde(tag = "type", content = "c")]  //{"type":"Request","c":{"id":"33","method":"get","params":33}}
    //#[serde(untagged)] //{"id":"33","method":"get","params":33}
    enum Message {
        Request {
            id: String,
            method: String,
            params: Params,
        },
        Response {
            id: String,
            result: Params,
        }
    }

    #[test]
    fn test_enum() {
        let message = Message::Request {id: "33".to_string(), method: "get".to_string(),params: Params(33)};
        let m = serde_json::to_string(&message).unwrap();
        println!("{}",m);
    }

    #[derive(Deserialize, Debug)]
    struct Req {
        #[serde(default = "default_resource")]
        resource: String,
        #[serde(default)]
        timeout: Timeout,
        #[serde(default = "Priority::lowest")]
        priority: Priority,
    }

    fn default_resource() -> String {
        "/".to_string()
    }

    #[derive(Deserialize, Debug)]
    struct Timeout(u32);
    impl Default for Timeout {
        fn default() ->Self {
            Timeout(30)
        }
    }

    #[derive(Deserialize, Debug)]
    enum Priority {
        ExtraHigh, High, Normal, Low, ExtraLow,
    }

    impl Priority {
        fn lowest() -> Self {
            Priority::ExtraLow
        }
    }

    #[test]
    fn test_default() {
        let json = r#"
            [
                {
                    "resource": "/users"
                },
                {
                    "timeout": 5,
                    "priority": "High"
                }
            ]
        "#;

        let request:Vec<Req> = serde_json::from_str(json).unwrap();
        println!("{:?}", request[0]);
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Pagination {
        limit: u64,
        offset: u64,
        total: u64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Users {
        users: Vec<User>,
                             //{"users":[{"name":"zhangsan","age":22},{"name":"lisi","age":33}],"pagination":{"limit":22,"offset":22,"total":11}}
        #[serde(flatten)]  //{"users":[{"name":"zhangsan","age":22},{"name":"lisi","age":33}],"limit":22,"offset":22,"total":11}
        pagination: Pagination,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct User {
        name: String,
        age: u8,
    }

    #[test]
    fn test_flatten() {
        let user = Users{
            users: vec![User{name: "zhangsan".to_string(), age: 22}, User{name: "lisi".to_string(), age: 33}],
            pagination: Pagination{
                limit: 22,
                offset: 22,
                total: 11,
            }
        };
        let u = serde_json::to_string(&user).unwrap();
        println!("{}", u);
    }
}
