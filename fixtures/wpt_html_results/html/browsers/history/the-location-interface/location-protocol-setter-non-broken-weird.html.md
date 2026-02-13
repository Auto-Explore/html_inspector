# html/browsers/history/the-location-interface/location-protocol-setter-non-broken-weird.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-protocol-setter-non-broken-weird.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Set location.protocol from an HTTP URL</title>
<!-- In particular, valid non-broken schemes that are nevertheless not going to work -->
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<iframe src=/common/blank.html></iframe>
<iframe src=/common/blank.html></iframe>
<iframe src=/common/blank.html></iframe>
<iframe src=/common/blank.html></iframe>
<iframe src=/common/blank.html></iframe>
<iframe src=/common/blank.html></iframe>
<script>
self.onload = () => {
  [
    'x',
    'data',
    'file',
    'ftp',
    'http+x'
  ].forEach((val, index) => {
    async_test((t) => {
      self[index].location.protocol = val
      t.step_timeout(() => {
        assert_equals(self[index].location.protocol, location.protocol)
        assert_equals(self[index].location.host, location.host)
        assert_equals(self[index].location.port, location.port)
        t.done()
        // Experimentally, 4 seconds is long enough for the navigation to
        // complete, if it happens.
      }, 4000)
    }, "Set location.protocol to " + val)
  })
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
  "source_name": "html/browsers/history/the-location-interface/location-protocol-setter-non-broken-weird.html"
}
```
