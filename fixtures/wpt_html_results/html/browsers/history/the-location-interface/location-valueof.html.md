# html/browsers/history/the-location-interface/location-valueof.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-valueof.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Location valueOf</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(() => {
  assert_equals(location.valueOf, Object.prototype.valueOf)
  assert_equals(typeof location.valueOf.call(5), "object")
  const prop = Object.getOwnPropertyDescriptor(location, "valueOf")
  assert_false(prop.enumerable)
  assert_false(prop.writable)
  assert_false(prop.configurable)
})
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
  "source_name": "html/browsers/history/the-location-interface/location-valueof.html"
}
```
