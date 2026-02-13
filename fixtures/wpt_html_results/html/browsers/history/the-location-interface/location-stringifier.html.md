# html/browsers/history/the-location-interface/location-stringifier.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-stringifier.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Location stringifier</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://webidl.spec.whatwg.org/#es-stringifier">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/common/stringifiers.js></script>
<div id=log></div>
<script>
test_stringifier_attribute(location, "href", true);

test(function() {
  const prop1 = Object.getOwnPropertyDescriptor(location, "toString"),
        prop2 = Object.getOwnPropertyDescriptor(location, "href")

  assert_true(prop1.enumerable)
  assert_false(prop1.writable)
  assert_false(prop1.configurable)

  assert_true(prop2.enumerable)
  assert_false(prop2.configurable)
  assert_equals(typeof prop2.get, "function")
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
  "source_name": "html/browsers/history/the-location-interface/location-stringifier.html"
}
```
