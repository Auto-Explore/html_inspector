# html/semantics/document-metadata/the-link-element/stylesheet-bad-mime-type.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/stylesheet-bad-mime-type.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>stylesheets served with bad MIME types</title>
<link rel="author" title="Michael[tm] Smith" href="mailto:mike@w3.org">
<link rel="help" href="https://html.spec.whatwg.org/#link-type-stylesheet:process-the-linked-resource">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  async_test((t) => {
    const link = document.createElement("link");
    link.rel = "stylesheet";
    link.href = "resources/stylesheet-bad-mime-type.css";
    link.onload = t.unreached_func();
    link.onerror = t.step_func_done();
    document.head.append(link);
  }, "'load' event does not fire at link@rel=stylesheet having non-empty resource with bad MIME type");
  async_test((t) => {
    t.step_timeout(() => {
      const link = document.createElement("link");
      link.rel = "stylesheet";
      link.href = "resources/stylesheet-bad-mime-type-empty.css";
      link.onload = t.unreached_func();
      link.onerror = t.step_func_done();
      document.head.append(link);
    }, 2000);
  }, "'load' event does not fire at link@rel=stylesheet having empty resource with bad MIME type");
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
  "source_name": "html/semantics/document-metadata/the-link-element/stylesheet-bad-mime-type.html"
}
```
