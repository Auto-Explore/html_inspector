# html/semantics/embedded-content/the-embed-element/embed-change-src-2.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/embed-change-src-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>The embed element update with new content when the src attribute is set</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-embed-element">
<link rel="help" href="https://crbug.com/479233864">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
promise_test(async () => {
  const embed = document.body.appendChild(document.createElement('embed'));

  const boundsBefore = embed.getBoundingClientRect();
  assert_equals(boundsBefore.width, 0, 'represents nothing');

  const loadPromise = new Promise(resolve => embed.onload = resolve);
  embed.src = 'data:image/svg+xml,<svg height="100" width="100" xmlns="http://www.w3.org/2000/svg"><rect height="100" width="100" fill="green"/></svg>';

  await loadPromise;

  const boundsAfter = embed.getBoundingClientRect();
  assert_equals(boundsAfter.width, 100, 'represents its content navigable');
});
</script>
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
  "source_name": "html/semantics/embedded-content/the-embed-element/embed-change-src-2.html"
}
```
