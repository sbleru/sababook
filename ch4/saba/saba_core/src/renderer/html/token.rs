use crate::renderer::html::attribute::Attribute;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlToken {
    // 開始タグ
    StartTag {
        tag: String,
        self_closing: bool,
        attributes: Vec<Attribute>,
    },
    // 終了タグ
    EndTag {
        tag: String,
    },
    // 文字
    Char(char),
    // ファイルの終了（End Of File）
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    /// https://html.spec.whatwg.org/multipage/parsing.html#data-state
    Data,
    /// https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state
    TagOpen,
    /// https://html.spec.whatwg.org/multipage/parsing.html#end-tag-open-state
    EndTagOpen,
    /// https://html.spec.whatwg.org/multipage/parsing.html#tag-name-state
    TagName,
    /// https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-name-state
    BeforeAttributeName,
    /// https://html.spec.whatwg.org/multipage/parsing.html#attribute-name-state
    AttributeName,
    /// https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-name-state
    AfterAttributeName,
    /// https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-value-state
    BeforeAttributeValue,
    /// https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(double-quoted)-state
    AttributeValueDoubleQuoted,
    /// https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(single-quoted)-state
    AttributeValueSingleQuoted,
    /// https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(unquoted)-state
    AttributeValueUnquoted,
    /// https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-value-(quoted)-state
    AfterAttributeValueQuoted,
    /// https://html.spec.whatwg.org/multipage/parsing.html#self-closing-start-tag-state
    SelfClosingStartTag,
    /// https://html.spec.whatwg.org/multipage/parsing.html#script-data-state
    ScriptData,
    /// https://html.spec.whatwg.org/multipage/parsing.html#script-data-less-than-sign-state
    ScriptDataLessThanSign,
    /// https://html.spec.whatwg.org/multipage/parsing.html#script-data-end-tag-open-state
    ScriptDataEndTagOpen,
    /// https://html.spec.whatwg.org/multipage/parsing.html#script-data-end-tag-name-state
    ScriptDataEndTagName,
    /// https://html.spec.whatwg.org/multipage/parsing.html#temporary-buffer
    TemporaryBuffer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlTokenizer {
    state: State,
    pos: usize,
    reconsume: bool,
    latest_token: Option<HtmlToken>,
    input: Vec<char>,
    buf: String,
}

impl HtmlTokenizer {
    pub fn new(html: String) -> Self {
        Self {
            state: State::Data,
            pos: 0,
            reconsume: false,
            latest_token: None,
            input: html.chars().collect(),
            buf: String::new(),
        }
    }

    fn is_eof(&self) -> bool {
        self.pos > self.input.len()
    }

    fn consume_next_input(&mut self) -> char {
        let c = self.input[self.pos];
        self.pos += 1;
        c
    }

    fn reconsume_input(&mut self) -> char {
        self.reconsume = false;
        self.input[self.pos - 1]
    }

    fn create_tag(&mut self, start_tag_token: bool) {
        if start_tag_token {
            self.latest_token = Some(HtmlToken::StartTag {
                tag: String::new(),
                self_closing: false,
                attributes: Vec::new(),
            });
        } else {
            self.latest_token = Some(HtmlToken::EndTag { tag: String::new() });
        }
    }

    fn append_tag_name(&mut self, c: char) {
        assert!(self.latest_token.is_some());

        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    ref mut tag,
                    self_closing: _,
                    attributes: _,
                }
                | HtmlToken::EndTag { ref mut tag } => tag.push(c),
                _ => panic!("`latest_token` should be either StartTag or EndTag"),
            }
        }
    }

    fn take_latest_token(&mut self) -> Option<HtmlToken> {
        assert!(self.latest_token.is_some());

        let t = self.latest_token.as_ref().cloned();
        self.latest_token = None;
        assert!(self.latest_token.is_none());

        t
    }

    fn start_new_attribute(&mut self) {
        assert!(self.latest_token.is_some());

        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    tag: _,
                    self_closing: _,
                    ref mut attributes,
                } => {
                    attributes.push(Attribute::new());
                }
                _ => panic!("`latest_token` should be either StartTag"),
            }
        }
    }

    fn append_attribute(&mut self, c: char, is_name: bool) {
        assert!(self.latest_token.is_some());

        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    tag: _,
                    self_closing: _,
                    ref mut attributes,
                } => {
                    let len = attributes.len();
                    assert!(len > 0);

                    attributes[len - 1].add_char(c, is_name);
                }
                _ => panic!("`latest_token` should be either StartTag"),
            }
        }
    }

    fn set_self_closing_flag(&mut self) {
        assert!(self.latest_token.is_some());

        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    tag: _,
                    ref mut self_closing,
                    attributes: _,
                } => *self_closing = true,
                _ => panic!("`latest_token` should be either StartTag"),
            }
        }
    }
}

impl Iterator for HtmlTokenizer {
    type Item = HtmlToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }

        loop {
            let c = match self.reconsume {
                true => self.reconsume_input(),
                false => self.consume_next_input(),
            };

            match self.state {
                State::Data => {
                    if c == '<' {
                        self.state = State::TagOpen;
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    return Some(HtmlToken::Char(c));
                }
                State::TagOpen => {
                    if c == '/' {
                        self.state = State::EndTagOpen;
                        continue;
                    }

                    if c.is_ascii_alphabetic() {
                        self.reconsume = true;
                        self.state = State::TagName;
                        self.create_tag(true);
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.reconsume = true;
                    self.state = State::Data;
                }
                State::EndTagOpen => {
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    if c.is_ascii_alphabetic() {
                        self.reconsume = true;
                        self.state = State::TagName;
                        self.create_tag(false);
                        continue;
                    }
                }
                State::TagName => {
                    if c == ' ' {
                        self.state = State::BeforeAttributeName;
                        continue;
                    }

                    if c == '/' {
                        self.state = State::SelfClosingStartTag;
                        continue;
                    }

                    if c == '>' {
                        self.state = State::Data;
                        return self.take_latest_token();
                    }

                    if c.is_ascii_uppercase() {
                        self.append_tag_name(c.to_ascii_lowercase());
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.append_tag_name(c);
                }

                State::BeforeAttributeName => {
                    if c == '/' || c == '>' || self.is_eof() {
                        self.reconsume = true;
                        self.state = State::AfterAttributeName;
                        continue;
                    }

                    self.reconsume = true;
                    self.state = State::AttributeName;
                    self.start_new_attribute();
                }
                State::AttributeName => {
                    if c == ' ' || c == '/' || c == '>' || self.is_eof() {
                        self.reconsume = true;
                        self.state = State::AfterAttributeName;
                        continue;
                    }

                    if c == '=' {
                        self.state = State::BeforeAttributeValue;
                        continue;
                    }

                    if c.is_ascii_uppercase() {
                        self.append_attribute(c.to_ascii_lowercase(), /*is_name*/ true);
                        continue;
                    }

                    self.append_attribute(c, /*is_name*/ true);
                }
                State::AfterAttributeName => {
                    if c == ' ' {
                        // 空白文字は無視する
                        continue;
                    }

                    if c == '/' {
                        self.state = State::SelfClosingStartTag;
                        continue;
                    }

                    if c == '=' {
                        self.state = State::BeforeAttributeValue;
                        continue;
                    }

                    if c == '>' {
                        self.state = State::Data;
                        return self.take_latest_token();
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.reconsume = true;
                    self.state = State::AttributeName;
                    self.start_new_attribute();
                }
                State::BeforeAttributeValue => {
                    if c == ' ' {
                        // 空白文字は無視する
                        continue;
                    }

                    if c == '"' {
                        self.state = State::AttributeValueDoubleQuoted;
                        continue;
                    }

                    if c == '\'' {
                        self.state = State::AttributeValueSingleQuoted;
                        continue;
                    }

                    self.reconsume = true;
                    self.state = State::AttributeValueUnquoted;
                }
                State::AttributeValueDoubleQuoted => {
                    if c == '"' {
                        self.state = State::AfterAttributeValueQuoted;
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.append_attribute(c, /*is_name*/ false);
                }
                State::AttributeValueSingleQuoted => {
                    if c == '\'' {
                        self.state = State::AfterAttributeValueQuoted;
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.append_attribute(c, /*is_name*/ false);
                }
                State::AttributeValueUnquoted => {
                    if c == ' ' {
                        self.state = State::BeforeAttributeName;
                        continue;
                    }

                    if c == '>' {
                        self.state = State::Data;
                        return self.take_latest_token();
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.append_attribute(c, /*is_name*/ false);
                }
                State::AfterAttributeValueQuoted => {
                    if c == ' ' {
                        self.state = State::BeforeAttributeName;
                        continue;
                    }

                    if c == '/' {
                        self.state = State::SelfClosingStartTag;
                        continue;
                    }

                    if c == '>' {
                        self.state = State::Data;
                        return self.take_latest_token();
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.reconsume = true;
                    self.state = State::BeforeAttributeName;
                }
                State::SelfClosingStartTag => {
                    if c == '>' {
                        self.set_self_closing_flag();
                        self.state = State::Data;
                        return self.take_latest_token();
                    }

                    if self.is_eof() {
                        // invalid parse error.
                        return Some(HtmlToken::Eof);
                    }
                }
                State::ScriptData => {
                    if c == '<' {
                        self.state = State::ScriptDataLessThanSign;
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    return Some(HtmlToken::Char(c));
                }
                State::ScriptDataLessThanSign => {
                    if c == '/' {
                        // 一時的なバッファを空文字でリセットする
                        self.buf = String::new();
                        self.state = State::ScriptDataEndTagOpen;
                        continue;
                    }

                    self.reconsume = true;
                    self.state = State::ScriptData;
                    return Some(HtmlToken::Char('<'));
                }
                State::ScriptDataEndTagOpen => {
                    if c.is_ascii_alphabetic() {
                        self.reconsume = true;
                        self.state = State::ScriptDataEndTagName;
                        self.create_tag(false);
                        continue;
                    }

                    self.reconsume = true;
                    self.state = State::ScriptData;
                    // 仕様では、"<"と"/"の2つの文字トークンを返すとなっているが、
                    // 私たちの実装ではnextメソッドからは一つのトークンしか返せない
                    // ため、"<"のトークンのみを返す
                    return Some(HtmlToken::Char('<'));
                }
                State::ScriptDataEndTagName => {
                    if c == '>' {
                        self.state = State::Data;
                        return self.take_latest_token();
                    }

                    if c.is_ascii_alphabetic() {
                        self.buf.push(c);
                        self.append_tag_name(c.to_ascii_lowercase());
                        continue;
                    }

                    self.state = State::TemporaryBuffer;
                    self.buf = String::from("</") + &self.buf;
                    self.buf.push(c);
                    continue;
                }
                State::TemporaryBuffer => {
                    self.reconsume = true;

                    if self.buf.chars().count() == 0 {
                        self.state = State::ScriptData;
                        continue;
                    }

                    // remove the first char
                    let c = self
                        .buf
                        .chars()
                        .nth(0)
                        .expect("self.buf should have at least 1 char");
                    self.buf.remove(0);
                    return Some(HtmlToken::Char(c));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn test_empty() {
        let html = "".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        assert!(tokenizer.next().is_none());
    }

    #[test]
    fn test_start_and_end_tag() {
        let html = "<body></body>".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        let expected = [
            HtmlToken::StartTag {
                tag: "body".to_string(),
                self_closing: false,
                attributes: Vec::new(),
            },
            HtmlToken::EndTag {
                tag: "body".to_string(),
            },
        ];
        for e in expected {
            assert_eq!(Some(e), tokenizer.next());
        }
    }

    #[test]
    fn test_attributes() {
        let html = "<p class=\"A\" id='B' foo=bar></p>".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        let mut attr1 = Attribute::new();
        attr1.add_char('c', true);
        attr1.add_char('l', true);
        attr1.add_char('a', true);
        attr1.add_char('s', true);
        attr1.add_char('s', true);
        attr1.add_char('A', false);

        let mut attr2 = Attribute::new();
        attr2.add_char('i', true);
        attr2.add_char('d', true);
        attr2.add_char('B', false);

        let mut attr3 = Attribute::new();
        attr3.add_char('f', true);
        attr3.add_char('o', true);
        attr3.add_char('o', true);
        attr3.add_char('b', false);
        attr3.add_char('a', false);
        attr3.add_char('r', false);

        let expected = [
            HtmlToken::StartTag {
                tag: "p".to_string(),
                self_closing: false,
                attributes: vec![attr1, attr2, attr3],
            },
            HtmlToken::EndTag {
                tag: "p".to_string(),
            },
        ];
        for e in expected {
            assert_eq!(Some(e), tokenizer.next());
        }
    }

    #[test]
    fn test_self_closing_tag() {
        let html = "<img />".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        let expected = [HtmlToken::StartTag {
            tag: "img".to_string(),
            self_closing: true,
            attributes: Vec::new(),
        }];
        for e in expected {
            assert_eq!(Some(e), tokenizer.next());
        }
    }

    #[test]
    fn test_script_tag() {
        let html = "<script>js code;</script>".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        let expected = [
            HtmlToken::StartTag {
                tag: "script".to_string(),
                self_closing: false,
                attributes: Vec::new(),
            },
            HtmlToken::Char('j'),
            HtmlToken::Char('s'),
            HtmlToken::Char(' '),
            HtmlToken::Char('c'),
            HtmlToken::Char('o'),
            HtmlToken::Char('d'),
            HtmlToken::Char('e'),
            HtmlToken::Char(';'),
            HtmlToken::EndTag {
                tag: "script".to_string(),
            },
        ];
        for e in expected {
            assert_eq!(Some(e), tokenizer.next());
        }
    }

    // main.rsで使用されるhtmlのテスト
    #[test]
    fn test_complete_html_document() {
        let html = r#"<html>
<head></head>
<body>
  <h1 id="title">H1 title</h1>
  <h2 class="class">H2 title</h2>
  <p>Test text.</p>
  <p>
    <a href="example.com">Link1</a>
    <a href="example.com">Link2</a>
  </p>
</body>
</html>"#.to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        
        // id属性の設定
        let mut id_attr = Attribute::new();
        id_attr.add_char('i', true);
        id_attr.add_char('d', true);
        id_attr.add_char('t', false);
        id_attr.add_char('i', false);
        id_attr.add_char('t', false);
        id_attr.add_char('l', false);
        id_attr.add_char('e', false);

        // class属性の設定
        let mut class_attr = Attribute::new();
        class_attr.add_char('c', true);
        class_attr.add_char('l', true);
        class_attr.add_char('a', true);
        class_attr.add_char('s', true);
        class_attr.add_char('s', true);
        class_attr.add_char('c', false);
        class_attr.add_char('l', false);
        class_attr.add_char('a', false);
        class_attr.add_char('s', false);
        class_attr.add_char('s', false);

        // href属性の設定
        let mut href_attr1 = Attribute::new();
        href_attr1.add_char('h', true);
        href_attr1.add_char('r', true);
        href_attr1.add_char('e', true);
        href_attr1.add_char('f', true);
        href_attr1.add_char('e', false);
        href_attr1.add_char('x', false);
        href_attr1.add_char('a', false);
        href_attr1.add_char('m', false);
        href_attr1.add_char('p', false);
        href_attr1.add_char('l', false);
        href_attr1.add_char('e', false);
        href_attr1.add_char('.', false);
        href_attr1.add_char('c', false);
        href_attr1.add_char('o', false);
        href_attr1.add_char('m', false);

        let mut href_attr2 = Attribute::new();
        href_attr2.add_char('h', true);
        href_attr2.add_char('r', true);
        href_attr2.add_char('e', true);
        href_attr2.add_char('f', true);
        href_attr2.add_char('e', false);
        href_attr2.add_char('x', false);
        href_attr2.add_char('a', false);
        href_attr2.add_char('m', false);
        href_attr2.add_char('p', false);
        href_attr2.add_char('l', false);
        href_attr2.add_char('e', false);
        href_attr2.add_char('.', false);
        href_attr2.add_char('c', false);
        href_attr2.add_char('o', false);
        href_attr2.add_char('m', false);

        // 期待されるトークンの配列
        let expected_tokens = vec![
            // <html>
            HtmlToken::StartTag {
                tag: "html".to_string(),
                self_closing: false,
                attributes: Vec::new(),
            },
            HtmlToken::Char('\n'),
            // <head></head>
            HtmlToken::StartTag {
                tag: "head".to_string(),
                self_closing: false,
                attributes: Vec::new(),
            },
            HtmlToken::EndTag {
                tag: "head".to_string(),
            },
            HtmlToken::Char('\n'),
            // <body>
            HtmlToken::StartTag {
                tag: "body".to_string(),
                self_closing: false,
                attributes: Vec::new(),
            },
            HtmlToken::Char('\n'),
            HtmlToken::Char(' '),
            HtmlToken::Char(' '),
            // <h1 id="title">H1 title</h1>
            HtmlToken::StartTag {
                tag: "h1".to_string(),
                self_closing: false,
                attributes: vec![id_attr],
            },
            HtmlToken::Char('H'), HtmlToken::Char('1'), HtmlToken::Char(' '),
            HtmlToken::Char('t'), HtmlToken::Char('i'), HtmlToken::Char('t'),
            HtmlToken::Char('l'), HtmlToken::Char('e'),
            HtmlToken::EndTag {
                tag: "h1".to_string(),
            },
            HtmlToken::Char('\n'),
            HtmlToken::Char(' '),
            HtmlToken::Char(' '),
            // <h2 class="class">H2 title</h2>
            HtmlToken::StartTag {
                tag: "h2".to_string(),
                self_closing: false,
                attributes: vec![class_attr],
            },
            HtmlToken::Char('H'), HtmlToken::Char('2'), HtmlToken::Char(' '),
            HtmlToken::Char('t'), HtmlToken::Char('i'), HtmlToken::Char('t'),
            HtmlToken::Char('l'), HtmlToken::Char('e'),
            HtmlToken::EndTag {
                tag: "h2".to_string(),
            },
            HtmlToken::Char('\n'),
            HtmlToken::Char(' '),
            HtmlToken::Char(' '),
            // <p>Test text.</p>
            HtmlToken::StartTag {
                tag: "p".to_string(),
                self_closing: false,
                attributes: Vec::new(),
            },
            HtmlToken::Char('T'), HtmlToken::Char('e'), HtmlToken::Char('s'),
            HtmlToken::Char('t'), HtmlToken::Char(' '), HtmlToken::Char('t'),
            HtmlToken::Char('e'), HtmlToken::Char('x'), HtmlToken::Char('t'),
            HtmlToken::Char('.'),
            HtmlToken::EndTag {
                tag: "p".to_string(),
            },
            HtmlToken::Char('\n'),
            HtmlToken::Char(' '),
            HtmlToken::Char(' '),
            // <p>
            HtmlToken::StartTag {
                tag: "p".to_string(),
                self_closing: false,
                attributes: Vec::new(),
            },
            HtmlToken::Char('\n'),
            HtmlToken::Char(' '),
            HtmlToken::Char(' '),
            HtmlToken::Char(' '),
            HtmlToken::Char(' '),
            // <a href="example.com">Link1</a>
            HtmlToken::StartTag {
                tag: "a".to_string(),
                self_closing: false,
                attributes: vec![href_attr1],
            },
            HtmlToken::Char('L'), HtmlToken::Char('i'), HtmlToken::Char('n'),
            HtmlToken::Char('k'), HtmlToken::Char('1'),
            HtmlToken::EndTag {
                tag: "a".to_string(),
            },
            HtmlToken::Char('\n'),
            HtmlToken::Char(' '),
            HtmlToken::Char(' '),
            HtmlToken::Char(' '),
            HtmlToken::Char(' '),
            // <a href="example.com">Link2</a>
            HtmlToken::StartTag {
                tag: "a".to_string(),
                self_closing: false,
                attributes: vec![href_attr2],
            },
            HtmlToken::Char('L'), HtmlToken::Char('i'), HtmlToken::Char('n'),
            HtmlToken::Char('k'), HtmlToken::Char('2'),
            HtmlToken::EndTag {
                tag: "a".to_string(),
            },
            HtmlToken::Char('\n'),
            HtmlToken::Char(' '),
            HtmlToken::Char(' '),
            // </p>
            HtmlToken::EndTag {
                tag: "p".to_string(),
            },
            HtmlToken::Char('\n'),
            // </body>
            HtmlToken::EndTag {
                tag: "body".to_string(),
            },
            HtmlToken::Char('\n'),
            // </html>
            HtmlToken::EndTag {
                tag: "html".to_string(),
            },
        ];

        for expected in expected_tokens {
            let actual = tokenizer.next();
            assert_eq!(Some(expected), actual);
        }
    }

    #[test]
    fn test_mixed_self_closing_and_regular_tags() {
        let html = r#"<div><img src="test.jpg" /><p>Text</p><br/></div>"#.to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        
        // img src属性
        let mut src_attr = Attribute::new();
        src_attr.add_char('s', true);
        src_attr.add_char('r', true);
        src_attr.add_char('c', true);
        src_attr.add_char('t', false);
        src_attr.add_char('e', false);
        src_attr.add_char('s', false);
        src_attr.add_char('t', false);
        src_attr.add_char('.', false);
        src_attr.add_char('j', false);
        src_attr.add_char('p', false);
        src_attr.add_char('g', false);

        let expected_tokens = vec![
            // <div>
            HtmlToken::StartTag {
                tag: "div".to_string(),
                self_closing: false,
                attributes: Vec::new(),
            },
            // <img src="test.jpg" />
            HtmlToken::StartTag {
                tag: "img".to_string(),
                self_closing: true,
                attributes: vec![src_attr],
            },
            // <p>
            HtmlToken::StartTag {
                tag: "p".to_string(),
                self_closing: false,
                attributes: Vec::new(),
            },
            // Text
            HtmlToken::Char('T'),
            HtmlToken::Char('e'),
            HtmlToken::Char('x'),
            HtmlToken::Char('t'),
            // </p>
            HtmlToken::EndTag {
                tag: "p".to_string(),
            },
            // <br/>
            HtmlToken::StartTag {
                tag: "br".to_string(),
                self_closing: true,
                attributes: Vec::new(),
            },
            // </div>
            HtmlToken::EndTag {
                tag: "div".to_string(),
            },
        ];

        for expected in expected_tokens {
            let actual = tokenizer.next();
            assert_eq!(Some(expected), actual);
        }
    }
}
