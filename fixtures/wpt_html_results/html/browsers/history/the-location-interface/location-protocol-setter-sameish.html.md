# html/browsers/history/the-location-interface/location-protocol-setter-sameish.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-protocol-setter-sameish.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Set location.protocol to the scheme it already was</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<iframe src=/common/blank.html></iframe>
<iframe src=/common/blank.html></iframe>
<iframe src=/common/blank.html></iframe>
<iframe src=/common/blank.html></iframe>
<script>
self.onload = () => {
  [
    "http",
    "ht\x0Atp",
    "http\x0A",
    "\x09ht\x09\x0AtP"
  ].forEach((val, index) => {
    async_test(t => {
      self[index].frameElement.onload = t.step_func_done(() => {
        assert_equals(self[index].location.protocol, "http:");
      });
      self[index].location.protocol = val;
    }, `Set location.protocol to ${encodeURI(val)} (percent-encoded here for clarity)`);
  });
}
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
  "source_name": "html/browsers/history/the-location-interface/location-protocol-setter-sameish.html"
}
```
