# html/browsers/browsing-the-web/navigating-across-documents/empty_fragment.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/empty_fragment.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Navigating to the same URL with an empty fragment aborts the navigation</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<iframe src="empty_fragment_iframe.html"></iframe>
<script>
// If the navigation were not aborted, we would expect multiple load events
// as the page continually reloads itself.
async_test(function(t) {
  var count = 0;
  var iframe = document.querySelector('iframe');
  iframe.onload = t.step_func(function() {
    count++;
  });
  window.child_succeeded = t.step_func_done(function() {
    assert_equals(count, 1);
  });
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/empty_fragment.html"
}
```
