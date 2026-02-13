# html/browsers/history/the-location-interface/location-symbol-toprimitive.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-symbol-toprimitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Location Symbol.toPrimitive</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(() => {
  assert_equals(location[Symbol.toPrimitive], undefined)
  const prop = Object.getOwnPropertyDescriptor(location, Symbol.toPrimitive)
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
  "source_name": "html/browsers/history/the-location-interface/location-symbol-toprimitive.html"
}
```
