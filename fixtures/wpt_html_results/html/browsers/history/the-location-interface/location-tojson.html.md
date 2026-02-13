# html/browsers/history/the-location-interface/location-tojson.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-tojson.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Location has no toJSON</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(() => {
  assert_equals(location.toJSON, undefined)
  assert_equals(Object.getOwnPropertyDescriptor(location, "toJSON"), undefined)
  assert_false(location.hasOwnProperty("toJSON"))
})
</script>
<!-- See https://github.com/whatwg/html/pull/2294 for context. (And the HTML Standard of course.) -->
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
  "source_name": "html/browsers/history/the-location-interface/location-tojson.html"
}
```
