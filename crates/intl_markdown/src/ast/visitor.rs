use crate::{
    BlockNode, CodeBlock, CodeSpan, Document, Emphasis, Heading, Hook, Icu, IcuDate,
    IcuDateTimeStyle, IcuNumber, IcuNumberStyle, IcuPlural, IcuPluralArm, IcuSelect, IcuTime,
    IcuVariable, InlineContent, Link, Paragraph, Strikethrough, Strong, TextOrPlaceholder,
};

pub trait Visitor {
    fn visit_block_node(&mut self, _node: &BlockNode) {}
    fn exit_block_node(&mut self, _node: &BlockNode) {}
    fn visit_code_block(&mut self, _node: &CodeBlock) {}
    fn exit_code_block(&mut self, _node: &CodeBlock) {}
    fn visit_code_span(&mut self, _node: &CodeSpan) {}
    fn exit_code_span(&mut self, _node: &CodeSpan) {}
    fn visit_document(&mut self, _node: &Document) {}
    fn exit_document(&mut self, _node: &Document) {}
    fn visit_emphasis(&mut self, _node: &Emphasis) {}
    fn exit_emphasis(&mut self, _node: &Emphasis) {}
    fn visit_heading(&mut self, _node: &Heading) {}
    fn exit_heading(&mut self, _node: &Heading) {}
    fn visit_hook(&mut self, _node: &Hook) {}
    fn exit_hook(&mut self, _node: &Hook) {}
    fn visit_icu(&mut self, _node: &Icu) {}
    fn exit_icu(&mut self, _node: &Icu) {}
    fn visit_icu_date(&mut self, _node: &IcuDate) {}
    fn exit_icu_date(&mut self, _node: &IcuDate) {}
    fn visit_icu_date_time_style(&mut self, _node: &IcuDateTimeStyle) {}
    fn exit_icu_date_time_style(&mut self, _node: &IcuDateTimeStyle) {}
    fn visit_icu_number(&mut self, _node: &IcuNumber) {}
    fn exit_icu_number(&mut self, _node: &IcuNumber) {}
    fn visit_icu_number_style(&mut self, _node: &IcuNumberStyle) {}
    fn exit_icu_number_style(&mut self, _node: &IcuNumberStyle) {}
    fn visit_icu_plural(&mut self, _node: &IcuPlural) {}
    fn exit_icu_plural(&mut self, _node: &IcuPlural) {}
    fn visit_icu_plural_arm(&mut self, _node: &IcuPluralArm) {}
    fn exit_icu_plural_arm(&mut self, _node: &IcuPluralArm) {}
    fn visit_icu_select(&mut self, _node: &IcuSelect) {}
    fn exit_icu_select(&mut self, _node: &IcuSelect) {}
    fn visit_icu_time(&mut self, _node: &IcuTime) {}
    fn exit_icu_time(&mut self, _node: &IcuTime) {}
    fn visit_icu_variable(&mut self, _node: &IcuVariable) {}
    fn exit_icu_variable(&mut self, _node: &IcuVariable) {}
    fn visit_inline_content(&mut self, _node: &InlineContent) {}
    fn exit_inline_content(&mut self, _node: &InlineContent) {}
    fn visit_link(&mut self, _node: &Link) {}
    fn exit_link(&mut self, _node: &Link) {}
    fn visit_link_destination(&mut self, _node: &TextOrPlaceholder) {}
    fn exit_link_destination(&mut self, _node: &TextOrPlaceholder) {}
    fn visit_paragraph(&mut self, _node: &Paragraph) {}
    fn exit_paragraph(&mut self, _node: &Paragraph) {}
    fn visit_strikethrough(&mut self, _node: &Strikethrough) {}
    fn exit_strikethrough(&mut self, _node: &Strikethrough) {}
    fn visit_strong(&mut self, _node: &Strong) {}
    fn exit_strong(&mut self, _node: &Strong) {}
    fn visit_text_or_placeholder(&mut self, _node: &TextOrPlaceholder) {}
    fn exit_text_or_placeholder(&mut self, _node: &TextOrPlaceholder) {}
    fn visit_thematic_break(&mut self) {}
    fn exit_thematic_break(&mut self) {}
    fn visit_hard_line_break(&mut self) {}
    fn exit_hard_line_break(&mut self) {}
    fn visit_icu_pound(&mut self) {}
    fn exit_icu_pound(&mut self) {}
    fn visit_text(&mut self, _node: &String) {}
    fn exit_text(&mut self, _node: &String) {}
}

pub fn visit_with_mut<V: Visitor>(visitor: &mut V, document: &Document) {
    Traversal::traverse_document(visitor, document);
}

pub struct Traversal;

impl Traversal {
    #[inline(always)]
    pub fn traverse_children<V: Visitor, N, F>(
        visitor: &mut V,
        nodes: &Vec<N>,
        mut traverse_func: F,
    ) where
        V: Visitor,
        F: FnMut(&mut V, &N) -> (),
    {
        for child in nodes {
            traverse_func(visitor, child);
        }
    }

    #[inline(always)]
    pub fn traverse_inline_children<V: Visitor>(visitor: &mut V, children: &Vec<InlineContent>) {
        for child in children {
            Self::traverse_inline_content(visitor, child);
        }
    }

    pub fn traverse_document<V: Visitor>(visitor: &mut V, node: &Document) {
        visitor.visit_document(node);
        Self::traverse_children(visitor, node.blocks(), Self::traverse_block_node);
        visitor.exit_document(node);
    }

    pub fn traverse_block_node<V: Visitor>(visitor: &mut V, node: &BlockNode) {
        visitor.visit_block_node(node);
        match node {
            BlockNode::Paragraph(paragraph) => Self::traverse_paragraph(visitor, paragraph),
            BlockNode::Heading(heading) => Self::traverse_heading(visitor, heading),
            BlockNode::CodeBlock(code_block) => Self::traverse_code_block(visitor, code_block),
            BlockNode::ThematicBreak => visitor.visit_thematic_break(),
            BlockNode::InlineContent(inline_content) => {
                Self::traverse_inline_children(visitor, inline_content)
            }
        }
        visitor.exit_block_node(node);
    }

    pub fn traverse_paragraph<V: Visitor>(visitor: &mut V, node: &Paragraph) {
        visitor.visit_paragraph(node);
        Self::traverse_inline_children(visitor, node.content());
        visitor.exit_paragraph(node);
    }

    pub fn traverse_inline_content<V: Visitor>(visitor: &mut V, content: &InlineContent) {
        visitor.visit_inline_content(content);
        match content {
            InlineContent::Text(text) => Self::traverse_text(visitor, text),
            InlineContent::Emphasis(emphasis) => Self::traverse_emphasis(visitor, emphasis),
            InlineContent::Strong(strong) => Self::traverse_strong(visitor, strong),
            InlineContent::Link(link) => Self::traverse_link(visitor, link),
            InlineContent::CodeSpan(code_span) => Self::traverse_code_span(visitor, code_span),
            InlineContent::Hook(hook) => Self::traverse_hook(visitor, hook),
            InlineContent::Strikethrough(strikethrough) => {
                Self::traverse_strikethrough(visitor, strikethrough)
            }
            InlineContent::Icu(icu) => Self::traverse_icu(visitor, icu),
            InlineContent::IcuPound => visitor.visit_icu_pound(),
            InlineContent::HardLineBreak => visitor.visit_hard_line_break(),
        }
        visitor.exit_inline_content(content);
    }

    pub fn traverse_heading<V: Visitor>(visitor: &mut V, heading: &Heading) {
        visitor.visit_heading(heading);
        Self::traverse_inline_children(visitor, heading.content());
        visitor.exit_heading(heading);
    }

    pub fn traverse_code_block<V: Visitor>(visitor: &mut V, code_block: &CodeBlock) {
        visitor.visit_code_block(code_block);
        visitor.exit_code_block(code_block);
    }

    pub fn traverse_text<V: Visitor>(visitor: &mut V, text: &String) {
        visitor.visit_text(text);
        visitor.exit_text(text);
    }

    pub fn traverse_emphasis<V: Visitor>(visitor: &mut V, emphasis: &Emphasis) {
        visitor.visit_emphasis(emphasis);
        Self::traverse_inline_children(visitor, emphasis.content());
        visitor.exit_emphasis(emphasis);
    }

    pub fn traverse_strong<V: Visitor>(visitor: &mut V, strong: &Strong) {
        visitor.visit_strong(strong);
        Self::traverse_inline_children(visitor, strong.content());
        visitor.exit_strong(strong);
    }

    pub fn traverse_strikethrough<V: Visitor>(visitor: &mut V, strikethrough: &Strikethrough) {
        visitor.visit_strikethrough(strikethrough);
        Self::traverse_inline_children(visitor, strikethrough.content());
        visitor.exit_strikethrough(strikethrough);
    }

    pub fn traverse_link<V: Visitor>(visitor: &mut V, link: &Link) {
        visitor.visit_link(link);
        Self::traverse_inline_children(visitor, link.label());
        Self::traverse_link_destination(visitor, link.destination());
        visitor.exit_link(link);
    }

    pub fn traverse_link_destination<V: Visitor>(visitor: &mut V, handler: &TextOrPlaceholder) {
        visitor.visit_link_destination(handler);
        // Only traversing placeholders separately, since Text and Handler are just String values
        // that are _not_ visible content in this context.
        match handler {
            TextOrPlaceholder::Placeholder(placeholder) => Self::traverse_icu(visitor, placeholder),
            _ => {}
        }
        visitor.exit_link_destination(handler);
    }

    pub fn traverse_hook<V: Visitor>(visitor: &mut V, hook: &Hook) {
        visitor.visit_hook(hook);
        Self::traverse_inline_children(visitor, hook.content());
        visitor.exit_hook(hook);
    }

    pub fn traverse_code_span<V: Visitor>(visitor: &mut V, code_span: &CodeSpan) {
        visitor.visit_code_span(code_span);
        visitor.exit_code_span(code_span);
    }

    pub fn traverse_icu<V: Visitor>(visitor: &mut V, icu: &Icu) {
        visitor.visit_icu(icu);
        match icu {
            Icu::IcuVariable(variable) => Self::traverse_icu_variable(visitor, variable),
            Icu::IcuPlural(plural) => Self::traverse_icu_plural(visitor, plural),
            Icu::IcuSelect(select) => Self::traverse_icu_select(visitor, select),
            Icu::IcuDate(date) => Self::traverse_icu_date(visitor, date),
            Icu::IcuTime(time) => Self::traverse_icu_time(visitor, time),
            Icu::IcuNumber(number) => Self::traverse_icu_number(visitor, number),
        }
        visitor.exit_icu(icu);
    }

    pub fn traverse_icu_variable<V: Visitor>(visitor: &mut V, variable: &IcuVariable) {
        visitor.visit_icu_variable(variable);
        visitor.exit_icu_variable(variable);
    }

    pub fn traverse_icu_plural<V: Visitor>(visitor: &mut V, plural: &IcuPlural) {
        visitor.visit_icu_plural(plural);
        Self::traverse_icu_variable(visitor, plural.variable());
        Self::traverse_children(visitor, plural.arms(), Self::traverse_icu_plural_arm);
        visitor.exit_icu_plural(plural);
    }

    pub fn traverse_icu_plural_arm<V: Visitor>(visitor: &mut V, arm: &IcuPluralArm) {
        visitor.visit_icu_plural_arm(arm);
        Self::traverse_inline_children(visitor, arm.content());
        visitor.exit_icu_plural_arm(arm);
    }

    pub fn traverse_icu_select<V: Visitor>(visitor: &mut V, select: &IcuSelect) {
        visitor.visit_icu_select(select);
        Self::traverse_icu_variable(visitor, select.variable());
        Self::traverse_children(visitor, select.arms(), Self::traverse_icu_plural_arm);
        visitor.exit_icu_select(select);
    }

    pub fn traverse_icu_date<V: Visitor>(visitor: &mut V, date: &IcuDate) {
        visitor.visit_icu_date(date);
        Self::traverse_icu_variable(visitor, date.variable());
        if let Some(style) = date.style.as_ref() {
            Self::traverse_icu_date_time_style(visitor, style);
        }
        visitor.exit_icu_date(date);
    }

    pub fn traverse_icu_date_time_style<V: Visitor>(visitor: &mut V, style: &IcuDateTimeStyle) {
        visitor.visit_icu_date_time_style(style);
        visitor.exit_icu_date_time_style(style);
    }

    pub fn traverse_icu_time<V: Visitor>(visitor: &mut V, time: &IcuTime) {
        visitor.visit_icu_time(time);
        Self::traverse_icu_variable(visitor, time.variable());
        if let Some(style) = time.style.as_ref() {
            Self::traverse_icu_date_time_style(visitor, style);
        }
        visitor.exit_icu_time(time);
    }

    pub fn traverse_icu_number<V: Visitor>(visitor: &mut V, number: &IcuNumber) {
        visitor.visit_icu_number(number);
        if let Some(style) = number.style.as_ref() {
            Self::traverse_icu_number_style(visitor, style);
        }
        visitor.exit_icu_number(number);
    }

    pub fn traverse_icu_number_style<V: Visitor>(visitor: &mut V, style: &IcuNumberStyle) {
        visitor.visit_icu_number_style(style);
        visitor.exit_icu_number_style(style);
    }
}