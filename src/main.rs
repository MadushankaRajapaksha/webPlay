use axum::{
    routing::{get, post},
    response::Html,
    extract::{Form},
    Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, IpAddr};



#[derive(Debug, Deserialize, Serialize)]
struct VideoForm {
    video_url: String,
}

#[derive(Parser)]
#[command(name = "webPlay")]
#[command(version = "1.0.0")]
#[command(about = "A simple web-based video player server")]
#[command(long_about = "WebPlay is a lightweight web server that allows you to stream videos from URLs on any device in your network.

USAGE:
    webplay [OPTIONS]

EXAMPLES:
    webplay                    # Start server on default port 3000
    webplay --port 8080        # Start server on port 8080
    webplay --help             # Show this help message

After starting the server:
1. Open any of the displayed URLs in your browser
2. Enter a video URL (supports direct video links)
3. Click 'Play Video' to start streaming

SUPPORTED FORMATS:
- MP4, WebM, OGV video formats
- Direct video URLs from any source
- HLS streams (m3u8)

FEATURES:
- Network streaming to any device
- Custom video player with controls
- Picture-in-Picture mode
- Fullscreen support
- Keyboard shortcuts
- Mobile-friendly interface")]
struct Args {
    /// Port to run the server on
    #[arg(short, long, default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Create static directory if it doesn't exist


    let app = Router::new()
        .route("/", get(home_page))
        .route("/play", post(play_video_post));


    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));

    // Print all available IP addresses
    print_server_addresses(args.port);

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to port {}: {}", args.port, e);
            eprintln!("Try a different port with --port <number>");
            std::process::exit(1);
        }
    };

    println!("\n🚀 Server is running!");
    println!("📱 Share any of the above URLs with devices on your network\n");

    // Print usage guide
    print_usage_guide();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn print_usage_guide() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                      WebPlay Usage Guide                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("🎬 HOW TO USE:");
    println!("   1. Copy a video URL from any source");
    println!("   2. Paste it into the input field on the web page");
    println!("   3. Click '▶ Play Video' to start streaming");
    println!();
    println!("🎮 PLAYER CONTROLS:");
    println!("   • Spacebar: Play/Pause");
    println!("   • Arrow Keys: Seek ±5 seconds");
    println!("   • F: Toggle fullscreen");
    println!("   • M: Mute/Unmute");
    println!("   • Click video: Play/Pause");
    println!("   • Drag progress bar: Seek");
    println!("   • Picture-in-Picture button: PiP mode");
    println!();
    println!("📱 MOBILE SUPPORT:");
    println!("   • Touch to play/pause");
    println!("   • Swipe left/right to seek");
    println!("   • All controls work on mobile devices");
    println!();
    println!("🔧 TROUBLESHOOTING:");
    println!("   • Make sure you're on the same WiFi network");
    println!("   • Check firewall settings if others can't connect");
    println!("   • Supported formats: MP4, WebM, OGV, HLS");
    println!();
    println!("════════════════════════════════════════════════════════════════");
    println!();
}

fn print_server_addresses(port: u16) {
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║          Video Player Server - Available URLs          ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    // Get all network interfaces
    if let Ok(interfaces) = local_ip_address::list_afinet_netifas() {
        let mut found_ips = Vec::new();

        for (name, ip) in interfaces {
            // Skip loopback and filter for IPv4
            if ip.is_loopback() {
                continue;
            }

            match ip {
                IpAddr::V4(ipv4) => {
                    let url = format!("http://{}:{}", ipv4, port);
                    println!("  🌐 {} ({})", url, name);
                    found_ips.push(url);
                }
                _ => {}
            }
        }

        // Always show localhost
        println!("  🏠 http://localhost:{} (Local)", port);
        println!("  🏠 http://127.0.0.1:{} (Local)", port);

        if found_ips.is_empty() {
            println!("\n⚠️  No network interfaces found!");
            println!("   Make sure you're connected to WiFi/Ethernet");
        }
    } else {
        println!("❌ Could not detect network interfaces");
        println!("🏠 Fallback: http://localhost:{}", port);
    }
}

async fn home_page() -> Html<String> {
    // read with file
    let content = include_str!("../static/player.html");
    Html(content.to_string())


}
 

async fn play_video_post(Form(video_form): Form<VideoForm>) -> Html<String> {
    let video_url = video_form.video_url;

    let content = include_str!("../static/video.html");
    let html = content.replace("<video id=\"videoPlayer\" preload=\"metadata\"></video>", &format!("<video id=\"videoPlayer\" src=\"{}\" preload=\"metadata\"></video>", video_url));

    Html(html)
}
