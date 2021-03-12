trait Job {
    fn set_name(&mut self, name: String);
    fn say_work(&self);
}

struct Student {
    name: String,
}

struct Teacher {
    name: String,
}

impl Job for Student {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn say_work(&self) {
        println!("{} is a student", self.name);
    }
}

impl Job for Teacher {
    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn say_work(&self) {
        println!("{} is a teacher", self.name);
    }
}

//转为enum
enum Ejob {
    Student(Student),
    Teacher(Teacher),
}

impl Job for Ejob {
    fn set_name(&mut self, name: String) {
        match self {
            Ejob::Student(s) => s.name = name,
            Ejob::Teacher(t) => t.name = name,
        }
    }

    fn say_work(&self) {
        match self {
            Ejob::Student(s) => println!("{} is a student", s.name),
            Ejob::Teacher(t) => println!("{} is a teacher", t.name),
        }
    }
}

fn main() {
    //版本1
    let v1: Vec<Box<Job>> = vec![Box::new(Student{name: "tom".into()}), Box::new(Teacher{name: "lily".into()})];

    //版本2
    let v2: Vec<Ejob> = vec![Ejob::Student(Student{name: "tom".into()}), Ejob::Teacher(Teacher{name: "lily".into()})];
}
