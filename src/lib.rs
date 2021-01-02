use std::fs::File;
use std::io;
use std::io::BufRead;

use futures::future::BoxFuture;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Handle;
use log::LevelFilter;
use rand::seq::IteratorRandom;
use teloxide::dispatching::{DispatcherHandler, DispatcherHandlerRx};
use teloxide::error_handlers::OnError;
use teloxide::prelude::UpdateWithCx;
use teloxide::requests::Request;
use teloxide::types::{
    InlineQuery, InlineQueryResult, InlineQueryResultArticle, InputMessageContent,
    InputMessageContentText,
};

macro_rules! appender {
    ($l:tt,$L:ident) => {
        Appender::builder()
            .filter(Box::new(ThresholdFilter::new(LevelFilter::$L)))
            .build(
                stringify!($l),
                Box::new(
                    FileAppender::builder()
                        .encoder(Box::new(PatternEncoder::new(
                            "[{h({l})}] {d(%Y-%m-%d %H:%M:%S)} {f}:{L}{n}{m}{n}{n}",
                        )))
                        .build("log/".to_owned() + stringify!($l) + ".log")
                        .unwrap(),
                ),
            )
    };
}

pub struct InlineQueryHandler {
    quotations: Vec<String>,
}

impl InlineQueryHandler {
    pub fn new(filename: &str) -> InlineQueryHandler {
        let mut handler = InlineQueryHandler { quotations: vec![] };
        let lines = io::BufReader::new(File::open(filename).unwrap()).lines();
        for line in lines {
            if let Ok(quotation) = line {
                handler.quotations.push(quotation)
            }
        }
        handler
    }
}

impl DispatcherHandler<InlineQuery> for InlineQueryHandler {
    fn handle(self, mut updates: DispatcherHandlerRx<InlineQuery>) -> BoxFuture<'static, ()>
        where
            UpdateWithCx<InlineQuery>: Send + 'static,
    {
        Box::pin(async move {
            loop {
                let mut vec: Vec<InlineQueryResult> = vec![];
                let quotations: Vec<&String>;
                if let Some(msg) = updates.recv().await {
                    if msg.update.query == "" {
                        if self.quotations.len() <= 5 {
                            quotations = self.quotations.iter().collect();
                        } else {
                            quotations = self
                                .quotations
                                .iter()
                                .choose_multiple(&mut rand::thread_rng(), 5);
                        }
                    } else {
                        quotations = self
                            .quotations
                            .iter()
                            .filter(|quotation| quotation.contains(&msg.update.query))
                            .collect::<Vec<&String>>();
                    }
                    for quotation in quotations {
                        vec.push(InlineQueryResult::Article(InlineQueryResultArticle::new(
                            quotation.as_str(),
                            quotation.as_str(),
                            InputMessageContent::Text(InputMessageContentText::new(
                                quotation.as_str(),
                            )),
                        )));
                        if vec.len() >= 50 {
                            break;
                        }
                    }
                    let answer = msg.bot.answer_inline_query(msg.update.id.clone(), vec);
                    answer.cache_time(5).send().await.log_on_error().await;
                };
            }
        })
    }
}

pub fn init_log() -> Handle {
    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Trace)))
                .build(
                    "stdout",
                    Box::new(
                        ConsoleAppender::builder()
                            .encoder(Box::new(PatternEncoder::new(
                                "[{h({l})}] {d(%Y-%m-%d %H:%M:%S)} {f}:{L}{n}{m}{n}{n}",
                            )))
                            .build(),
                    ),
                ),
        )
        .appender(appender!(info, Info))
        .appender(appender!(trace, Trace))
        .appender(appender!(debug, Debug))
        .appender(appender!(error, Error))
        .appender(appender!(warn, Warn))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("info")
                .appender("warn")
                .appender("debug")
                .appender("error")
                .appender("trace")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    log4rs::init_config(config).unwrap()
}
