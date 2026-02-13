# html/browsers/the-window-object/window-indexed-properties-delete-no-cache.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/window-indexed-properties-delete-no-cache.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<meta name=timeout content=long>
<title>Deletion of WindowProxy's indexed properties is not cached</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/window-object.html#windowproxy-delete">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
const iframe = document.createElement("iframe");
iframe.srcdoc = "";

test(() => {
  assert_equals(window.length, 0);
  for (let i = 0; i < 1e5; i++) {
    assert_true(delete window[0]);
  }

  document.body.append(iframe);
  assert_false(delete window[0]);
}, "Absence of index '0' is not cached");

test(() => {
  assert_equals(window.length, 1);
  for (let i = 0; i < 1e5; i++) {
    assert_false(delete window[0]);
  }

  iframe.remove();
  assert_true(delete window[0]);
}, "Presence of index '0' is not cached");
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
  "source_name": "html/browsers/the-window-object/window-indexed-properties-delete-no-cache.html"
}
```
