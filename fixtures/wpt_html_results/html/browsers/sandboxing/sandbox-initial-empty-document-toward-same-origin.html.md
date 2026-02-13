# html/browsers/sandboxing/sandbox-initial-empty-document-toward-same-origin.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/sandbox-initial-empty-document-toward-same-origin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>
  Check sandbox-flags inheritance in case of javascript window reuse.
</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
promise_test(async test => {
  let message = new Promise(resolve =>
    window.addEventListener("message", event => resolve(event.data))
  );

  // Create an initial empty document in the iframe, sandboxed. It will attempt
  // to load a slow page, but won't have time.
  let iframe = document.createElement("iframe");
  iframe.setAttribute("sandbox", "allow-scripts allow-same-origin");
  iframe.src = "/fetch/api/resources/infinite-slow-response.py";
  document.body.appendChild(iframe);

  // Remove sandbox flags. This should apply to documents committed from
  // navigations started after this instruction.
  iframe.removeAttribute("sandbox");
  iframe.src = "./resources/check-sandbox-flags.html";

  // The window is reused, but the new sandbox flags should be used.
  assert_equals(await message, "document-domain-is-allowed");
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
  "source_name": "html/browsers/sandboxing/sandbox-initial-empty-document-toward-same-origin.html"
}
```
