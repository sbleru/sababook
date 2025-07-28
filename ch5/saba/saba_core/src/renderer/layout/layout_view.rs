use crate::constants::CONTENT_AREA_WIDTH;
use crate::display_item::DisplayItem;
use crate::renderer::css::cssom::StyleSheet;
use crate::renderer::dom::api::get_target_element_node;
use crate::renderer::dom::node::ElementKind;
use crate::renderer::dom::node::Node;
use crate::renderer::layout::layout_object::create_layout_object;
use crate::renderer::layout::layout_object::LayoutObject;
use crate::renderer::layout::layout_object::LayoutObjectKind;
use crate::renderer::layout::layout_object::LayoutPoint;
use crate::renderer::layout::layout_object::LayoutSize;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

fn build_layout_tree(
    node: &Option<Rc<RefCell<Node>>>,
    parent_obj: &Option<Rc<RefCell<LayoutObject>>>,
    cssom: &StyleSheet,
) -> Option<Rc<RefCell<LayoutObject>>> {
    // `create_layout_object`関数によって、ノードとなるLayoutObjectの作成を試みる。
    // CSSによって"display:none"が指定されていた場合、ノードは作成されない
    let mut target_node = node.clone();
    let mut layout_object = create_layout_object(node, parent_obj, cssom);
    // もしノードが作成されなかった場合、DOMノードの兄弟ノードを使用してLayoutObjectの
    // 作成を試みる。LayoutObjectが作成されるまで、兄弟ノードを辿り続ける
    while layout_object.is_none() {
        if let Some(n) = target_node {
            target_node = n.borrow().next_sibling().clone();
            layout_object = create_layout_object(&target_node, parent_obj, cssom);
        } else {
            // もし兄弟ノードがない場合、処理するべきDOMツリーは終了したので、今まで
            // 作成したレイアウトツリーを返す
            return layout_object;
        }
    }

    if let Some(n) = target_node {
        let original_first_child = n.borrow().first_child();
        let original_next_sibling = n.borrow().next_sibling();
        let mut first_child = build_layout_tree(&original_first_child, &layout_object, cssom);
        let mut next_sibling = build_layout_tree(&original_next_sibling, &None, cssom);

        // もし子ノードに"display:node"が指定されていた場合、LayoutObjectは作成され
        // ないため、子ノードの兄弟ノードを使用してLayoutObjectの作成を試みる。
        // LayoutObjectが作成されるか、辿るべき兄弟ノードがなくなるまで処理を繰り返す
        if first_child.is_none() && original_first_child.is_some() {
            let mut original_dom_node = original_first_child
                .expect("first child should exist")
                .borrow()
                .next_sibling();

            loop {
                first_child = build_layout_tree(&original_dom_node, &layout_object, cssom);

                if first_child.is_none() && original_dom_node.is_some() {
                    original_dom_node = original_dom_node
                        .expect("next sibling should exist")
                        .borrow()
                        .next_sibling();
                    continue;
                }

                break;
            }
        }

        // もし兄弟ノードに"display:node"が指定されていた場合、LayoutObjectは作成され
        // ないため、兄弟ノードの兄弟ノードを使用してLayoutObjectの作成を試みる。
        // LayoutObjectが作成されるか、辿るべき兄弟ノードがなくなるまで処理を繰り返す
        if next_sibling.is_none() && n.borrow().next_sibling().is_some() {
            let mut original_dom_node = original_next_sibling
                .expect("first child should exist")
                .borrow()
                .next_sibling();

            loop {
                next_sibling = build_layout_tree(&original_dom_node, &None, cssom);

                if next_sibling.is_none() && original_dom_node.is_some() {
                    original_dom_node = original_dom_node
                        .expect("next sibling should exist")
                        .borrow()
                        .next_sibling();
                    continue;
                }

                break;
            }
        }

        let obj = match layout_object {
            Some(ref obj) => obj,
            None => panic!("render object should exist here"),
        };
        obj.borrow_mut().set_first_child(first_child);
        obj.borrow_mut().set_next_sibling(next_sibling);
    }

    layout_object
}

#[derive(Debug, Clone)]
pub struct LayoutView {
    root: Option<Rc<RefCell<LayoutObject>>>,
}

impl LayoutView {
    pub fn new(root: Rc<RefCell<Node>>, cssom: &StyleSheet) -> Self {
        // レイアウトツリーは描画される要素だけを持つツリーなので、<body>タグを取得し、その子要素以下を
        // レイアウトツリーのノードに変換する。
        let body_root = get_target_element_node(Some(root), ElementKind::Body);

        let mut tree = Self {
            root: build_layout_tree(&body_root, &None, cssom),
        };

        tree.update_layout();

        tree
    }

    fn calculate_node_size(node: &Option<Rc<RefCell<LayoutObject>>>, parent_size: LayoutSize) {
        if let Some(n) = node {
            // ノードがブロック要素の場合、子ノードのレイアウトを計算する前に横幅を決める
            if n.borrow().kind() == LayoutObjectKind::Block {
                n.borrow_mut().compute_size(parent_size);
            }

            let first_child = n.borrow().first_child();
            Self::calculate_node_size(&first_child, n.borrow().size());

            let next_sibling = n.borrow().next_sibling();
            Self::calculate_node_size(&next_sibling, parent_size);

            // 子ノードのサイズが決まった後にサイズを計算する。
            // ブロック要素のとき、高さは子ノードの高さに依存する
            // インライン要素のとき、高さも横幅も子ノードに依存する
            n.borrow_mut().compute_size(parent_size);
        }
    }

    fn calculate_node_position(
        node: &Option<Rc<RefCell<LayoutObject>>>,
        parent_point: LayoutPoint,
        previous_sibling_kind: LayoutObjectKind,
        previous_sibling_point: Option<LayoutPoint>,
        previous_sibling_size: Option<LayoutSize>,
    ) {
        if let Some(n) = node {
            n.borrow_mut().compute_position(
                parent_point,
                previous_sibling_kind,
                previous_sibling_point,
                previous_sibling_size,
            );

            // ノード（node）の子ノードの位置を計算をする
            let first_child = n.borrow().first_child();
            Self::calculate_node_position(
                &first_child,
                n.borrow().point(),
                LayoutObjectKind::Block,
                None,
                None,
            );

            // ノード（node）の兄弟ノードの位置を計算する
            let next_sibling = n.borrow().next_sibling();
            Self::calculate_node_position(
                &next_sibling,
                parent_point,
                n.borrow().kind(),
                Some(n.borrow().point()),
                Some(n.borrow().size()),
            );
        }
    }

    fn update_layout(&mut self) {
        Self::calculate_node_size(&self.root, LayoutSize::new(CONTENT_AREA_WIDTH, 0));

        Self::calculate_node_position(
            &self.root,
            LayoutPoint::new(0, 0),
            LayoutObjectKind::Block,
            None,
            None,
        );
    }

    fn paint_node(node: &Option<Rc<RefCell<LayoutObject>>>, display_items: &mut Vec<DisplayItem>) {
        match node {
            Some(n) => {
                display_items.extend(n.borrow_mut().paint());

                let first_child = n.borrow().first_child();
                Self::paint_node(&first_child, display_items);

                let next_sibling = n.borrow().next_sibling();
                Self::paint_node(&next_sibling, display_items);
            }
            None => (),
        }
    }

    pub fn paint(&self) -> Vec<DisplayItem> {
        let mut display_items = Vec::new();

        Self::paint_node(&self.root, &mut display_items);

        display_items
    }

    pub fn root(&self) -> Option<Rc<RefCell<LayoutObject>>> {
        self.root.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alloc::string::ToString;
    use crate::renderer::css::cssom::CssParser;
    use crate::renderer::css::token::CssTokenizer;
    use crate::renderer::dom::api::get_style_content;
    use crate::renderer::dom::node::Element;
    use crate::renderer::dom::node::NodeKind;
    use crate::renderer::html::parser::HtmlParser;
    use crate::renderer::html::token::HtmlTokenizer;
    use crate::renderer::layout::computed_style::{Color, FontSize};
    use alloc::string::String;
    use alloc::vec::Vec;

    fn create_layout_view(html: String) -> LayoutView {
        let t = HtmlTokenizer::new(html);
        let window = HtmlParser::new(t).construct_tree();
        let dom = window.borrow().document();
        let style = get_style_content(dom.clone());
        let css_tokenizer = CssTokenizer::new(style);
        let cssom = CssParser::new(css_tokenizer).parse_stylesheet();
        LayoutView::new(dom, &cssom)
    }

    #[test]
    fn test_empty() {
        let layout_view = create_layout_view("".to_string());
        assert_eq!(None, layout_view.root());
    }

    #[test]
    fn test_body() {
        let html = "<html><head></head><body></body></html>".to_string();
        let layout_view = create_layout_view(html);

        let root = layout_view.root();
        assert!(root.is_some());
        assert_eq!(
            LayoutObjectKind::Block,
            root.clone().expect("root should exist").borrow().kind()
        );
        assert_eq!(
            NodeKind::Element(Element::new("body", Vec::new())),
            root.clone()
                .expect("root should exist")
                .borrow()
                .node_kind()
        );
    }

    #[test]
    fn test_text() {
        let html = "<html><head></head><body>text</body></html>".to_string();
        let layout_view = create_layout_view(html);

        let root = layout_view.root();
        assert!(root.is_some());
        assert_eq!(
            LayoutObjectKind::Block,
            root.clone().expect("root should exist").borrow().kind()
        );
        assert_eq!(
            NodeKind::Element(Element::new("body", Vec::new())),
            root.clone()
                .expect("root should exist")
                .borrow()
                .node_kind()
        );

        let text = root.expect("root should exist").borrow().first_child();
        assert!(text.is_some());
        assert_eq!(
            LayoutObjectKind::Text,
            text.clone()
                .expect("text node should exist")
                .borrow()
                .kind()
        );
        assert_eq!(
            NodeKind::Text("text".to_string()),
            text.clone()
                .expect("text node should exist")
                .borrow()
                .node_kind()
        );
    }

    #[test]
    fn test_display_none() {
        let html = "<html><head><style>body{display:none;}</style></head><body>text</body></html>"
            .to_string();
        let layout_view = create_layout_view(html);

        assert_eq!(None, layout_view.root());
    }

    #[test]
    fn test_hidden_class() {
        let html = r#"<html>
<head>
<style>
  .hidden {
    display: none;
  }
</style>
</head>
<body>
  <a class="hidden">link1</a>
  <p></p>
  <p class="hidden"><a>link2</a></p>
</body>
</html>"#
            .to_string();
        let layout_view = create_layout_view(html);

        let root = layout_view.root();
        assert!(root.is_some());
        assert_eq!(
            LayoutObjectKind::Block,
            root.clone().expect("root should exist").borrow().kind()
        );
        assert_eq!(
            NodeKind::Element(Element::new("body", Vec::new())),
            root.clone()
                .expect("root should exist")
                .borrow()
                .node_kind()
        );

        let p = root.expect("root should exist").borrow().first_child();
        assert!(p.is_some());
        assert_eq!(
            LayoutObjectKind::Block,
            p.clone().expect("p node should exist").borrow().kind()
        );
        assert_eq!(
            NodeKind::Element(Element::new("p", Vec::new())),
            p.clone().expect("p node should exist").borrow().node_kind()
        );

        assert!(p
            .clone()
            .expect("p node should exist")
            .borrow()
            .first_child()
            .is_none());
        assert!(p
            .expect("p node should exist")
            .borrow()
            .next_sibling()
            .is_none());
    }

    #[test]
    fn test_background_color_by_name() {
        let html = r#"<html>
<head>
<style>
  body { background-color: red; }
</style>
</head>
<body>test</body>
</html>"#
            .to_string();
        let layout_view = create_layout_view(html);

        let root = layout_view.root();
        assert!(root.is_some());
        
        let style = root.expect("root should exist").borrow().style();
        assert_eq!(
            Color::from_name("red").unwrap(),
            style.background_color()
        );
    }

    #[test]
    fn test_background_color_by_code() {
        let html = r#"<html>
<head>
<style>
  body { background-color: #ff0000; }
</style>
</head>
<body>test</body>
</html>"#
            .to_string();
        let layout_view = create_layout_view(html);

        let root = layout_view.root();
        assert!(root.is_some());
        
        let style = root.expect("root should exist").borrow().style();
        assert_eq!(
            Color::from_name("red").unwrap(),
            style.background_color()
        );
    }

    #[test]
    fn test_text_color_by_name() {
        let html = r#"<html>
<head>
<style>
  body { color: blue; }
</style>
</head>
<body>test</body>
</html>"#
            .to_string();
        let layout_view = create_layout_view(html);

        let root = layout_view.root();
        assert!(root.is_some());
        
        let style = root.expect("root should exist").borrow().style();
        assert_eq!(
            Color::from_name("blue").unwrap(),
            style.color()
        );
    }

    #[test]
    fn test_text_color_by_code() {
        let html = r#"<html>
<head>
<style>
  body { color: #0000ff; }
</style>
</head>
<body>test</body>
</html>"#
            .to_string();
        let layout_view = create_layout_view(html);

        let root = layout_view.root();
        assert!(root.is_some());
        
        let style = root.expect("root should exist").borrow().style();
        assert_eq!(
            Color::from_name("blue").unwrap(),
            style.color()
        );
    }

    #[test]
    fn test_display_inline() {
        let html = r#"<html>
<head>
<style>
  p { display: inline; }
</style>
</head>
<body><p>inline text</p></body>
</html>"#
            .to_string();
        let layout_view = create_layout_view(html);

        let root = layout_view.root();
        assert!(root.is_some());
        
        let p = root.expect("root should exist").borrow().first_child();
        assert!(p.is_some());
        assert_eq!(
            LayoutObjectKind::Inline,
            p.expect("p node should exist").borrow().kind()
        );
    }

    #[test]
    fn test_display_block() {
        let html = r#"<html>
<head>
<style>
  a { display: block; }
</style>
</head>
<body><a>block link</a></body>
</html>"#
            .to_string();
        let layout_view = create_layout_view(html);

        let root = layout_view.root();
        assert!(root.is_some());
        
        let a = root.expect("root should exist").borrow().first_child();
        assert!(a.is_some());
        assert_eq!(
            LayoutObjectKind::Block,
            a.expect("a node should exist").borrow().kind()
        );
    }

    #[test]
    fn test_multiple_css_properties() {
        let html = r#"<html>
<head>
<style>
  .styled {
    background-color: red;
    color: white;
    display: block;
  }
</style>
</head>
<body><p class="styled">styled text</p></body>
</html>"#
            .to_string();
        let layout_view = create_layout_view(html);

        let root = layout_view.root();
        assert!(root.is_some());
        
        let p = root.expect("root should exist").borrow().first_child();
        assert!(p.is_some());
        let p_ref = p.expect("p should exist");
        
        // pはblock要素として設定されているはず
        assert_eq!(LayoutObjectKind::Block, p_ref.borrow().kind());
        
        let style = p_ref.borrow().style();
        assert_eq!(
            Color::from_name("red").unwrap(),
            style.background_color()
        );
        assert_eq!(
            Color::white(),
            style.color()
        );
    }

    #[test]
    fn test_id_selector() {
        let html = r#"<html>
<head>
<style>
  #special { background-color: green; }
</style>
</head>
<body><p id="special">special paragraph</p></body>
</html>"#
            .to_string();
        let layout_view = create_layout_view(html);

        let root = layout_view.root();
        assert!(root.is_some());
        
        let p = root.expect("root should exist").borrow().first_child();
        assert!(p.is_some());
        
        let style = p.expect("p should exist").borrow().style();
        assert_eq!(
            Color::from_name("green").unwrap(),
            style.background_color()
        );
    }

    #[test]
    fn test_type_selector() {
        let html = r#"<html>
<head>
<style>
  p { color: red; }
  h1 { background-color: yellow; }
</style>
</head>
<body>
  <p>paragraph</p>
  <h1>heading</h1>
</body>
</html>"#
            .to_string();
        let layout_view = create_layout_view(html);

        let root = layout_view.root();
        assert!(root.is_some());
        
        let p = root.clone().expect("root should exist").borrow().first_child();
        assert!(p.is_some());
        let p_style = p.expect("p should exist").borrow().style();
        assert_eq!(
            Color::from_name("red").unwrap(),
            p_style.color()
        );
        
        let h1 = root.expect("root should exist").borrow().first_child()
            .expect("first child should exist").borrow().next_sibling();
        assert!(h1.is_some());
        let h1_style = h1.expect("h1 should exist").borrow().style();
        assert_eq!(
            Color::from_name("yellow").unwrap(),
            h1_style.background_color()
        );
    }

    #[test]
    fn test_css_inheritance() {
        let html = r#"<html>
<head>
<style>
  body { color: blue; }
</style>
</head>
<body>
  <p>inherited text</p>
  <h1>
    <a>nested inherited text</a>
  </h1>
</body>
</html>"#
            .to_string();
        let layout_view = create_layout_view(html);

        let root = layout_view.root();
        assert!(root.is_some());
        
        // bodyの色が青に設定されている
        let body_style = root.clone().expect("root should exist").borrow().style();
        assert_eq!(
            Color::from_name("blue").unwrap(),
            body_style.color()
        );
        
        // このテストは継承が動作することを確認する簡単なテスト
        // bodyの最初の要素（p）を取得して継承を確認
        let first_element = root.clone().expect("root should exist").borrow().first_child();
        assert!(first_element.is_some());
        let first_style = first_element.as_ref().unwrap().borrow().style();
        assert_eq!(
            Color::from_name("blue").unwrap(),
            first_style.color()
        );
    }

//     #[test]
//     fn test_default_font_sizes() {
//         let html = r#"<html>
// <head></head>
// <body><h1>Heading 1</h1></body>
// </html>"#
//             .to_string();
//         let layout_view = create_layout_view(html);

//         let root = layout_view.root();
//         assert!(root.is_some());
        
//         // bodyの最初の子要素（h1）を取得
//         let h1 = root.expect("root should exist").borrow().first_child();
//         assert!(h1.is_some());
        
//         let h1_ref = h1.expect("h1 should exist");
//         let h1_style = h1_ref.borrow().style();
        
//         // h1要素のフォントサイズはXXLargeであることを確認
//         assert_eq!(FontSize::XXLarge, h1_style.font_size());
//     }
}
