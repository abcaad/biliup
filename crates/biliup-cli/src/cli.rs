use biliup::uploader::bilibili::{Studio, Vid};
use biliup::uploader::util::SubmitOption;
use clap::{Parser, Subcommand};

use crate::UploadLine;
use std::path::PathBuf;

/// 扩展路径中的 ~ 为用户主目录
pub fn expand_path(path: PathBuf) -> PathBuf {
    if let Some(path_str) = path.to_str() {
        let expanded = shellexpand::tilde(path_str);
        return PathBuf::from(expanded.as_ref());
    }
    path
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    // /// Turn debugging information on
    // #[clap(short, long, parse(from_occurrences))]
    // debug: usize,
    #[clap(subcommand)]
    pub command: Commands,

    /// 配置代理
    #[arg(short, long, default_value = None)]
    pub proxy: Option<String>,

    /// 登录信息文件
    #[arg(short, long, default_value = "cookies.json")]
    pub user_cookie: PathBuf,

    // #[arg(long, default_value = "sqlx=debug,tower_http=debug,info")]
    #[arg(long, default_value = "tower_http=debug,info")]
    pub rust_log: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 登录B站并保存登录信息
    Login,
    /// 手动验证并刷新登录信息
    Renew,
    /// 上传视频
    Upload {
        /// 提交接口
        #[arg(long)]
        submit: Option<SubmitOption>,

        // Optional name to operate on
        // name: Option<String>,
        /// 需要上传的视频路径,若指定配置文件投稿不需要此参数
        #[arg()]
        video_path: Vec<PathBuf>,

        /// Sets a custom config file
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,

        /// 选择上传线路
        #[arg(short, long, value_enum)]
        line: Option<UploadLine>,

        /// 单视频文件最大并发数
        #[arg(long, default_value = "3")]
        limit: usize,

        #[command(flatten)]
        studio: Studio,
        // #[arg(required = false, last = true, default_value = "client")]
        // submit: Option<String>,
    },
    /// 是否要对某稿件追加视频
    Append {
        /// 提交接口
        #[arg(long)]
        submit: Option<SubmitOption>,

        // Optional name to operate on
        // name: Option<String>,
        /// vid为稿件 av 或 bv 号
        #[arg(short, long)]
        vid: Vid,
        /// 需要上传的视频路径,若指定配置文件投稿不需要此参数
        #[arg()]
        video_path: Vec<PathBuf>,

        /// 选择上传线路
        #[arg(short, long, value_enum)]
        line: Option<UploadLine>,

        /// 单视频文件最大并发数
        #[arg(long, default_value = "3")]
        limit: usize,

        /// 新视频添加到稿件最前面
        #[arg(long, default_value = "false")]
        insert: bool,

        /// 不将新分P添加到稿件中
        #[arg(long, default_value = "false")]
        skip_binding: bool,

        // #[command(flatten)]
        // studio: Studio,
    },
    /// 将分P转移到其他稿件  不可用  仅能删除
    Transfer {
        /// 提交接口
        #[arg(long)]
        submit: Option<SubmitOption>,

        // Optional name to operate on
        // name: Option<String>,
        /// vid为稿件 av 或 bv 号  原分P所在稿件
        #[arg(short, long)]
        from_vid: Vid,

        /// vid为稿件 av 或 bv 号  分P将转移到的稿件
        #[arg(short, long)]
        to_vid: Vid,

        /// 将要转移的分P序号 从1开始
        #[arg(short, long)]
        index: usize,

        /// 新视频添加到稿件最前面
        #[arg(long, default_value = "false")]
        insert: bool,

        // #[command(flatten)]
        // studio: Studio,
    },
    /// 将分P添加到其他稿件  输入 json 或 title filename desc
    AddPart {
        /// 提交接口
        #[arg(long)]
        submit: Option<SubmitOption>,

        // Optional name to operate on
        // name: Option<String>,
        /// vid为稿件 av 或 bv 号
        #[arg(short, long)]
        vid: Vid,
        
        /// Video 对象的 title
        #[arg(short, long)]
        title: Option<String>,

        /// Video 对象的 filename
        #[arg(short, long)]
        filename: Option<String>,

        /// Video 对象的 desc
        #[arg(short, long)]
        desc: Option<String>,

        /// Video 对象的 json 例如 '{"title":"","filename":"","desc":""}'
        #[arg(long)]
        video_json: Option<String>,

        /// 将要添加到的分P序号 从1开始   为 0 时添加到末尾
        #[arg(long, default_value = "1")]
        index: usize,

        // #[command(flatten)]
        // studio: Studio,
    },
    /// 打印视频详情
    Show {
        /// vid为稿件 av 或 bv 号
        // #[clap()]
        vid: Vid,
    },
    /// 输出flv元数据
    DumpFlv {
        #[arg()]
        file_name: PathBuf,
    },
    /// 下载视频
    Download {
        url: String,

        /// Output filename template. e.p. "./video/%Y-%m-%dT%H_%M_%S{title}"
        #[arg(short, long, default_value = "{title}")]
        output: String,

        /// 按照大小分割视频
        #[arg(long, value_parser = human_size)]
        split_size: Option<u64>,

        /// 按照时间分割视频
        #[arg(long)]
        split_time: Option<humantime::Duration>,
    },
    /// 启动web服务，默认端口19159
    Server {
        /// Specify bind address
        #[arg(short, long, default_value = "0.0.0.0")]
        bind: String,

        /// Port to use
        #[arg(short, long, default_value = "19159")]
        port: u16,

        /// 开启登录密码认证
        #[arg(long, default_value = "false")]
        auth: bool,
    },
    /// 列出所有已上传的视频
    List {
        /// 只包含进行中的视频
        #[arg(long)]
        is_pubing: bool,

        /// 只包含已通过的视频
        #[arg(long)]
        pubed: bool,

        /// 只包含未通过的视频
        #[arg(long)]
        not_pubed: bool,

        /// 从第几页开始获取
        #[arg(short, long, default_value = "1")]
        from_page: u32,

        /// 最大获取页数
        #[arg(short, long)]
        max_pages: Option<u32>,
    },
}

fn human_size(s: &str) -> Result<u64, String> {
    let ret = match s.as_bytes() {
        [init @ .., b'K'] => parse_u8(init)? * 1000.0,
        [init @ .., b'M'] => parse_u8(init)? * 1000.0 * 1000.0,
        [init @ .., b'G'] => parse_u8(init)? * 1000.0 * 1000.0 * 1000.0,
        init => parse_u8(init)?,
    };
    Ok(ret as u64)
}

fn parse_u8(string: &[u8]) -> Result<f64, String> {
    let string = String::from_utf8_lossy(string);
    string
        .parse()
        .map_err(|e| format!("{string} is not ascii digit. {:?}", e))
}
