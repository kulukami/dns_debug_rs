use debug_uget::{std_resolve, trust_from_system_conf_resolve};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 {
        println!("Try resolving ... ");
    } else {
        println!("usage:\n ./resolve www.baidu.com google.com ... ");
        return;
    }

    for i in 1..=args.len() - 1 {
        let each_target = &args[i];

        match trust_from_system_conf_resolve(each_target) {
            Ok(ipinfo) => {
                println!("td_sys_resolve: {:?}", ipinfo);
            }
            Err(e) => {
                println!("td_sys_resolve error: {:?}", e);
            }
        }

        match std_resolve(each_target) {
            Ok(ipinfo) => {
                println!("rs_std_resolve: {:?}", ipinfo);
            }
            Err(e) => {
                println!("rs_std_resolve error: {:?}", e);
            }
        }
    }
}
