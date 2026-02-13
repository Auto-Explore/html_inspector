# html/browsers/browsing-the-web/history-traversal/resources/api-availability-1.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/resources/api-availability-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>API availability following history traversal - 1</title>
<script>
var controller = opener;
var t = controller.t;
var assert_not_equals = controller.assert_not_equals;

t.step(function() {
  // If this document is discarded as a result of navigation, then this script
  // will be executed a second time. The semantics this test intends to verify
  // cannot be observed under these conditions, the discarding is not itself a
  // violation. Silently pass the test in that case.
  if (controller.hasNavigated) {
    t.done();
    return;
  }

  t.step_timeout(function() {
    assert_not_equals(window.history, null, 'history');
    assert_not_equals(window.localStorage, null, 'localStorage');
    assert_not_equals(window.location, null, 'location');
    assert_not_equals(window.navigator, null, 'navigator');
    assert_not_equals(window.opener, null, 'opener');
    assert_not_equals(window.sessionStorage, null, 'sessionStorage');

    t.done();
  }, 1000);

  controller.navigate();
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/resources/api-availability-1.html"
}
```
