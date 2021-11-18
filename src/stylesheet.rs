use cssparser::{Parser, ParserInput, RuleListParser};
use crate::rules::CssRuleList;
use crate::parser::TopLevelRuleParser;
use crate::printer::Printer;
use crate::traits::ToCss;
use crate::properties::prefixes::Browsers;
use crate::declaration::DeclarationHandler;

pub struct StyleSheet {
  rules: CssRuleList
}

impl StyleSheet {
  pub fn parse<'i>(code: &str) -> StyleSheet{
    let mut input = ParserInput::new(&code);
    let mut parser = Parser::new(&mut input);
    let rule_list_parser = RuleListParser::new_for_stylesheet(&mut parser, TopLevelRuleParser {});

    let mut rules = vec![];
    for rule in rule_list_parser {
      let rule = if let Ok((_, rule)) = rule {
        rule
      } else {
        continue
      };

      rules.push(rule)
    }

    StyleSheet {
      rules: CssRuleList(rules)
    }
  }

  pub fn minify(&mut self, targets: Option<Browsers>) {
    let mut handler = DeclarationHandler::new(false, targets);
    let mut important_handler = DeclarationHandler::new(true, targets);
    self.rules.minify(targets, &mut handler, &mut important_handler);
  }

  pub fn to_css(&self, minify: bool) -> Result<String, std::fmt::Error> {
    let mut dest = String::new();
    let mut printer = Printer::new(&mut dest, minify);
    let mut first = true;

    for rule in &self.rules.0 {
      if first {
        first = false;
      } else {
        printer.newline()?;
      }
  
      rule.to_css(&mut printer)?;
      printer.newline()?;
    }

    Ok(dest)
  }
}
