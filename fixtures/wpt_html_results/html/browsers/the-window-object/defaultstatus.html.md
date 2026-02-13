# html/browsers/the-window-object/defaultstatus.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/defaultstatus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>window.defaultStatus and window.defaultstatus are not supported</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://developer.mozilla.org/en-US/docs/Web/API/Window/defaultStatus">
<link rel="help" href="https://crbug.com/692835">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
test(() => {
  assert_false(window.hasOwnProperty('defaultStatus'));
  assert_false(window.hasOwnProperty('defaultstatus'));
  assert_equals(window.defaultStatus,undefined);
  assert_equals(window.defaultstatus,undefined);
}, "The window.defaultStatus and window.defaultstatus attributes are not supported");
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
  "source_name": "html/browsers/the-window-object/defaultstatus.html"
}
```
