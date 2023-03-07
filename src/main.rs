use clap::{Parser,ValueEnum};
use pbs::{Attrl,Op,Server};
use regex::Regex;
use serde_json;

#[derive(Debug, Parser)]
#[command(name = "pbs_metrics")]
#[command(about = "generate metrics for pbs", long_about = None)]
struct Cli {
    #[arg(value_enum)]
    resource: Resource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Resource {
    Host,
    Resv,
    Que,
}

fn main() {
    let args = Cli::parse();
    let srv = Server::new();
    match args.resource {
        Resource::Host => {
            let re_good = Regex::new(r"free|resv|job").unwrap();
            let re_bad = Regex::new(r"off|down|unknown").unwrap();
            let resp: Vec<_> = srv.stat_host(&None, None).unwrap().resources.iter_mut().map(|x| {
                x.add("name".to_string(), Attrl::Value(Op::Default(x.name())));
                if let Attrl::Value(op) = x.attribs().get("state").unwrap() {
                    if re_good.is_match(&op.val()) && !re_bad.is_match(&op.val()) {
                        x.add("avail".to_string(), Attrl::Value(Op::Default("true".to_string())));
                    } else {
                        x.add("avail".to_string(), Attrl::Value(Op::Default("false".to_string())));
                    }
                } else { panic!() }
                x.attribs().json()
            }).collect();
            println!("{{\"measurement\": \"pbs_hosts\", \"datapoints\": {}}}", serde_json::to_string(&resp).unwrap());
        },
        Resource::Resv => {
            let resp: Vec<_> = srv.stat_reservation(&None, None).unwrap().resources.iter_mut().map(|x| {
                x.add("name".to_string(), Attrl::Value(Op::Default(x.name())));
                x.attribs().json()
            }).collect();
            println!("{{\"measurement\": \"pbs_resvs\", \"datapoints\": {}}}", serde_json::to_string(&resp).unwrap());
        },
        Resource::Que => {
            let resp: Vec<_> = srv.stat_que(&None, None).unwrap().resources.iter_mut().map(|x| {
                x.add("name".to_string(), Attrl::Value(Op::Default(x.name())));
                x.attribs().json()
            }).collect();
            println!("{{\"measurement\": \"pbs_resvs\", \"datapoints\": {}}}", serde_json::to_string(&resp).unwrap());
        },
    };
}
