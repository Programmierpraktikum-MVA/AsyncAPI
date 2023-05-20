use gtmpl_derive::Gtmpl;

#[derive(Gtmpl)]
pub struct PubsubTemplate {
    pub server_url: String,
    pub channel_name: String,
    pub schema: String,
}
