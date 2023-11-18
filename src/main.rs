use std::collections::HashMap;
use std::io;
use std::process::exit;
use std::io:: Write;
#[derive(Default)]
struct Database {
    all_students: HashMap<u32, student_list>,
}

struct student_list {
    id: u32,
    name: String,
    age: u32,
    school: String,
}

impl Database {
    fn add_student(&mut self) {
        let mut input = String::new();
        print!("1.学号：");
        io::stdout().flush().unwrap();
        input.clear();
        let addid = match std::io::stdin().read_line(&mut input) {
            Ok(id) => input.to_string().trim().parse::<u32>().unwrap(),
            Err(e) => {
                println!("输入错误，请重新输入");
                return;
            }
        };
        print!("2.年龄：");
        io::stdout().flush().unwrap();
        input.clear();
        let addage = match std::io::stdin().read_line(&mut input) {
            Ok(age) => input.to_string().trim().parse::<u32>().unwrap(),
            Err(e) => {
                println!("输入错误，请重新输入");
                return;
            }
        };
        print!("3.姓名：");
        io::stdout().flush().unwrap();
        input.clear();
        let addname = match std::io::stdin().read_line(&mut input) {
            Ok(name) => input.to_string().trim().to_string(),
            Err(e) => {
                println!("输入错误，请重新输入");
                return;
            }
        };
        print!("4.学校：");
        io::stdout().flush().unwrap();
        input.clear();
        let addschool = match std::io::stdin().read_line(&mut input) {
            Ok(school) => input.to_string().trim().to_string(),
            Err(e) => {
                println!("输入错误，请重新输入");
                return;
            }
        };

        self.all_students.insert(addid, student_list { id: addid, name: addname.clone(), age: addage , school: addschool.clone()});
        println!("录入成功,该学生信息为:\n");
        println!("学号：{} 姓名：{} 年龄：{} 学校：{}", addid, addname, addage, addschool);
    }

    fn modify_student(&mut self) {
        let mut input = String::new();
        print!("请输入要修改学生的学号：");
        io::stdout().flush().unwrap();
        input.clear();
        let orid = match std::io::stdin().read_line(&mut input) {
            Ok(id) => input.to_string().trim().parse::<u32>().unwrap(),
            Err(e) => {
                println!("输入错误，请重新输入");
                return;
            }
        };

        // 获取并打印原始学生信息
        if let Some(stu) = self.all_students.get(&orid) {
            println!("原始学生信息：");
            println!("学号：{} 姓名：{} 年龄：{} 学校：{}", stu.id, stu.name, stu.age, stu.school);
        } else {
            println!("没有找到学号为 {} 的学生", orid);
            return;
        }

        print!("1.学号：");
        io::stdout().flush().unwrap();
        input.clear();
        let modid = match std::io::stdin().read_line(&mut input) {
            Ok(id) => input.to_string().trim().parse::<u32>().unwrap(),
            Err(e) => {
                println!("输入错误，请重新输入");
                return;
            }
        };
        print!("2.年龄：");
        io::stdout().flush().unwrap();
        input.clear();
        let modage = match std::io::stdin().read_line(&mut input) {
            Ok(age) => input.to_string().trim().parse::<u32>().unwrap(),
            Err(e) => {
                println!("输入错误，请重新输入");
                return;
            }
        };
        print!("3.姓名：");
        io::stdout().flush().unwrap();
        input.clear();
        let modname = match std::io::stdin().read_line(&mut input) {
            Ok(name) => input.to_string().trim().to_string(),
            Err(e) => {
                println!("输入错误，请重新输入");
                return;
            }
        };
        print!("4.学校：");
        io::stdout().flush().unwrap();
        input.clear();
        let modschool = match std::io::stdin().read_line(&mut input) {
            Ok(school) => input.to_string().trim().to_string(),
            Err(e) => {
                println!("输入错误，请重新输入");
                return;
            }
        };

        let mut stuinfo = student_list {
            name: modname.clone(),
            school: modschool.clone(),
            id: modid,
            age: modage,
        };

        self.all_students.insert(modid, stuinfo);
        println!("信息修改成功,修改后的信息为：\n");
        println!("学号：{} 姓名：{} 年龄：{} 学校：{}", modid, modname, modage, modschool);
    }

    fn delete_student(&mut self) {
        let mut input = String::new();
        print!("请输入要删除学生的学号：");
        io::stdout().flush().unwrap();
        input.clear();
        let delid = match std::io::stdin().read_line(&mut input) {
            Ok(id) => input.to_string().trim().parse::<u32>().unwrap(),
            Err(e) => {
                println!("输入错误，请重新输入");
                return;
            }
        };

        let mut stuinfo = self.all_students.remove(&delid);
        match stuinfo {
            Some(info) => {
                println!("已删除学生信息：\n");
                println!("学号：{} 姓名：{} 年龄：{} 学校：{}", info.id, info.name, info.age, info.school);
            },
            None => println!("没有学生信息")
        }
    }

    fn show_info(&self) {
        for (id, stuinfo) in self.all_students.iter() {
            println!("学号：{} 姓名：{} 年龄：{} 学校：{}", id, stuinfo.name, stuinfo.age, stuinfo.school);
        }
    }
}

fn main() {
    let mut student_data = Database::default();
    loop {
        println!("菜单：\n   1. 添加学生信息\n");
        println!("   2. 修改学生信息\n");
        println!("   3. 删除学生信息\n");
        println!("   4. 显示全部学生信息\n");
        println!("   5. 退出\n");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse().unwrap();
        match choice {
            1 => student_data.add_student(),
            2 => student_data.modify_student(),
            3 => student_data.delete_student(),
            4 => student_data.show_info(),
            5 => break,
            _ => println!("请输入 1~5 之间的数字"),
        }
    }
}