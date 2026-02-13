# html/dom/partial-updates/tentative/stream-html-with-trusted-types-no-policy.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/stream-html-with-trusted-types-no-policy.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="/trusted-types/support/helper.sub.js"></script>

    <meta
      http-equiv="Content-Security-Policy"
      content="require-trusted-types-for 'script';"
    />
  </head>
  <body>
    <div id="container"></div>
    <script>
      test((t) => {
        var container = document.querySelector("#container");
        let d = document.createElement("div");
        document.querySelector("#container").appendChild(d);
        assert_throws_js(TypeError, () => d.streamHTMLUnsafe());
      }, "Streaming with trusted types and no default policy throws.");
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meta.csp.invalid",
      "message": "Bad value “require-trusted-types-for 'script';” for attribute “content” on element “meta”.",
      "severity": "Warning",
      "span": {
        "byte_end": 323,
        "byte_start": 216,
        "col": 5,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/partial-updates/tentative/stream-html-with-trusted-types-no-policy.html"
}
```
