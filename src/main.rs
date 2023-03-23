use regex::Regex;
use std::io::{self, BufRead};

fn convert(lines: &str) -> String {
    let re = Regex::new(r"(?P<src>https:.+.(png|jpg|jpeg|mov))").unwrap();

    let tds = lines
        .split('\n')
        .map(|line| match re.captures(line) {
            Some(caps) => {
                format!(
                    r#"  <td>
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
{tds}
</table>"#,
        tds = &tds
    )
}

fn main() {
    let stdin = io::stdin();
    // for line in stdin.lock().lines() {
    //     println!("{}", line.unwrap());
    // }
    let lines = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", convert(&lines));

    // let stdin = new TextDecoder().decode(await Deno.readAll(Deno.stdin))
    // console.log(convert(stdin.trim()))
}

#[test]
fn test_main() {
    let expected = r#"<table>
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
</table>"#;
    let fixture = r#"
    ![image.png](https://image.test/uploads/000000000000000000000000000000000000.png =WxH)
    ![image.png](https://image.test/uploads/000000000000000000000000000000000001.png =WxH)
    ![image.png](https://image.test/uploads/000000000000000000000000000000000002.png =WxH)
    ![image.png](https://image.test/uploads/000000000000000000000000000000000003.mov =WxH)
"#;
    assert_eq!(convert(&fixture), expected);
}
