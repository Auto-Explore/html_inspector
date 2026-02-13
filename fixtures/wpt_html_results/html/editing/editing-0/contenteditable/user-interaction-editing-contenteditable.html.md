# html/editing/editing-0/contenteditable/user-interaction-editing-contenteditable.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/editing-0/contenteditable/user-interaction-editing-contenteditable.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Editing: contentEditable attribute test</title>
    <link rel="author" title="Baidu" href="mailto: guopengcheng@baidu.com" />
    <link
      rel="help"
      href="https://html.spec.whatwg.org/multipage/#contenteditable"
    />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <div id="log"></div>
  </head>
  <body>
    <script>
      function testContentEditable(variationFunc, title, expectIsContentEditable, expectContentEditable) {
        test(() => {
          const div = document.createElement("div");
          variationFunc(div);
          assert_equals(div.isContentEditable, expectIsContentEditable, 'isContentEditable');
          assert_equals(div.contentEditable, expectContentEditable, 'contentEditable');
        }, title);
      }

      testContentEditable(el => {
      }, "no contenteditable attribute", false, "inherit");

      testContentEditable(el => {
        el.setAttribute("contenteditable", "");
      }, "empty contentEditable attribute", true, "true");

      testContentEditable(el => {
        el.contentEditable = "true";
      }, 'set contentEditable = "true"', true, "true");

      testContentEditable(el => {
        el.contentEditable = "false";
      }, 'set contentEditable = "false"', false, "false");

      testContentEditable(el => {
        const parent = document.createElement("div");
        parent.appendChild(el);
        parent.contentEditable = "true";
      }, 'set parent element contentEditable = "true"', true, "inherit");

      testContentEditable(el => {
        const parent = document.createElement("div");
        parent.appendChild(el);
        parent.contentEditable = "false";
      }, 'set parent element contentEditable = "false"', false, "inherit");

      testContentEditable(el => {
        el.contentEditable = "true";
        el.removeAttribute("contenteditable");
      }, 'set contentEditable = "true" and then remove contenteditable attribute', false, "inherit");

      testContentEditable(el => {
        el.setAttribute("contenteditable", "plaintext-only");
      }, "contentEditable=plaintext-only attribute", true, "plaintext-only");

      testContentEditable(el => {
        const parent = document.createElement("div");
        parent.appendChild(el);
        parent.contentEditable = "plaintext-only";
      }, 'set parent element contentEditable = "plaintext-only"', true, "inherit");
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.invalid",
      "message": "Bad value “mailto: guopengcheng@baidu.com” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 168,
        "byte_start": 95,
        "col": 5,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “head”.",
      "severity": "Error",
      "span": {
        "byte_end": 420,
        "byte_start": 413,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.parser.body.already_open",
      "message": "Start tag “body” seen but an element of the same type was already open.",
      "severity": "Error",
      "span": {
        "byte_end": 429,
        "byte_start": 423,
        "col": 3,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.cannot_recover",
      "message": "Cannot recover after last error. Any further errors will be ignored.",
      "severity": "Error",
      "span": {
        "byte_end": 429,
        "byte_start": 423,
        "col": 3,
        "line": 14
      }
    }
  ],
  "source_name": "html/editing/editing-0/contenteditable/user-interaction-editing-contenteditable.html"
}
```
