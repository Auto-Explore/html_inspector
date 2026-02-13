# html/semantics/document-metadata/the-link-element/stylesheet-non-OK-status.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/stylesheet-non-OK-status.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>stylesheet served with non-OK status</title>
<link rel="author" title="Michael[tm] Smith" href="mailto:mike@w3.org">
<link rel="help" href="https://html.spec.whatwg.org/#fetching-and-processing-a-resource-from-a-link-element:processresponseconsumebody">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  async_test((t) => {
    const link = document.createElement("link");
    link.rel = "stylesheet";
    link.href = "resources/302-no-Location-header-text-css.asis";
    link.onload = t.unreached_func();
    link.onerror = t.step_func_done();
    document.head.append(link);
  }, "'load' event does not fire at link@rel=stylesheet having resource with non-OK response status");
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
  "source_name": "html/semantics/document-metadata/the-link-element/stylesheet-non-OK-status.html"
}
```
