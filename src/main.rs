use reqwest::{Client,Error};
enum Network {
    Mtn,
    Airtel,
}

struct DataPlan {
    name: String,
    price: u32,
    plan_id: u32
}

fn get_data_plans(network: &Network) -> Vec<DataPlan> {
    match network {
        Network::Mtn => vec![
            DataPlan {
                name: "MTN SME 500MB".to_string(),
                price: 200,
                plan_id: 157,
            },
            DataPlan {
                name: "MTN SME 2GB".to_string(),
                price: 400,
                plan_id: 200,
            },
        ],
        Network::Airtel => vec![
            DataPlan {
                name: "Airtel 500MB".to_string(),
                price: 150,
                plan_id:2000,
            },
            DataPlan {
                name: "Airtel 1.5GB".to_string(),
                price: 300,
                plan_id:300,
            },
        ],
    }
}

fn choose_plan(plans: &[DataPlan]) -> Option<&DataPlan> {
    println!("Please choose a data plan:");

    for (i, plan) in plans.iter().enumerate() {
        println!("PlanID{} {}: {} - {} Naira", i + 1, plan.plan_id,plan.name, plan.price);
    }

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    
    if let Ok(index) = choice.trim().parse::<usize>() {
        if index > 0 && index <= plans.len() {
            return Some(&plans[index - 1]);
        }
    }

    None
}

fn get_phone_number() -> String {
    println!("Please enter your phone number:");
    let mut phone_number = String::new();
    std::io::stdin().read_line(&mut phone_number).expect("Failed to read line");
    phone_number.trim().to_string()
}

async fn send_request(plan: &DataPlan, phone_number: &str) -> Result<(), reqwest::Error> {
   let planid:Vec<DataPlan> = plan.plan_id;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Step 1: Choose a network
    println!("Please choose a network:");
    println!("1: MTN");
    println!("2: Airtel");

    let mut network_choice = String::new();
    std::io::stdin().read_line(&mut network_choice).expect("Failed to read line");

    let network = match network_choice.trim() {
        "1" => Network::Mtn,
        "2" => Network::Airtel,
        _ => {
            println!("Invalid choice. Exiting.");
            return;
        }
    };

    // Step 2: Load data plans based on the network
    let plans = get_data_plans(&network);

    // Step 3: Choose a plan
    if let Some(selected_plan) = choose_plan(&plans) {
        // Step 4: Capture the phone number
        let phone_number = get_phone_number();

        // Step 5: Send the request using reqwest
        if let Err(e) = send_request(selected_plan, &phone_number).await {
            println!("An error occurred: {}", e);
        }
    } else {
        println!("Invalid plan choice. Exiting.");
    }
}
