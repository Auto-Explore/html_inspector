# html/semantics/forms/the-input-element/input-whitespace.html

Counts:
- errors: 2
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-whitespace.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
  <head>
    <title>Chrome whitespace bug</title>
    <link rel="author" title="Steinar H. Gunderson" href="mailto:sesse@chromium.org">
    <link rel="help" href="https://crbug.com/1309014">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="/resources/testdriver.js"></script>
    <script src="/resources/testdriver-vendor.js"></script>
    <style>
      [data-foo] { color: red; }
      div input { color: inherit; }
    </style>
  </head>
  <body>
    <div id="container" data-foo="foo"><input id="input1"></input></div>
    <script>
      async_test(t => {
        let container = document.getElementById('container');
        let input = document.getElementById('input1');
        input.onkeypress = function(e) {
          container.removeAttribute('data-foo');
          input.style.display = 'block';
        };
        test_driver.send_keys(input, "a b")
          .then(t.step_func(() => {
             assert_equals(input.value, "a b");
             t.done();
          }));
      }, "whitespace should not be eaten after parent style recalculation");
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 634,
        "byte_start": 626,
        "col": 59,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 1211,
        "byte_start": 1204,
        "col": 1,
        "line": 34
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
  "source_name": "html/semantics/forms/the-input-element/input-whitespace.html"
}
```
