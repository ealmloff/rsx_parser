use dioxus::core::{Attribute, NodeFactory, ScopeState, VNode};

use crate::{
    ast::{Node, RsxCall},
    build_element::build_element,
    AttributeScope, ATTRIBUTES_MAP,
};

fn build<'a>(rsx: RsxCall<'a>, scope: &'a ScopeState) -> VNode<'a> {
    let factory = NodeFactory::new(scope);
    let children_built = factory.bump().alloc(Vec::new());
    for child in rsx.0 {
        children_built.push(build_node(child, &factory));
    }
    factory.fragment_from_iter(children_built.iter())
}

fn build_node<'a, 'b>(node: Node<'a>, factory: &'b NodeFactory<'a>) -> VNode<'a> {
    let bump = factory.bump();
    match node {
        Node::Element(element) => {
            let tag = element.tag;
            let attributes = bump.alloc(Vec::new());
            for attr in element.attributes {
                let name = attr.name;
                let value = bump.alloc(attr.value.to_string());

                attributes.push(Attribute {
                    name: name,
                    value: value.as_str(),
                    is_static: true,
                    is_volatile: false,
                    namespace: ATTRIBUTES_MAP.get_str(name).and_then(|entries| {
                        entries
                            .iter()
                            .find(|entry| match entry.scope {
                                AttributeScope::Global => true,
                                AttributeScope::Specific(scope_tag) => scope_tag == tag,
                            })
                            .and_then(|e| e.namespace)
                    }),
                })
            }
            let key = None;
            let children = bump.alloc(Vec::new());
            for child in element.children {
                children.push(build_node(child, factory));
            }
            build_element(
                factory,
                tag,
                &[],
                attributes.as_slice(),
                children.as_slice(),
                key,
            )
        }
        Node::Text(text) => {
            let text: String = text.0.iter().map(|v| v.to_string()).collect();
            factory.text(format_args!("{}", text))
        }
    }
}
