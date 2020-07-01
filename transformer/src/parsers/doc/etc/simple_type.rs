use crate::parsers::doc::etc::annotation::Annotation;
use crate::parsers::doc::etc::primitive_type::PrimitiveType;
use crate::parsers::doc::etc::primitive_type::RestrictedPrimitiveType;
use crate::parsers::doc::etc::r#type::TypeParseError;
use crate::parsers::doc::etc::XML_SCHEMA_NS;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub(super) struct SimpleType {
    pub(super) annotation: Option<Annotation>,
    pub(super) name: String,
    pub(super) pattern: Option<String>,
    pub(super) list: bool,
    pub(super) parent: PrimitiveType,
    pub(super) enumeration: Vec<String>,
    pub(super) min_inclusive: Option<String>,
}

impl TryFrom<&xmltree::XMLNode> for SimpleType {
    type Error = TypeParseError;

    fn try_from(value: &xmltree::XMLNode) -> Result<Self, Self::Error> {
        match value {
            xmltree::XMLNode::Element(xmltree::Element {
                namespace: Some(namespace),
                name,
                attributes,
                children,
                ..
            }) if namespace == XML_SCHEMA_NS && name == "simpleType" => {
                let name = attributes
                    .get("name")
                    .ok_or(TypeParseError::MissingName)?
                    .clone();
                let annotation = children
                    .iter()
                    .filter_map(|c| Annotation::try_from(c).ok())
                    .next();
                for child in children {
                    match child {
                        xmltree::XMLNode::Element(xmltree::Element {
                            namespace: Some(namespace),
                            name: node_name,
                            attributes,
                            ..
                        }) if namespace == XML_SCHEMA_NS && node_name == "list" => {
                            let parent = attributes
                                .get("itemType")
                                .ok_or(TypeParseError::MissingItemTypeValue)?;
                            return Ok(Self {
                                annotation,
                                name: name.clone(),
                                enumeration: Vec::new(),
                                list: true,
                                min_inclusive: None,
                                parent: parent.parse()?,
                                pattern: None,
                            });
                        }
                        xmltree::XMLNode::Element(xmltree::Element {
                            namespace: Some(namespace),
                            name: node_name,
                            attributes,
                            children,
                            ..
                        }) if namespace == XML_SCHEMA_NS && node_name == "restriction" => {
                            let parent =
                                attributes.get("base").ok_or(TypeParseError::MissingBase)?;
                            let pattern = children
                                .iter()
                                .filter_map(|child| match child {
                                    xmltree::XMLNode::Element(xmltree::Element {
                                        namespace: Some(namespace),
                                        name,
                                        attributes,
                                        ..
                                    }) if namespace == XML_SCHEMA_NS && name == "pattern" => {
                                        attributes.get("value").cloned()
                                    }
                                    _ => None,
                                })
                                .next();
                            let min_inclusive = children
                                .iter()
                                .filter_map(|child| match child {
                                    xmltree::XMLNode::Element(xmltree::Element {
                                        namespace: Some(namespace),
                                        name,
                                        attributes,
                                        ..
                                    }) if namespace == XML_SCHEMA_NS && name == "minInclusive" => {
                                        attributes.get("value").cloned()
                                    }
                                    _ => None,
                                })
                                .next();

                            let enumeration = children
                                .iter()
                                .filter_map(|child| match child {
                                    xmltree::XMLNode::Element(xmltree::Element {
                                        namespace: Some(namespace),
                                        name,
                                        attributes,
                                        ..
                                    }) if namespace == XML_SCHEMA_NS && name == "enumeration" => {
                                        attributes.get("value").cloned()
                                    }
                                    _ => None,
                                })
                                .collect();
                            return Ok(Self {
                                annotation,
                                name: name.clone(),
                                enumeration,
                                list: false,
                                min_inclusive,
                                parent: parent.parse()?,
                                pattern,
                            });
                        }
                        _ => {}
                    }
                }
                Err(TypeParseError::NotTypeNode)
            }
            _ => Err(TypeParseError::NotTypeNode),
        }
    }
}

impl From<&SimpleType> for openapiv3::Schema {
    fn from(t: &SimpleType) -> Self {
        let schema_data = openapiv3::SchemaData {
            deprecated: t.annotation.as_ref().map(|a| a.deprecated).unwrap_or(false),
            title: Some(t.name.clone()),
            description: t.annotation.as_ref().map(|a| a.description.clone()),
            ..Default::default()
        };

        let r#type = openapiv3::Type::from(&RestrictedPrimitiveType {
            r#type: t.parent,
            enumeration: &t.enumeration,
            min_inclusive: &t.min_inclusive,
            pattern: &t.pattern,
        });

        let schema_kind = openapiv3::SchemaKind::Type(r#type);
        if t.list {
            Self {
                schema_data,
                schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::Array(
                    openapiv3::ArrayType {
                        items: openapiv3::ReferenceOr::boxed_item(openapiv3::Schema {
                            schema_kind,
                            schema_data: Default::default(),
                        }),
                        max_items: None,
                        min_items: None,
                        unique_items: false,
                    },
                )),
            }
        } else {
            Self {
                schema_data,
                schema_kind,
            }
        }
    }
}
