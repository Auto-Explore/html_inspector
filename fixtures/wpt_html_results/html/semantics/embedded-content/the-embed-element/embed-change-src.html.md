# html/semantics/embedded-content/the-embed-element/embed-change-src.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/embed-change-src.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<meta charset="utf-8">
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-embed-element">
<link rel="help" href="http://crbug.com/1035330">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
promise_test(async () => {
  const embed = document.createElement('embed');
  let loadPromise = new Promise(resolve => embed.onload = resolve);
  embed.src = '/media/white.mp4';
  document.body.appendChild(embed);

  await loadPromise;

  loadPromise = new Promise(resolve => embed.onload = resolve);
  embed.src = '/media/white.webm';

  await loadPromise;
}, 'Verifies that embed elements reload with new content when the src attribute is changed.');
</script>
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
  "source_name": "html/semantics/embedded-content/the-embed-element/embed-change-src.html"
}
```
