use std::error::Error;
use std::env;
use std::collections::HashMap;
use std::process::Command;
use serde::Deserialize;
use csv::ReaderBuilder;

#[derive(Debug, Deserialize)]
struct ServerInfo {
    server_name: String,
    user_name: String,
    ip_name: String,
    password: String,
    port: String,
}

fn ssh_to(server_info: &ServerInfo) {
    let user_name: &str = server_info.user_name.as_str();
    let ip_name: &str = server_info.ip_name.as_str();
    let password: &str = server_info.password.as_str();
    let port: &str = server_info.port.as_str();
    // status(), output(), spawn()三者的区别？？？
    // sshpass -p lsl001 ssh -p 22 slliu@192.168.1.102
    Command::new("sshpass")
        .arg("-p")
        .arg(password)
        .arg("ssh")
        .arg("-p")
        .arg(port)
        .arg(format!("{}@{}", user_name, ip_name))
        .status()
        .expect("服务器登录失败！");
}

fn main()-> Result<(), Box<dyn Error>>{
    // 构建自动登录命令行工具
    // 1.实现通过命令` ccc server_name `自动登录远程服务器
        // ssh slliu@192.168.1.102   -p lsl001
        // ccc slliu102 即可自动登录
    // 2.实行从配置文件中读取服务器地址和用户密码信息
    // 3.实现对密码进行自定义加密
    // sshpass -p slliu001 ssh slliu@192.168.1.103
    // sshpass -p lsl001 ssh -p 22 slliu@192.168.1.102
    let args: Vec<String> = env::args().collect();
    // 参数列表的长度
    // let args_len = args.len();
    let server_name: &str = &args[1];
    // 实现存储服务器信息的字典类型的结果
    let mut server_name_dict: HashMap<String, ServerInfo> = HashMap::new();
    let path: &str = "server_info.txt";
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_path(path)?;
    let mut rdr_iter = rdr.deserialize();
    if let Some(result) = rdr_iter.next() {
        let record: ServerInfo = result?;
        server_name_dict.insert(record.server_name.clone(), record);
    }


    // server_name_dict.insert("slliu102", ServerInfo{
    //     server_name: String::from("slliu102"),
    //     user_name: String::from("slliu"),
    //     ip_name: String::from("192.168.1.102"),
    //     password: String::from("lsl001"),
    //     port: String::from("22"),
    // });

    let server_info_option: Option<&ServerInfo> = server_name_dict.get(server_name);
    match server_info_option {
        None => println!("此服务器名称不存在。"),
        Some(server_info) => ssh_to(server_info),
    }
    Ok(())
}
