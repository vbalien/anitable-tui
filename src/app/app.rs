use std::vec;
use chrono::{Local, DateTime, Datelike};
use std::convert::TryFrom;
use crate::util::TabsState;
use anitable::*;


pub struct App<'a> {
    pub tabs: TabsState<'a>,
    pub items: Vec<AnimeData>,
    pub selected: usize,
    pub should_quit: bool,
    client: Anitable,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let local: DateTime<Local> = Local::now();
        Self {
            tabs: TabsState::new(vec!["일", "월", "화", "수", "목", "금", "토", "기타", "신작"], (local.weekday() as usize + 1) % 7),
            items: vec![],
            selected: 0,
            should_quit: false,
            client: Anitable::new(),
        }
    }

    pub async fn get_list(&mut self) {
        self.selected = 0;
        let tabletype = Tabletype::try_from(self.tabs.index as u8).unwrap();
        self.items = self.client.list(tabletype).await.unwrap();
    }

    pub async fn on_key(&mut self, key: char) {
        match key {
            'q' => self.should_quit = true,
            'r' => {self.get_list().await;},
            _ => {},
        }
    }

    pub async fn on_next(&mut self) {
        self.tabs.next();
        self.get_list().await;
    }

    pub async fn on_prev(&mut self) {
        self.tabs.previous();
        self.get_list().await;
    }

    pub fn on_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.items.len() - 1;
        }
    }

    pub fn on_down(&mut self) {
        self.selected += 1;
        if self.selected > self.items.len() - 1 {
            self.selected = 0;
        }
    }
}
