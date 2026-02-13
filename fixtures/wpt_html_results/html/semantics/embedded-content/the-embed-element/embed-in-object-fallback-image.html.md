# html/semantics/embedded-content/the-embed-element/embed-in-object-fallback-image.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/embed-in-object-fallback-image.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Ensure that embed elements inside object elements load when the objects
  fallback</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<link rel="author" title="Peng Zhou" href="mailto:zhoupeng.1996@bytedance.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-embed-element">
<link rel="match" href="embed-in-object-fallback-image-ref.html">
<meta name="assert" content="Check if the embed element represents an image when it has a object ancestor that is showing its fallback content">
<style>
  embed {
    background-color: red;
    height: 100px;
    width: 100px;
  }
</style>
<body>
  <p>Test passes if there is <strong> red</strong>.</p>
  <object type="application/x-shockwave-flash">
    <embed src="/images/red-16x16.png" type="image/png"></embed>
  </object>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 778,
        "byte_start": 733,
        "col": 3,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 843,
        "byte_start": 835,
        "col": 57,
        "line": 20
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
  "source_name": "html/semantics/embedded-content/the-embed-element/embed-in-object-fallback-image.html"
}
```
