# html/semantics/scripting-1/the-script-element/css-module/relative-urls.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/css-module/relative-urls.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<head>
  <title>Test resolution of relative URL in CSS module</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>
<body>
  <div id="target"></div>
  <script type="module">
    import styleSheet from "./resources/load-relative-url.css" with { type: "css"};
    test(() => {
      const target = document.querySelector("#target");
      document.adoptedStyleSheets = [ styleSheet ];
      let backgroundStyle = window.getComputedStyle(target).backgroundImage;
      assert_not_equals(backgroundStyle.indexOf("css-module/resources/image.png"), -1);
    }, "A relative URL in a CSS module should be resolved relative to the CSS file's URL, not the importing document's URL");
  </script>
</body>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/scripting-1/the-script-element/css-module/relative-urls.html"
}
```
