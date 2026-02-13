# html/semantics/embedded-content/the-embed-element/embed-represent-nothing-03.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/embed-represent-nothing-03.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: The embed element represents nothing when it has a media ancestor</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-embed-element">
<link rel="match" href="embed-represent-nothing-ref.html">
<meta name="assert" content="Check if the embed element represents nothing when it has a media ancestor">
<style>
  embed {
    background-color: red;
    height: 100px;
    width: 100px;
  }
</style>
<body>
  <p>Test passes if there is <strong>no red</strong>.</p>
  <video>
    <embed src="/images/red-16x16.png">
  </video>
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
  "source_name": "html/semantics/embedded-content/the-embed-element/embed-represent-nothing-03.html"
}
```
