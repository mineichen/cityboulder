use async_std::task;
use scraper::{Html, Selector};
use chrono::Utc;


async fn fetch(url: String) -> surf::Result<cityboulder_data::VisitorLookup> {
    println!("Fetch data from {}", url);
    let mut res = surf::get(url).await?;
    let body_str = res.body_string().await?;
    //println!("{}", body_str);

    let fragment = Html::parse_fragment(&body_str);
    let selector = Selector::parse("div").unwrap();
    let div = fragment.select(&selector).next().expect("Couldn't find div");
    let c = div.inner_html().trim().parse::<i32>().expect("expected a number in div");

    Ok(cityboulder_data::VisitorLookup {
        guest_count: c,
        recorded_at: Utc::now().naive_utc()
    })
    /*
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("stats.csv")
        .unwrap();
    writeln!(file, "{};{}", lookup.recorded_at, lookup.guest_count).expect("Couldn't write to file");
    */
}

fn main() {
    println!("Run crawling");

    let mut args = std::env::args().skip(1);
    let url = args.next().unwrap_or_else(|| "http://cityboulder.ch/Corona_Ticker/get_guests.php".to_string());
    let sqlite_path = args.next().unwrap_or_else(|| "sqlite.db".to_string() );
    let lookup = task::block_on(fetch(url)).unwrap();
    let repo = cityboulder_data::VisitorRepository::new(&sqlite_path);
    repo.run_migration();
    repo.save_visitor_lookup(&lookup);
    
    for visitor in repo.load() {
        println!("Got a visitor with id: {}", serde_json::to_string(&visitor).unwrap());
    }
}