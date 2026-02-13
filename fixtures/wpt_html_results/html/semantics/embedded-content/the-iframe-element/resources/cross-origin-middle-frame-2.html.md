# html/semantics/embedded-content/the-iframe-element/resources/cross-origin-middle-frame-2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/resources/cross-origin-middle-frame-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!-- This document is used for the test `move-node-local-root-events-still-fire.html` -->
<script src="/common/get-host-info.sub.js"></script>

<body>
<script>
(async function() {
  // This iframe will be cross-origin to the current document, but same-origin
  // with its grandparent, our direct parent.
  const iframe = document.createElement('iframe');
  // A URL that's cross-origin to this document, and same-origin to our parent.
  const crossOriginChildURL = new URL('create-button-with-pointerdown-listener.html', location.href).toString().replace(location.origin, get_host_info().HTTP_ORIGIN);
  iframe.src = crossOriginChildURL;
  const loadPromise = new Promise(resolve => iframe.onload = resolve);
  document.body.append(iframe);
  await loadPromise;

  parent.postMessage('grandchild loaded', '*');
})();
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/resources/cross-origin-middle-frame-2.html"
}
```
