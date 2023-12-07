use scraper::Selector;
use serde::de::DeserializeOwned;
use tracing::debug;

use crate::{error::Error, Client};

use self::deserializer::Deserializer;

pub mod deserializer;

static SQL_URL: &str =
    "https://erogamescape.dyndns.org/~ap2/ero/toukei_kaiseki/sql_for_erogamer_form.php";

impl Client {
    #[tracing::instrument(err, skip(self))]
    pub async fn execute_sql<T: DeserializeOwned>(
        &self,
        sql: &str,
    ) -> Result<Vec<T>, crate::error::Error> {
        let text = self
            .client
            .post(SQL_URL)
            .form(&[("sql", sql)])
            .send()
            .await?
            .text()
            .await?;
        parse_sql_reponse(&text)
    }
}

fn parse_sql_reponse<T: DeserializeOwned>(text: &str) -> Result<Vec<T>, crate::error::Error> {
    let document = scraper::Html::parse_document(text);

    let result_title = document
        .select(&Selector::parse("#query_result_header").unwrap())
        .next()
        .ok_or_else(|| Error::HtmlParse("No result title".to_string()))?
        .text()
        .next()
        .ok_or_else(|| Error::HtmlParse("No result title".to_string()))?;

    if result_title.trim() == "エラー(SQL間違い)" {
        let error_message = document
            .select(&Selector::parse("#query_result_main").unwrap())
            .next()
            .ok_or_else(|| Error::HtmlParse("No error message".to_string()))?
            .text()
            .collect::<String>();
        return Err(Error::SqlExecute(error_message));
    }

    let rows_s = Selector::parse("#query_result_main tr").unwrap();
    let mut rows = document.select(&rows_s);

    let Some(headers) = rows.next() else {
        return Ok(Vec::new());
    };
    let headers = headers
        .select(&Selector::parse("th").unwrap())
        .map(|th| th.text().collect::<String>())
        .collect::<Vec<_>>();

    let mut contents = Vec::new();

    for row in rows {
        let content_strs = row
            .select(&Selector::parse("td").unwrap())
            .map(|td| td.text().collect::<String>())
            .collect::<Vec<_>>();
        if content_strs.len() != headers.len() {
            return Err(Error::HtmlParse(
                "The number of columns is different from the number of headers.".to_string(),
            ));
        }

        let deserialized: T = T::deserialize(&mut Deserializer::new(&headers, &content_strs))?;

        contents.push(deserialized);
    }

    Ok(contents)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::api::sql::parse_sql_reponse;

    #[derive(serde::Deserialize, Debug)]
    enum MakerKind {
        #[serde(rename = "CORPORATION")]
        Corporation,
    }

    #[allow(dead_code)]
    #[derive(serde::Deserialize, Debug)]
    struct Success1 {
        pub id: i32,
        pub brandname: String,
        pub brandfurigana: String,
        pub makername: String,
        pub makerfurigana: String,
        pub checked: Option<String>,
        pub url: String,
        pub kind: MakerKind,
        pub lost: bool,
        pub directlink: bool,
        pub median: i32,
        pub http_response_code: Option<i32>,
        pub twitter: Option<String>,
        pub twitter_data_widget_id: Option<String>,
        pub notes: Option<String>,
        pub erogetrailers: Option<i32>,
        pub cien: Option<String>,
    }

    #[test]
    fn sql_parse_1() {
        let html = include_str!("./test_data/success_1.html");
        let data: Vec<Success1> = parse_sql_reponse(html).unwrap();

        dbg!(&data);
    }
    #[test]
    fn sql_parse_hashmap() {
        let html = include_str!("./test_data/success_1.html");
        let data: Vec<HashMap<String, String>> = parse_sql_reponse(html).unwrap();

        dbg!(&data);
    }

    #[test]
    fn sql_parse_execute_error() {
        let html = include_str!("./test_data/error_1.html");
        let error: crate::error::Error = parse_sql_reponse::<Vec<Success1>>(html).unwrap_err();

        dbg!(&error);

        if let crate::error::Error::SqlExecute(message) = &error {
            assert_eq!(
                *message,
                r#"ERROR:  relation "aiueo" does not exist
LINE 1: EXPLAIN SELECT * FROM aiueo WHERE url=1
                              ^"#
                .to_string()
            );
        } else {
            panic!("error is not SqlExecute");
        }
    }

    #[tokio::test]
    async fn sql_execute_1() {
        let client = crate::Client::default();
        let data: Vec<Success1> = client
            .execute_sql("SELECT * FROM brandlist WHERE id=1795")
            .await
            .unwrap();
        let data = data.get(0).unwrap();

        assert_eq!(data.brandname, "ゆずソフト");

        dbg!(&data);
    }
}
