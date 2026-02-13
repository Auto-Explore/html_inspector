# html/browsers/history/the-location-interface/location-ancestor-origins-referrerpolicy-snapshot.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-ancestor-origins-referrerpolicy-snapshot.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>location.ancestorOrigins snapshot timing of referrerpolicy</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
promise_test(async () => {
  assert_implements(location.ancestorOrigins);
  const iframe = document.createElement('iframe');
  iframe.src = '/common/blank.html?pipe=trickle(d1)';
  iframe.referrerPolicy = 'no-referrer';
  const loaded = new Promise(resolve => iframe.onload = resolve);
  document.body.append(iframe);
  // initial about:blank should see 'no-referrer' results
  assert_array_equals(Array.from(iframe.contentWindow.location.ancestorOrigins), ['null']);
  iframe.referrerPolicy = '';
  await loaded;
  // The referrerpolicy attribute get snapshotted at step 16 of "beginning navigation"
  // https://html.spec.whatwg.org/#beginning-navigation and result should therefore,
  // still be ["null"], here.
  assert_array_equals(Array.from(iframe.contentWindow.location.ancestorOrigins), ["null"]);
});

promise_test(async () => {
  assert_implements(location.ancestorOrigins);
  const iframe = document.createElement('iframe');

  iframe.referrerPolicy = 'no-referrer';
  const loaded = new Promise(resolve => iframe.onload = resolve);
  document.body.append(iframe);
  // initial about:blank should see 'no-referrer' results
  assert_array_equals(Array.from(iframe.contentWindow.location.ancestorOrigins), ['null']);
  iframe.referrerPolicy = '';
  await loaded;

  await new Promise(resolve => {
    iframe.onload = resolve;
    iframe.src = '/common/blank.html?pipe=trickle(d1)';
  });
  assert_array_equals(Array.from(iframe.contentWindow.location.ancestorOrigins), [window.origin]);
});
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
  "source_name": "html/browsers/history/the-location-interface/location-ancestor-origins-referrerpolicy-snapshot.html"
}
```
