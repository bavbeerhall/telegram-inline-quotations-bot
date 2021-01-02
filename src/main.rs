use clap::{App, Arg};
use teloxide::dispatching::Dispatcher;

#[tokio::main]
async fn main() {
    telegram_inline_quotations_bot::init_log();
    let app = App::new("Telegram Inline Quotations Bot")
        .version("1.0")
        .arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .value_name("token")
                .help("Bot token")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("filename")
                .help("Filename of the quotations")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    Dispatcher::new(
        teloxide::BotBuilder::new()
            .token(app.value_of("token").unwrap())
            .build(),
    )
        .inline_queries_handler(telegram_inline_quotations_bot::InlineQueryHandler::new(
            app.value_of("file").unwrap(),
        ))
        .dispatch()
        .await;
}
