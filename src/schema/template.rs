use {
    markdown_it::{plugins::cmark, MarkdownIt},
    tagger::no_attr,
};

use {super::*, crate::Renderer};

impl Website {
    pub fn render(&self, r: &mut Renderer) {
        r.elem("html", |r| {
            r.attr("xmlns", "http://www.w3.org/1999/xhtml");
            r.attr("lang", "fr");
        })
            .build(|r| {
                r.elem("head", no_attr()).build(|r| {
                    r.elem("title", no_attr()).build(|r| {
                        r.put_raw(format!(
                            "{} - {}",
                            self.home.title.0 .0, self.home.subtitle.0 .0
                        ));
                    });

                    r.single("link", |r| {
                        r.attr("rel", "preload");
                        r.attr("as", "font");
                        r.attr("type", "font/woff");
                        r.attr("crossorigin", "anonymous");
                        r.attr("href", "./res/font/fengardoneue_regular-webfont.woff");
                    });


                    r.single("link", |r| {
                        r.attr("rel", "preload");
                        r.attr("as", "image");
                        r.attr("href", "./res/images/back.svg");
                    });

                    r.single("link", |r| {
                        r.attr("rel", "preload");
                        r.attr("as", "image");
                        r.attr("href", "./res/images/back.hover.svg");
                    });

                    r.elem("style", no_attr()).build(|r| {
                        r.put_raw("");
                    });

                    r.single("link", |r| {
                        r.attr("rel", "stylesheet");
                        r.attr("href", "./res/style/all.css");

                        let todo = "Clean/split this messy stylesheet
                        and include it inline into this HTML file
                        (minified and prefixed).";

                    });
                });

                r.elem("body", no_attr()).build(|r| {
                    r.elem("header", no_attr()).build(|r| {
                        r.elem("div", no_attr()).build(|r| {
                            self.home.title.render(r, 1, None);
                            self.home.subtitle.render(r, 2, None);
                            self.home.note.render(r);
                        });
                    });

                    r.elem("main", no_attr()).build(|r| {
                        r.elem("div", no_attr()).build(|r| {
                            self.benefits.render(r);
                            self.infos.render(r);
                            self.introduction.render(r);
                        });
                    });
                });
            });
    }
}

impl Introduction {
    fn render(&self, r: &mut Renderer) {
        let id = "introduction";
        r.elem("section", |r| {
            r.attr("id", id);
        })
            .build(|r| {
                r.elem("a", |r| {
                    r.attr("href", format!("#{}", id));
                }).build(|r| {
                    self.title.render(r, 3, Some(""));
                });

                r.elem("div", no_attr()).build(|r| {
                    r.elem("div", |r| {
                        r.attr("class", "left");
                    }).build(|r| {
                        for image in &self.images {
                            image.render(r, true);
                        }
                    });

                    r.elem("div", |r| {
                        r.attr("class", "right");
                    }).build(|r| {
                        self.text.render(r);
                    });
                });
            });
    }
}

impl Infos {
    fn render(&self, r: &mut Renderer) {
        let id = "infos";
        r.elem("section", |r| {
            r.attr("id", id);
        }).build(|r| {
            r.elem("a", |r| {
                r.attr("href", format!("#{}", id));
            }).build(|r| {
                self.title.render(r, 3, Some(""));
            });

            r.elem("div", no_attr()).build(|r| {
                self.hours.render(r);
                self.contacts.render(r);
            });
        });
    }
}

impl Contacts {
    fn render(&self, r: &mut Renderer) {
        r.elem("div", |r| {
            r.attr("class", "right");
        }).build(|r| {
            r.elem("address", no_attr()).build(|r| {
                r.elem("ul", no_attr()).build(|r| {
                    self.phone.render(r);
                    self.email.render(r);

                    let latitude = keep_decimals(6, &self.location.latitude);
                    let longitude = keep_decimals(6, &self.location.longitude);
                    let query = &self.location.query.join("+");

                    self.location.render(r, format!("https://maps.google.com/maps?ll={},{}&q={}", latitude, longitude, query));
                    self.location.render(r, format!("http://maps.google.com/maps?ll={},{}&q={}", latitude, longitude, query));

                    self.location.render(r, format!("https://maps.google.com/maps?ll={},{}", latitude, longitude));
                    self.location.render(r, format!("http://maps.google.com/maps?ll={},{}", latitude, longitude));
                });
            });
            self.socials.render(r);
        });
    }
}

fn keep_decimals(decimals: usize, number: &str) -> String {
    let (int, frac) = number.split_once('.').unwrap();
    format!("{}.{}", int, &frac[..decimals])
}

impl Location {
    fn render(&self, r: &mut Renderer, href: String) {
        r.elem("li", no_attr()).build(|r| {
            r.elem("a", |r| {
                r.attr("href", href);
            }).build(|r| {
                r.elem("pre", no_attr()).build(|r| {
                    r.elem("code", no_attr()).build(|r| {
                        r.put_raw(format!("{}\n{} {}", self.address, self.postcode, self.city));
                    });
                });
            });
        });
    }
}

impl Socials {
    fn render(&self, r: &mut Renderer) {
        r.elem("ul", |r| {
            r.attr("class", "socials");
        }).build(|r| {
            self.facebook.render(r, "facebook");
            self.instagram.render(r, "instagram");
        });
    }
}

impl Social {
    fn render(&self, r: &mut Renderer, name: &str) {
        if let Some(social) = &self.0 {
            r.elem("li", no_attr()).build(|r| {
                r.elem("a", |r| {
                    r.attr("href", https(&social));
                }).build(|r| {
                    r.single("img", |r| {
                        r.attr("src", format!("./res/images/{}.svg", name));
                        r.attr("alt", format!("page {}", name));
                    })
                });
            });
        }
    }
}

impl Hours {
    fn render(&self, r: &mut Renderer) {
        r.elem("table", |r| {
            r.attr("class", "left");
        }).build(|r| {
            r.elem("tbody", no_attr()).build(|r| {
                r.elem("tr", no_attr()).build(|r| {
                    r.elem("th", no_attr()).build(|_| {
                    });
                    r.elem("th", no_attr()).build(|r| {
                        r.put_raw("matin");
                    });
                    r.elem("th", no_attr()).build(|r| {
                        r.put_raw("après-midi");
                    });
                });

                self.monday.render(r, "lundi");
                self.tuesday.render(r, "mardi");
                self.wednesday.render(r, "mercredi");
                self.thursday.render(r, "jeudi");
                self.friday.render(r, "vendredi");
                self.saturday.render(r, "samedi");
                self.sunday.render(r, "dimanche");
            });
        });
    }
}

impl Day {
    fn render(&self, r: &mut Renderer, day: &str) {
        r.elem("tr", no_attr()).build(|r| {
            r.elem("th", no_attr()).build(|r| {
                r.put_raw(day);
            });
            self.0.0.render(r);
            self.0.1.render(r);
        });
    }
}

impl HalfDay {
    fn render(&self, r: &mut Renderer) {
        r.elem("td", no_attr()).build(|r| {
            if let Some((oh, om, ch, cm)) = self.0 {
                r.put_raw(format!("{oh:02}h{om:02} - {ch:02}h{cm:02}"));
            } else {
                r.elem("span", no_attr()).build(|r| {
                    r.put_raw("fermé");
                });
            }
        });
    }
}

impl Benefits {
    fn render(&self, r: &mut Renderer) {
        let id = "benefits";
        r.elem("section", |r| {
            r.attr("id", id);
        }).build(|r| {
            r.elem("a", |r| {
                r.attr("href", format!("#{}", id));
            }).build(|r| {
                self.title.render(r, 3, Some(""));
            });

            for (i, category) in self.categories.iter().enumerate() {
                category.render(r, i);
            }
        });
    }
}

impl Category {
    fn render(&self, r: &mut Renderer, i: usize) {
        let id = format!("category{}", i);
        r.elem("section", |r| {
            r.attr("id", &id);
        }).build(|r| {
            r.elem("a", |r| {
                r.attr("href", format!("#{}", id));
                r.attr("class", "row");
            }).build(|r| {
                if let Some(image) = &self.image {
                    image.render(r, false);
                }

                r.elem("div", no_attr()).build(|r| {
                    self.title.render(r, 4, Some("benefits"));
                    self.description.render(r);
                });
            });
            for benefit in &self.benefits {
                benefit.render(r);
            }
        });
    }
}

impl Benefit {
    fn render(&self, r: &mut Renderer) {
        r.elem("div", |r| {
            r.attr("class", "row");
        }).build(|r| {
            if let Some(image) = &self.image {
                image.render(r, false);
            }

            r.elem("div", no_attr()).build(|r| {
                self.title.render(r, 5, None);
                self.description.render(r);

                r.elem("div", no_attr()).build(|r| {
                    r.elem("div", no_attr()).build(|r| {
                        self.price.render(r);
                        self.book.render(r);
                    });
                });
            });
        });
    }
}

impl Book {
    fn render(&self, r: &mut Renderer) {
        r.elem("a", |r| {
            r.attr("href", https(&self.0));
            r.attr("class", "book");
        }).build(|r| {
            r.elem("span", no_attr()).build(|r| {
                r.put_raw("réserver");
            });
        });
    }
}

impl Email {
    fn render(&self, r: &mut Renderer) {
        r.elem("li", no_attr()).build(|r| {
            r.elem("a", |r| {
                r.attr("href", format!("mailto:{}", self.0));
            }).build(|r| {
                r.put_raw(&self.0);
            });
        });
    }
}

impl Image {
    fn render(&self, r: &mut Renderer, caption: bool) {
        r.elem("figure", no_attr()).build(|r| {
            r.single("img", |r| {
                r.attr("src", format!("./res/images/{}", self.0.0));
                r.attr("alt", &self.0.1);
            });

            if caption {
                r.elem("figcaption", no_attr()).build(|r| {
                    r.put_raw(&self.0.1);
                });
            }
        });
    }
}

impl Inline {
    fn render(&self, r: &mut Renderer) {
        r.put_raw_escapable(parse_md(false, &self.0));
    }
}

impl Note {
    fn render(&self, r: &mut Renderer) {
        if let Some(inline) = &self.0 {
            r.elem("p", |r| {
                r.attr("class", "note");
            })
            .build(|r| {
                inline.render(r);
            });
        }
    }
}

impl Phone {
    fn render(&self, r: &mut Renderer) {
        r.elem("li", no_attr()).build(|r| {
            r.elem("a", |r| {
                let tel: String = self.0.split_whitespace().collect();
                r.attr("href", format!("tel:{tel}"));
            }).build(|r| {
                r.put_raw(&self.0);
            });
        });
    }
}

impl Price {
    fn render(&self, r: &mut Renderer) {
        r.elem("span", |r| {
            r.attr("class", "price");
        }).build(|r| {
            r.elem("span", no_attr()).build(|r| {
                r.put_raw(self.0.0);
            });
            r.elem("span", no_attr()).build(|r| {
                r.put_raw('€');
            });
            r.elem("span", no_attr()).build(|r| {
                r.put_raw(format!("{:02}", self.0.1));
            });
        });
    }
}

impl Text {
    fn render(&self, r: &mut Renderer) {
        r.put_raw_escapable(parse_md(true, &self.0));
    }
}

impl Title {
    fn render(&self, r: &mut Renderer, level: u8, back: Option<&str>) {
        r.elem(format!("h{}", level), no_attr()).build(|r| {

            if let Some(id) = back {
                r.single("a", |r| {
                    r.attr("class", "back");
                    r.attr("href", format!("#{}", id));
                });

                r.elem("span", no_attr()).build(|r| {
                    self.0.render(r);
                });
            } else {
                self.0.render(r);
            }
        });
    }
}

// fn separator(r: &mut Renderer) {
//     r.elem("div", |r| {
//         r.attr("class", "separator");
//     })
// }

fn https(url: &str) -> String {
    format!("https://{url}")
}

fn parse_md(block: bool, input: &str) -> String {
    let parser = &mut MarkdownIt::new();

    if block {
        use cmark::block::*;
        paragraph::add(parser);
    }

    use cmark::inline::*;
    autolink::add(parser);
    backticks::add(parser);
    emphasis::add(parser);
    entity::add(parser);
    escape::add(parser);
    link::add(parser);

    parser
        .parse(input)
        .xrender()
        .trim()
        .to_string()
}
