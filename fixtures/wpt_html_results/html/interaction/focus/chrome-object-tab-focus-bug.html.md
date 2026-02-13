# html/interaction/focus/chrome-object-tab-focus-bug.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/chrome-object-tab-focus-bug.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Tabbing through object tag</title>
<link rel="author" title="atotic@chromium.org">
<link rel="help" href="https://crbug.com/1132895">
<meta assert="assert" content="Tabbed focus works through OBJECT tags">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<p>Pressing TAB twice should focus/highlight end checkbox</p>

<input type="checkbox" id="start">start</a>
<object type='text/html' data='data:text/html,' width='16' height='16'></object>
<input type="checkbox" id="end" >end</a>

<script>

let t = async_test("focus advances with tab key thorough object element");

let start = document.querySelector("#start");
let object = document.querySelector("object");
let end = document.querySelector("#end");
let tab = "\uE004";

t.step( _ => {
  document.querySelector("#start").focus();
  assert_equals(document.activeElement, start, "start got focus");
  test_driver.send_keys(document.activeElement, tab).then( _ => {
    t.step( _ => {
      assert_equals(document.activeElement, object, "object got focus");
      test_driver.send_keys(document.activeElement, tab).then( _ => {
        t.step( _ => {
          assert_equals(document.activeElement, end, "end got focus");
          t.done();
        });
      });
    });
  });
});

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 128,
        "byte_start": 81,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “a”.",
      "severity": "Error",
      "span": {
        "byte_end": 570,
        "byte_start": 566,
        "col": 40,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “a”.",
      "severity": "Error",
      "span": {
        "byte_end": 692,
        "byte_start": 688,
        "col": 37,
        "line": 16
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/interaction/focus/chrome-object-tab-focus-bug.html"
}
```
