use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::DateTime;
use chrono::Utc;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct Code(String);

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct Id(String);

#[derive(Deserialize, Debug)]
pub struct Ticket {
    pub id: Id,
    pub departure_code: Code,
    pub arrival_code: Code,

    #[serde(deserialize_with = "from_ts")]
    pub departure_time: DateTime<Utc>,

    #[serde(deserialize_with = "from_ts")]
    pub arrival_time: DateTime<Utc>,
    pub price: u64,
}

#[derive(Deserialize)]
pub struct Tickets {
    pub tickets: Vec<Ticket>,
}
