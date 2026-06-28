use z_cognition::Rule;
use z_runtime::CognitiveAgent;
use axum::{extract::State, http::StatusCode, response::Html, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
struct AskRequest {
    question: String,
}

#[derive(Serialize)]
struct AskResponse {
    answer: String,
    source: String,
    beliefs: usize,
}

type SharedAgent = Arc<Mutex<CognitiveAgent>>;

async fn ask_handler(
    State(agent): State<SharedAgent>,
    Json(req): Json<AskRequest>,
) -> Result<Json<AskResponse>, StatusCode> {
    let mut agent = agent.lock().await;
    let (answer, source) = agent.think(&req.question).await;
    let beliefs = agent.belief_count();
    Ok(Json(AskResponse {
        answer,
        source: source.to_string(),
        beliefs,
    }))
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

fn add_rules(agent: &mut CognitiveAgent) {
    // -- Greetings (one rule per word = 100% match on single word) --
    agent.add_rule(Rule::new("greeting:hello").with_condition("hello").with_conclusion("greeting"));
    agent.add_rule(Rule::new("greeting:hi").with_condition("hi").with_conclusion("greeting"));
    agent.add_rule(Rule::new("greeting:hey").with_condition("hey").with_conclusion("greeting"));
    agent.add_rule(Rule::new("greeting:sup").with_condition("sup").with_conclusion("greeting"));
    agent.add_rule(Rule::new("greeting:greetings").with_condition("greetings").with_conclusion("greeting"));
    agent.add_rule(Rule::new("greeting:greeting").with_condition("greeting").with_conclusion("greeting"));

    // -- What is ZeroicAI (general) --
    agent.add_rule(Rule::new("topic:what_is")
        .with_condition("what").with_condition("is").with_condition("zeroicai")
        .with_conclusion("what_is_zeroicai"));

    // -- Layman / simple explanation --
    agent.add_rule(Rule::new("topic:layman")
        .with_condition("simple").with_condition("layman").with_condition("explain")
        .with_condition("eli5").with_condition("plain").with_condition("easy")
        .with_conclusion("layman"));

    // -- Examples / what can you build --
    agent.add_rule(Rule::new("topic:examples")
        .with_condition("example").with_condition("build").with_condition("create")
        .with_condition("make").with_condition("use").with_condition("application")
        .with_condition("project").with_condition("could")
        .with_conclusion("examples"));

    // -- How it works --
    agent.add_rule(Rule::new("topic:how_it_works")
        .with_condition("how").with_condition("work").with_condition("does")
        .with_conclusion("how_it_works"));

    // -- Getting started --
    agent.add_rule(Rule::new("topic:getting_started")
        .with_condition("start").with_condition("begin").with_condition("setup")
        .with_condition("install").with_condition("quickstart").with_condition("tutorial")
        .with_conclusion("getting_started"));

    // -- Crates / architecture --
    agent.add_rule(Rule::new("topic:crates")
        .with_condition("crate").with_condition("module").with_condition("package")
        .with_condition("architecture").with_condition("structure")
        .with_conclusion("crates"));

    // -- Patterns (general) --
    agent.add_rule(Rule::new("topic:patterns")
        .with_condition("pattern").with_condition("organizational")
        .with_condition("many").with_condition("list")
        .with_conclusion("patterns"));

    // -- Individual patterns --
    agent.add_rule(Rule::new("topic:hierarchy")
        .with_condition("hierarchy").with_condition("hierarchical")
        .with_condition("command").with_condition("chain")
        .with_conclusion("hierarchy"));

    agent.add_rule(Rule::new("topic:swarm")
        .with_condition("swarm").with_condition("flock").with_condition("decentralized")
        .with_conclusion("swarm"));

    agent.add_rule(Rule::new("topic:coalition")
        .with_condition("coalition").with_condition("alliance").with_condition("temporary")
        .with_conclusion("coalition"));

    agent.add_rule(Rule::new("topic:market")
        .with_condition("market").with_condition("auction").with_condition("bid")
        .with_conclusion("market"));

    agent.add_rule(Rule::new("topic:federation")
        .with_condition("federation").with_condition("vote").with_condition("voting")
        .with_condition("governance")
        .with_conclusion("federation"));

    agent.add_rule(Rule::new("topic:team")
        .with_condition("team").with_condition("role").with_condition("leader")
        .with_condition("coordinator").with_condition("executor")
        .with_conclusion("team"));

    agent.add_rule(Rule::new("topic:holarchy")
        .with_condition("holarchy").with_condition("holon").with_condition("nested")
        .with_conclusion("holarchy"));

    agent.add_rule(Rule::new("topic:blackboard")
        .with_condition("blackboard").with_condition("shared").with_condition("knowledge")
        .with_conclusion("blackboard"));

    // -- Messaging --
    agent.add_rule(Rule::new("topic:messaging")
        .with_condition("message").with_condition("router").with_condition("send")
        .with_condition("communicate").with_condition("talk")
        .with_conclusion("messaging"));

    agent.add_rule(Rule::new("topic:performatives")
        .with_condition("performative").with_condition("fipa")
        .with_condition("inform").with_condition("request")
        .with_conclusion("performatives"));

    // -- BDI / Cognition --
    agent.add_rule(Rule::new("topic:bdi")
        .with_condition("bdi").with_condition("belief").with_condition("desire")
        .with_condition("intention")
        .with_conclusion("bdi"));

    agent.add_rule(Rule::new("topic:cognition")
        .with_condition("cognition").with_condition("reasoning").with_condition("think")
        .with_condition("reason").with_condition("belief")
        .with_conclusion("cognition_crate"));

    // -- Runtime / Supervisor --
    agent.add_rule(Rule::new("topic:runtime")
        .with_condition("runtime").with_condition("run").with_condition("execute")
        .with_condition("spawn").with_condition("task")
        .with_conclusion("runtime"));

    agent.add_rule(Rule::new("topic:supervisor")
        .with_condition("supervisor").with_condition("restart").with_condition("crash")
        .with_condition("failure").with_condition("recovery")
        .with_conclusion("supervisor"));

    // -- Agent trait --
    agent.add_rule(Rule::new("topic:agent_trait")
        .with_condition("trait").with_condition("implement").with_condition("agent")
        .with_condition("interface").with_condition("method")
        .with_conclusion("agent_trait"));

    agent.add_rule(Rule::new("topic:handle_message")
        .with_condition("handle").with_condition("receive").with_condition("incoming")
        .with_conclusion("handle_message"));

    agent.add_rule(Rule::new("topic:agent_context")
        .with_condition("context").with_condition("agentcontext")
        .with_conclusion("agent_context"));

    // -- Why Rust --
    agent.add_rule(Rule::new("topic:why_rust")
        .with_condition("why").with_condition("rust").with_condition("language")
        .with_condition("performance").with_condition("fast")
        .with_conclusion("why_rust"));

    // -- Comparison --
    agent.add_rule(Rule::new("topic:comparison")
        .with_condition("compare").with_condition("vs").with_condition("versus")
        .with_condition("difference").with_condition("other").with_condition("alternative")
        .with_condition("crewai").with_condition("autogen").with_condition("langgraph")
        .with_conclusion("comparison"));

    // -- xbot --
    agent.add_rule(Rule::new("topic:xbot")
        .with_condition("xbot").with_condition("twitter").with_condition("bot")
        .with_conclusion("xbot"));

    // -- Vision --
    agent.add_rule(Rule::new("topic:vision")
        .with_condition("vision").with_condition("goal").with_condition("roadmap")
        .with_condition("future").with_condition("plan")
        .with_conclusion("vision"));

    // -- Robotics --
    agent.add_rule(Rule::new("topic:robotics")
        .with_condition("robot").with_condition("robotics").with_condition("drone")
        .with_condition("autonomous").with_condition("iot")
        .with_conclusion("robotics"));

    // -- Agent vs Microservice --
    agent.add_rule(Rule::new("topic:agent_vs_microservice")
        .with_condition("microservice").with_condition("difference")
        .with_condition("versus").with_condition("actor")
        .with_conclusion("agent_vs_microservice"));

    // -- External APIs --
    agent.add_rule(Rule::new("topic:external_apis")
        .with_condition("api").with_condition("external").with_condition("integrate")
        .with_condition("http").with_condition("rest")
        .with_conclusion("external_apis"));

    // -- Owner / creator --
    agent.add_rule(Rule::new("owner:owner").with_condition("owner").with_conclusion("owner"));
    agent.add_rule(Rule::new("owner:creator").with_condition("creator").with_conclusion("owner"));
    agent.add_rule(Rule::new("owner:founder").with_condition("founder").with_conclusion("owner"));
    agent.add_rule(Rule::new("owner:who_made").with_condition("who").with_condition("made").with_conclusion("owner"));
    agent.add_rule(Rule::new("owner:who_built").with_condition("who").with_condition("built").with_conclusion("owner"));
    agent.add_rule(Rule::new("owner:who_created").with_condition("who").with_condition("created").with_conclusion("owner"));

    // -- GitHub --
    agent.add_rule(Rule::new("github:github").with_condition("github").with_conclusion("github"));
    agent.add_rule(Rule::new("github:repo").with_condition("repo").with_conclusion("github"));
    agent.add_rule(Rule::new("github:repository").with_condition("repository").with_conclusion("github"));
        
    // -- Open source / license --
    agent.add_rule(Rule::new("topic:open_source")
        .with_condition("open").with_condition("source").with_condition("license")
        .with_condition("github").with_condition("free")
        .with_conclusion("open_source"));

    // -- Individual crates --
    agent.add_rule(Rule::new("topic:z_core")
        .with_condition("z-core").with_condition("core").with_condition("agentid")
        .with_condition("agentstate").with_condition("trait")
        .with_conclusion("z_core"));

    agent.add_rule(Rule::new("topic:z_messaging")
        .with_condition("z-messaging").with_condition("mailbox").with_condition("router")
        .with_condition("channel").with_condition("delivery")
        .with_conclusion("z_messaging"));

    agent.add_rule(Rule::new("topic:z_cognition")
        .with_condition("z-cognition").with_condition("cognition").with_condition("reasoning")
        .with_condition("beliefbase").with_condition("planner")
        .with_conclusion("z_cognition"));

    agent.add_rule(Rule::new("topic:z_patterns")
        .with_condition("z-patterns").with_condition("organizational").with_condition("coordination")
        .with_conclusion("z_patterns"));

    agent.add_rule(Rule::new("topic:z_runtime")
        .with_condition("z-runtime").with_condition("execution").with_condition("engine")
        .with_conclusion("z_runtime"));

    // -- Circuit breaker --
    agent.add_rule(Rule::new("topic:circuit_breaker")
        .with_condition("circuit").with_condition("breaker").with_condition("fault")
        .with_condition("open").with_condition("halfopen")
        .with_conclusion("circuit_breaker"));

    // -- Scheduler --
    agent.add_rule(Rule::new("topic:scheduler")
        .with_condition("scheduler").with_condition("round").with_condition("robin")
        .with_condition("priority").with_condition("queue").with_condition("fair")
        .with_conclusion("scheduler"));

    // -- Metrics --
    agent.add_rule(Rule::new("topic:metrics")
        .with_condition("metric").with_condition("counter").with_condition("gauge")
        .with_condition("histogram").with_condition("telemetry").with_condition("monitor")
        .with_conclusion("metrics"));

    // -- Sandbox --
    agent.add_rule(Rule::new("topic:sandbox")
        .with_condition("sandbox").with_condition("isolation").with_condition("resource")
        .with_condition("limit").with_condition("quota")
        .with_conclusion("sandbox"));

    // -- AgentId --
    agent.add_rule(Rule::new("topic:agent_id")
        .with_condition("agentid").with_condition("uuid").with_condition("identity")
        .with_condition("unique").with_condition("id")
        .with_conclusion("agent_id"));

    // -- Installation / Cargo --
    agent.add_rule(Rule::new("topic:installation")
        .with_condition("install").with_condition("cargo").with_condition("toml")
        .with_condition("dependency").with_condition("add")
        .with_conclusion("installation"));

    // -- Performance --
    agent.add_rule(Rule::new("topic:performance")
        .with_condition("performance").with_condition("benchmark").with_condition("speed")
        .with_condition("latency").with_condition("throughput").with_condition("memory")
        .with_conclusion("performance"));

    // -- Roadmap --
    agent.add_rule(Rule::new("topic:roadmap")
        .with_condition("roadmap").with_condition("phase").with_condition("timeline")
        .with_condition("release").with_condition("plan").with_condition("next")
        .with_conclusion("roadmap"));

    // -- Products --
    agent.add_rule(Rule::new("topic:products")
        .with_condition("product").with_condition("cortex").with_condition("arena")
        .with_condition("recall").with_condition("building")
        .with_conclusion("products"));

    agent.add_rule(Rule::new("topic:cortex")
        .with_condition("cortex")
        .with_conclusion("cortex"));

    agent.add_rule(Rule::new("topic:arena")
        .with_condition("arena").with_condition("simulation").with_condition("simulate")
        .with_conclusion("arena"));

    agent.add_rule(Rule::new("topic:recall")
        .with_condition("recall").with_condition("memory").with_condition("persistent")
        .with_condition("episodic").with_condition("long-running")
        .with_conclusion("recall"));

    // -- Community --
    agent.add_rule(Rule::new("topic:community")
        .with_condition("community").with_condition("discord").with_condition("join")
        .with_condition("contribute").with_condition("social")
        .with_conclusion("community"));

    // -- Use cases --
    agent.add_rule(Rule::new("topic:use_cases")
        .with_condition("use").with_condition("case").with_condition("defi")
        .with_condition("fintech").with_condition("blockchain").with_condition("supply")
        .with_conclusion("use_cases"));

    // -- FIPA --
    agent.add_rule(Rule::new("topic:fipa")
        .with_condition("fipa").with_condition("acl").with_condition("standard")
        .with_condition("compliant").with_condition("ieee")
        .with_conclusion("fipa"));

    // -- License --
    agent.add_rule(Rule::new("topic:license")
        .with_condition("license").with_condition("mit").with_condition("apache")
        .with_condition("commercial").with_condition("dual")
        .with_conclusion("license"));

    // -- Docs --
    agent.add_rule(Rule::new("topic:docs")
        .with_condition("docs").with_condition("documentation").with_condition("guide")
        .with_condition("mdbook").with_condition("reference")
        .with_conclusion("docs"));

    // -- Status --
    agent.add_rule(Rule::new("topic:status")
        .with_condition("status").with_condition("current").with_condition("progress")
        .with_condition("stable").with_condition("alpha").with_condition("beta")
        .with_conclusion("status"));

    // -- DeFi / Blockchain --
    agent.add_rule(Rule::new("topic:defi")
        .with_condition("defi").with_condition("crypto").with_condition("mev")
        .with_condition("onchain").with_condition("solana").with_condition("ethereum")
        .with_conclusion("defi"));
}

#[tokio::main]
async fn main() {
    println!("Loading CognitiveAgent...");

    let mut agent = CognitiveAgent::from_config("data/beliefs.json", "data/config.json");
    add_rules(&mut agent);

    println!("  {} beliefs, {} rules loaded", agent.belief_count(), agent.rule_count());

    let shared = Arc::new(Mutex::new(agent));

    let app = Router::new()
        .route("/", get(index))
        .route("/ask", post(ask_handler))
        .with_state(shared);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("  Server running at http://localhost:{}", port);
    println!("  Press Ctrl+C to stop\n");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
