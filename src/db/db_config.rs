use std::{env, fs};

/*
 * Redis 配置
 * 
 * @param bind 地址
 * @param port 端口
 * @param password 密码
 * @param maxclients 客户端上限
 * @param databases 初始化数据库 
 * @param appendfilename 命令持久化文件
 * @param appendonly 是否开启持久化
 */
pub struct RedisConfig {
    pub bind: String,
    pub port: u16,
    pub password: Option<String>,
    pub databases: usize,
    pub appendfilename: Option<String>,
    pub appendonly: bool,
    pub hz: u64,
    pub appendfsync: Option<String>,
    pub maxclients: usize
}

impl Default for RedisConfig {
    fn default() -> Self {

        let filename = "appendonly.aof";
        let mut port = get_port_or(6379);
        let mut bind = get_bind_or(String::from("127.0.0.1"));
        let mut databases = get_databases_or(16);
        let mut password = get_password_or(None);
        let mut appendonly = get_appendonly_or(false);
        let mut appendfilename = get_appendfilename_or(Some(filename.to_string()));
        let mut hz = get_hz_or(10);
        let mut maxclients = get_maxclients_or(0);
        let mut appendfsync = get_appendfsync_or(None);
        let config_path = get_config_path_or(None);
        
        if let Some(config) = config_path {
            if let Ok(content) = fs::read_to_string(config) {
                for line in content.lines() {
                    if let Some((key, value)) = parse_config_line(line) {
                        print!("{}",key.as_str());
                        match key.as_str() {
                            "port" => port = value.parse().unwrap_or(port),
                            "bind" => bind = value.to_string(),
                            "password" => password = Some(value.to_string()),
                            "databases" => databases = value.parse().unwrap_or(databases),
                            "maxclients" => maxclients = value.parse().unwrap_or(maxclients),
                            "appendonly" => appendonly = value.parse().unwrap_or(appendonly),
                            "appendfilename" => appendfilename = Some(value.to_string()),
                            "appendfsync" => appendfsync = Some(value.to_string()),
                            "hz" => hz = value.parse().unwrap_or(hz),
                            _ => {}  // Ignore unknown config keys
                        }
                    }
                }
            } else {
                eprintln!("Error reading the config file");
            }
        }

        Self {
            port,
            password,
            appendfsync,
            databases,
            appendfilename,
            appendonly,
            maxclients,
            bind,
            hz
        }
    }
}

fn parse_config_line(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.splitn(2, '=').map(|s| s.trim()).collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

/*
 * 获取 port 参数
 *
 * @param default 默认值 false
 */
fn get_appendonly_or(default: bool) -> bool {
    let mut args = env::args().skip_while(|arg| arg != "--appendonly").take(2);
    if args.next().is_none() {
        return default;
    }

    if let Some(arg) = args.next() {
        return arg.parse().expect("'--appendonly' must have a value");
    } else {
        return default;
    }
}

/*
 * 获取 port 参数
 *
 * @param default 默认端口（6379）
 */
fn get_port_or(default: u16) -> u16 {
    let mut args = env::args().skip_while(|arg| arg != "--port").take(2);
    if args.next().is_none() {
        return default;
    }

    if let Some(arg) = args.next() {
        return arg.parse().expect("'--port' must have a value");
    } else {
        return default;
    }
}

fn get_hz_or(default: u64) -> u64 {
    let mut args = env::args().skip_while(|arg| arg != "--hz").take(2);
    if args.next().is_none() {
        return default;
    }

    if let Some(arg) = args.next() {
        return arg.parse().expect("'--hz' must have a value");
    } else {
        return default;
    }
}

/*
 * 获取 maxclients 参数
 *
 * @param default 默认值 1000
 */
fn get_maxclients_or(default: usize) -> usize {
    let mut args = env::args().skip_while(|arg| arg != "--maxclients").take(2);
    if args.next().is_none() {
        return default;
    }

    if let Some(arg) = args.next() {
        return arg.parse().expect("'--maxclients' must have a value");
    } else {
        return default;
    }
}

/*
 * 获取 databases 参数
 *
 * @param default 默认数量（16）
 */
fn get_databases_or(default: usize) -> usize {
    let mut args = env::args().skip_while(|arg| arg != "--databases").take(2);
    if args.next().is_none() {
        return default;
    }

    if let Some(arg) = args.next() {
        return arg.parse().expect("'--databases' must have a value");
    } else {
        return default;
    }
}

/*
 * 获取 password 参数
 *
 * @param default_password 默认密码（None）
 */
fn get_password_or(default_password: Option<String>) -> Option<String> {
    let mut args = env::args().skip_while(|arg| arg != "--password").take(2);
    if args.next().is_none() {
        return default_password;
    }

    if let Some(arg) = args.next() {
        return Some(arg);
    } else {
        return default_password;
    }
}

/*
 * 获取 password 参数
 *
 * @param default_password 默认密码（None）
 */
fn get_bind_or(default_bind: String) -> String {
    let mut args = env::args().skip_while(|arg| arg != "--bind").take(2);
    if args.next().is_none() {
        return default_bind;
    }

    if let Some(arg) = args.next() {
        return arg;
    } else {
        return default_bind;
    }
}

/*
 * 获取 appendfilename 参数
 */
fn get_appendfilename_or(default_appendfilename: Option<String>) -> Option<String> {
    let mut args = env::args().skip_while(|arg| arg != "--appendfilename").take(2);
    if args.next().is_none() {
        return default_appendfilename;
    }

    if let Some(arg) = args.next() {
        return Some(arg);
    } else {
        return default_appendfilename;
    }
}

/*
 * 获取 appendfsync 参数
 */
fn get_appendfsync_or(default_appendfsync: Option<String>) -> Option<String> {
    let mut args = env::args().skip_while(|arg| arg != "--appendfsync").take(2);
    if args.next().is_none() {
        return default_appendfsync;
    }

    if let Some(arg) = args.next() {
        return Some(arg);
    } else {
        return default_appendfsync;
    }
}

fn get_config_path_or(default_config_path: Option<String>) -> Option<String> {
    let mut args = env::args();
    if let Some(config_path) = args.nth(1) {
        return Some(config_path);
    } else {
        return default_config_path;
    }
}