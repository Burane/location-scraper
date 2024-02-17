use scraper::{Html, Selector};
use scraper::Node::Document;

const URL: &str = "https://www.afedim.fr/fr/location/annonces/Appartement/Caen-France/1-5-pieces/surface-0-100-m2/budget-0-1500-euros/rayon-10-km/disponible-False/options-/exclusPlafondRess-/Resultats";




pub async fn get_html() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:78.0) Gecko/20100101 Firefox/78.0")
        .build()?;
    let res = client.get(URL).send().await?;
    let body = res.text().await?;
    Ok(body)
}


// parse html to get announces data
pub async fn parse_html(html: &str)  {
    let html = get_html().await.unwrap();
    let document =  Html::parse_document(&html);
    let link_selector = Selector::parse("C:blocRecherche.blocRechercheDesk.P.C.F6_0.C5:link").unwrap();

    for link in document.select(&link_selector) {
        // Extract the href attribute of each link
        if let Some(href) = link.value().attr("href") {
            println!("Found link: {}", href);
        }
    }
}