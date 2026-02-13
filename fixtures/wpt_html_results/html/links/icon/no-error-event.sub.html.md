# html/links/icon/no-error-event.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/links/icon/no-error-event.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>&lt;link rel="icon"&gt; doesn't fire an error event</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  async_test((t) => {
    const link = document.createElement("link");
    link.rel = "icon";
    link.href = "http://{{hosts[][nonexistent]}}";

    link.addEventListener("error", t.unreached_func());
    document.head.append(link);
    t.step_timeout(() => t.done(), 2000);
  }, document.title);
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
  "source_name": "html/links/icon/no-error-event.sub.html"
}
```
