# html/browsers/history/the-location-interface/location_hash_set_empty_string.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location_hash_set_empty_string.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Set window.location.hash to an empty string</title>
<link rel="author" href="mailto:cristianb@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-location-hash">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
const orig_location = window.location.href;

window.location.hash = '';

test(() => {
    assert_true(window.location.hash === '');
    assert_true(window.location.href === orig_location);
}, 'window.location.hash is an empty string');
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
  "source_name": "html/browsers/history/the-location-interface/location_hash_set_empty_string.html"
}
```
