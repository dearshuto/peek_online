use clap::Parser;
use warp::Filter;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short('i'), long("index-html-path"), required = true)]
    index_html_path: String,

    #[arg(short('m'), long("media-path"), required = true)]
    media_path: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // index.html を返す
    let index_html_path = args.index_html_path.clone();
    let get_index = warp::get().and(warp::path::end()).map(move || {
        let index_html = std::fs::read_to_string(&index_html_path).unwrap();
        warp::reply::html(index_html)
    });

    // media のパスを設定
    let media_path = warp::path("media").and(warp::fs::dir(args.media_path));

    // サーバーを起動
    let filter = get_index.or(media_path);
    warp::serve(filter).run(([127, 0, 0, 1], 3030)).await;
}
