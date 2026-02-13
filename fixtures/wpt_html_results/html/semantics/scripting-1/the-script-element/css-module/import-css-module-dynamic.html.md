# html/semantics/scripting-1/the-script-element/css-module/import-css-module-dynamic.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/css-module/import-css-module-dynamic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>

<head>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>

<body>
    <script>
        promise_test(async function (test) {
            const css_module = await import("./resources/basic.css", { with: { type: "css" }});
            assert_true(css_module.default instanceof CSSStyleSheet);
            assert_equals(css_module.default.cssRules[0].cssText,
                "#test { background-color: rgb(255, 0, 0); }");
        }, "Load a CSS module with dynamic import()");

        promise_test(function (test) {
            return promise_rejects_js(test, TypeError,
                import("./resources/basic.css"),
                "Attempting to import() a CSS module without a type attribute should fail");
        }, "Ensure that loading a CSS module with dymnamic import() fails without a type attribute");
    </script>
</body>
```

```json
{
  "messages": [
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
  "source_name": "html/semantics/scripting-1/the-script-element/css-module/import-css-module-dynamic.html"
}
```
