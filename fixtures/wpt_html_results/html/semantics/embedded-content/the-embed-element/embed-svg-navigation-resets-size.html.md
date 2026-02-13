# html/semantics/embedded-content/the-embed-element/embed-svg-navigation-resets-size.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/embed-svg-navigation-resets-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Intrinsic sizing of SVG embedded via &lt;embed&gt; should disappear on navigation</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<embed id="element" name="embed" src='data:image/svg+xml,
  <svg xmlns="http://www.w3.org/2000/svg"
       xmlns:xlink="http://www.w3.org/1999/xlink"
       viewBox="0 0 1000 1000"
       width="400"
       height="400"
       font-size="100">
  </svg>'></embed>
<script>
promise_test(async () => {
  await new Promise(resolve => {
    onload = resolve;
  });

  assert_equals(element.scrollWidth, 400, "The width should be determined by the embedded SVG");
  assert_equals(element.scrollHeight, 400, "The height should be determined by the embedded SVG");

  await new Promise(resolve => {
    element.onload = resolve;
    element.src = "about:blank";
  });

  assert_equals(element.scrollWidth, 300, "The width should be the default");
  assert_equals(element.scrollHeight, 150, "The height should be the default");
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.embed.src.invalid",
      "message": "Bad value “data:image/svg+xml,\n  <svg xmlns=\"http://www.w3.org/2000/svg\"\n       xmlns:xlink=\"http://www.w3.org/1999/xlink\"\n       viewBox=\"0 0 1000 1000\"\n       width=\"400\"\n       height=\"400\"\n       font-size=\"100\">\n  </svg>” for attribute “src” on element “embed”.",
      "severity": "Warning",
      "span": {
        "byte_end": 490,
        "byte_start": 236,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 498,
        "byte_start": 490,
        "col": 11,
        "line": 13
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
  "source_name": "html/semantics/embedded-content/the-embed-element/embed-svg-navigation-resets-size.html"
}
```
