use openparty::config::Config;
use openparty::telegram;
use openparty::params::Params;


fn main() {
    let cfg: Config = Config::build();
    let params: Params = Params{ cfg: Option(cfg) };
    
    
}
