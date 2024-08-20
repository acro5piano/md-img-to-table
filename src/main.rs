use regex::Regex;
use std::io::{self, BufRead};

fn convert(lines: &str) -> String {
    let re = Regex::new(r"!\[.+\]\((?P<src>https://.+?)[ |\)]").expect("Unable to init regex");

    let tds = lines
        .trim()
        .split('\n')
        .map(|line| match re.captures(line) {
            Some(caps) => {
                format!(
                    r#"    <td>
      <img src="{src}" />
    </td>"#,
                    src = &caps["src"]
                )
            }
            _ => String::from(""),
        })
        .filter(|line| line.chars().count() > 0)
        .collect::<Vec<String>>()
        .join("\n");
    format!(
        r#"<table>
  <tr>
{tds}
  </tr>
</table>"#,
        tds = &tds
    )
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", convert(&lines));
}

#[test]
fn test_main() {
    let expected = r#"<table>
  <tr>
    <td>
      <img src="https://image.test/uploads/000000000000000000000000000000000000.png" />
    </td>
    <td>
      <img src="https://image.test/uploads/000000000000000000000000000000000001.png" />
    </td>
    <td>
      <img src="https://image.test/uploads/000000000000000000000000000000000002.png" />
    </td>
    <td>
      <img src="https://image.test/uploads/000000000000000000000000000000000003.mov" />
    </td>
    <td>
      <img src="https://image.test/uploads/000000000000000000000000000000000004.MOV" />
    </td>
    <td>
      <img src="https://image.test/uploads/no-extension-github-new-type" />
    </td>
  </tr>
</table>"#;
    let fixture = r#"
    ![image.png](https://image.test/uploads/000000000000000000000000000000000000.png =WxH)
    ![image.png](https://image.test/uploads/000000000000000000000000000000000001.png =WxH)
    ![image.png](https://image.test/uploads/000000000000000000000000000000000002.png =WxH)
    ![image.png](https://image.test/uploads/000000000000000000000000000000000003.mov =WxH)
    ![image.png](https://image.test/uploads/000000000000000000000000000000000004.MOV =WxH)
    ![image.png](https://image.test/uploads/no-extension-github-new-type)
"#;
    let converted = convert(&fixture);
    println!("{}", &converted); // For debug
    assert_eq!(converted, expected);
}
